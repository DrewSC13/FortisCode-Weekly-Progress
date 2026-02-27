use base64::Engine;
use ed25519_dalek::SigningKey;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

#[path = "../src/auth/verifier.rs"]
mod verifier;
use verifier::{KeyStore, Verifier};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iss: String,
    sub: String,
}

fn mint_with_key(signing_pem: &str, kid: &str, iss: &str, sub: &str, exp_unix: i64) -> String {
    let claims = Claims {
        exp: exp_unix as usize,
        iss: iss.to_string(),
        sub: sub.to_string(),
    };

    let mut header = Header::new(Algorithm::EdDSA);
    header.kid = Some(kid.to_string());

    let key = EncodingKey::from_ed_pem(signing_pem.as_bytes()).unwrap();
    jsonwebtoken::encode(&header, &claims, &key).unwrap()
}

fn split_jwt(jwt: &str) -> (String, String, String) {
    let parts: Vec<&str> = jwt.split('.').collect();
    (
        parts[0].to_string(),
        parts[1].to_string(),
        parts[2].to_string(),
    )
}

fn gen_keypair_pems() -> (String, String) {
    let mut rng = rand::rng();
    let seed: [u8; 32] = rng.random();

    let sk = SigningKey::from_bytes(&seed);

    let priv_pem = sk.to_pkcs8_pem(LineEnding::LF).unwrap().to_string();
    let pub_pem = sk
        .verifying_key()
        .to_public_key_pem(LineEnding::LF)
        .unwrap()
        .to_string();

    (priv_pem, pub_pem)
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

#[test]
fn test_firma_cruzada_rechazada() {
    rt().block_on(async {
        let issuer = "identity.service";

        let (_pem_priv_a, pem_pub_a) = gen_keypair_pems();
        let (pem_priv_b, _pem_pub_b) = gen_keypair_pems();

        std::fs::create_dir_all("target").unwrap();
        let tmp_pub_path = "target/test_public_a.pem";
        std::fs::write(tmp_pub_path, pem_pub_a.as_bytes()).unwrap();

        let keystore: Arc<KeyStore> =
            Arc::new(KeyStore::new(issuer, tmp_pub_path, None).await.unwrap());
        let verifier = Verifier::new(issuer, keystore);

        let exp = (OffsetDateTime::now_utc() + Duration::minutes(15)).unix_timestamp();
        let token = mint_with_key(&pem_priv_b, "kid-a", issuer, "user-1", exp);

        let res = verifier.verify(&token).await;
        assert!(
            res.is_err(),
            "Debe rechazarse firma con clave privada distinta"
        );
    });
}

#[test]
fn test_expiracion_retorna_expiredsignature() {
    rt().block_on(async {
        let issuer = "identity.service";

        let (pem_priv, pem_pub) = gen_keypair_pems();

        std::fs::create_dir_all("target").unwrap();
        let tmp_pub_path = "target/test_public_exp.pem";
        std::fs::write(tmp_pub_path, pem_pub.as_bytes()).unwrap();

        let keystore: Arc<KeyStore> =
            Arc::new(KeyStore::new(issuer, tmp_pub_path, None).await.unwrap());
        let verifier = Verifier::new(issuer, keystore);

        let exp_past = (OffsetDateTime::now_utc() - Duration::minutes(10)).unix_timestamp();
        let token = mint_with_key(&pem_priv, "kid-exp", issuer, "user-exp", exp_past);

        let res = verifier.verify(&token).await;
        assert!(res.is_err());

        let msg = res.err().unwrap().to_string();
        assert!(
            msg.contains("ExpiredSignature") || msg.to_lowercase().contains("expired"),
            "Se esperaba ExpiredSignature, pero fue: {msg}"
        );
    });
}

#[test]
fn prueba_caja_negra_microservicio_b_no_puede_modificar_payload() {
    rt().block_on(async {
        let issuer = "identity.service";

        let (pem_priv, pem_pub) = gen_keypair_pems();

        std::fs::create_dir_all("target").unwrap();
        let tmp_pub_path = "target/test_public_bb.pem";
        std::fs::write(tmp_pub_path, pem_pub.as_bytes()).unwrap();

        let keystore: Arc<KeyStore> =
            Arc::new(KeyStore::new(issuer, tmp_pub_path, None).await.unwrap());
        let verifier = Verifier::new(issuer, keystore);

        let exp = (OffsetDateTime::now_utc() + Duration::minutes(15)).unix_timestamp();
        let token = mint_with_key(&pem_priv, "kid-bb", issuer, "user-123", exp);

        let ok = verifier.verify(&token).await.unwrap();
        assert_eq!(ok.claims.sub, "user-123");

        let (h, p, s) = split_jwt(&token);

        let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(p.as_bytes())
            .unwrap();

        let mut payload_json: serde_json::Value = serde_json::from_slice(&payload_bytes).unwrap();
        payload_json["sub"] = serde_json::Value::String("admin".to_string());

        let new_payload = serde_json::to_vec(&payload_json).unwrap();
        let new_p = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(new_payload);

        let tampered = format!("{h}.{new_p}.{s}");
        let res = verifier.verify(&tampered).await;
        assert!(
            res.is_err(),
            "Debe fallar porque el payload fue modificado sin firmar"
        );
    });
}
