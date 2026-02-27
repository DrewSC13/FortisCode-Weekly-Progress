use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode, decode_header};
use redis::AsyncCommands;
use serde::Deserialize;
use std::{collections::HashSet, fs, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;

type RedisConn = redis::aio::MultiplexedConnection;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, allow(dead_code))] 
pub struct Claims {
    pub exp: usize,
    pub iss: String,
    pub sub: String,
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("missing kid in jwt header")]
    MissingKid,

    #[error("public key not found for kid={0}")]
    PublicKeyNotFound(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

/// KeyStore:
/// - Cache local en memoria (kid -> pem bytes)
/// - Redis opcional para rotación dinámica de claves públicas
pub struct KeyStore {
    local_cache: Arc<RwLock<std::collections::HashMap<String, Vec<u8>>>>,
    redis: Option<Arc<RwLock<RedisConn>>>,
}

impl KeyStore {
    pub async fn new(
        _issuer_unused: impl Into<String>,
        fallback_public_pem_path: impl Into<String>,
        redis_url: Option<String>,
    ) -> Result<Self, VerifyError> {
        let fallback_public_pem_path = fallback_public_pem_path.into();

        let redis = if let Some(url) = redis_url {
            let client = redis::Client::open(url)?;
            let conn: RedisConn = client.get_multiplexed_async_connection().await?;
            Some(Arc::new(RwLock::new(conn)))
        } else {
            None
        };

        // Cargar clave fallback desde disco a cache local
        let mut local = std::collections::HashMap::new();
        let fallback = fs::read(&fallback_public_pem_path)?;
        local.insert("fallback".to_string(), fallback);

        Ok(Self {
            local_cache: Arc::new(RwLock::new(local)),
            redis,
        })
    }

    /// Publica clave pública por kid (jwk:{kid}) - rotación estilo "kid".
    #[allow(dead_code)]
    pub async fn redis_publish_public_key(
        &self,
        kid: &str,
        public_pem: &[u8],
    ) -> Result<(), VerifyError> {
        let Some(r) = &self.redis else {
            return Ok(());
        };
        let mut conn = r.write().await;
        let key = format!("jwk:{kid}");
        let _: () = conn.set(key, public_pem).await?;
        Ok(())
    }

    /// Marca kid activo (jwk:active_kid).
    #[allow(dead_code)]
    pub async fn redis_set_active_kid(&self, kid: &str) -> Result<(), VerifyError> {
        let Some(r) = &self.redis else {
            return Ok(());
        };
        let mut conn = r.write().await;
        let _: () = conn.set("jwk:active_kid", kid).await?;
        Ok(())
    }

    /// DEMO: guardar UNA clave pública bajo una key fija (REDIS_PUBKEY_KEY)
    #[allow(dead_code)]
    pub async fn redis_set_pubkey_single(
        &self,
        redis_key: &str,
        public_pem: &[u8],
    ) -> Result<(), VerifyError> {
        let Some(r) = &self.redis else {
            return Ok(());
        };
        let mut conn = r.write().await;
        let _: () = conn.set(redis_key, public_pem).await?;
        Ok(())
    }

    /// DEMO: cargar UNA clave pública desde Redis (REDIS_PUBKEY_KEY)
    /// y meterla en cache local como kid="redis"
    #[allow(dead_code)]
    pub async fn redis_load_pubkey_single_into_cache(
        &self,
        redis_key: &str,
    ) -> Result<(), VerifyError> {
        let Some(r) = &self.redis else {
            return Ok(());
        };
        let mut conn = r.write().await;
        let pem: Option<Vec<u8>> = conn.get(redis_key).await?;
        if let Some(pem) = pem {
            let mut cache = self.local_cache.write().await;
            cache.insert("redis".to_string(), pem);
        }
        Ok(())
    }

    /// Obtiene clave pública:
    /// 1) cache local por kid
    /// 2) redis jwk:{kid}
    /// 3) fallback (public.pem)
    pub async fn get_public_key_pem(&self, kid: &str) -> Result<Vec<u8>, VerifyError> {
        // 1) cache local
        {
            let cache = self.local_cache.read().await;
            if let Some(pem) = cache.get(kid) {
                return Ok(pem.clone());
            }
        }

        // 2) redis jwk:{kid}
        if let Some(r) = &self.redis {
            let mut conn = r.write().await;
            let redis_key = format!("jwk:{kid}");
            let pem: Option<Vec<u8>> = conn.get(redis_key).await?;
            if let Some(pem) = pem {
                let mut cache = self.local_cache.write().await;
                cache.insert(kid.to_string(), pem.clone());
                return Ok(pem);
            }
        }

        // 3) fallback
        let fallback = {
            let cache = self.local_cache.read().await;
            cache.get("fallback").cloned()
        };

        fallback.ok_or_else(|| VerifyError::PublicKeyNotFound(kid.to_string()))
    }
}

pub struct Verifier {
    issuer: String,
    keystore: Arc<KeyStore>,
}

impl Verifier {
    pub fn new(issuer: impl Into<String>, keystore: Arc<KeyStore>) -> Self {
        Self {
            issuer: issuer.into(),
            keystore,
        }
    }

    /// Valida JWT:
    /// - solo EdDSA
    /// - iss requerido
    /// - exp requerido y validado
    pub async fn verify(&self, token: &str) -> Result<TokenData<Claims>, VerifyError> {
        let header = decode_header(token)?;
        let kid = header.kid.ok_or(VerifyError::MissingKid)?;

        // Seguridad: SOLO EdDSA permitido (bloquea none + simétricos + demás)
        if header.alg != Algorithm::EdDSA {
            return Err(VerifyError::Jwt(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidAlgorithm,
            )));
        }

        let pem = self.keystore.get_public_key_pem(&kid).await?;
        let decoding_key = DecodingKey::from_ed_pem(&pem)?;

        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.algorithms = vec![Algorithm::EdDSA];
        validation.set_issuer(std::slice::from_ref(&self.issuer));
        validation.validate_exp = true;

        let mut req = HashSet::new();
        req.insert("exp".to_string());
        req.insert("iss".to_string());
        req.insert("sub".to_string());
        validation.required_spec_claims = req;

        Ok(decode::<Claims>(token, &decoding_key, &validation)?)
    }
}
