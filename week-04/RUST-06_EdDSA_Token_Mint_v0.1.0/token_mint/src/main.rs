use std::env;
use std::sync::Arc;

mod auth {
    pub mod signer;
    pub mod verifier;
}

use auth::signer::Signer;
use auth::verifier::{KeyStore, Verifier};

fn usage() -> ! {
    eprintln!(
        r#"Usage:
  cargo run -- mint <user_id>
  cargo run -- verify <jwt>
  cargo run -- push-pubkey-redis

Env:
  TOKEN_ISS=fortiscode-auth
  REDIS_URL=redis://127.0.0.1:6379
  REDIS_PUBKEY_KEY=token_mint:pubkey
"#
    );
    std::process::exit(2);
}

fn must_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        eprintln!("Missing env var: {name}");
        std::process::exit(2);
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| usage());

    let issuer = must_env("TOKEN_ISS");
    let redis_url = env::var("REDIS_URL").ok();
    let redis_pubkey_key = env::var("REDIS_PUBKEY_KEY").ok();

    match cmd.as_str() {
        "mint" => {
            let user_id = args.next().unwrap_or_else(|| usage());

            // firmar con private.pem (solo Auth)
            // kid fijo para demo (puedes versionarlo)
            let signer =
                Signer::from_private_pem_file("keys/private.pem", "kid-demo", &issuer, 15)?;
            let jwt = signer.mint(user_id)?;
            print!("{jwt}");
        }

        "verify" => {
            let jwt = args.next().unwrap_or_else(|| usage());

            // microservicios: solo public.pem + opcional redis
            let keystore: Arc<KeyStore> =
                Arc::new(KeyStore::new(&issuer, "keys/public.pem", redis_url).await?);

            // si existe REDIS_PUBKEY_KEY, intenta refrescar desde redis (sin restart)
            if let Some(k) = &redis_pubkey_key {
                // carga y cachea como kid="redis" (si existe)
                let _ = keystore.redis_load_pubkey_single_into_cache(k).await;
            }

            // verificador: validación iss + exp, solo EdDSA
            let verifier = Verifier::new(&issuer, keystore);

            // IMPORTANTÍSIMO:
            // el token trae kid en header (kid-demo normalmente).
            // si quieres que verifique contra kid="redis", entonces en mint deberías poner kid="redis"
            // o guardar en cache el kid que viene. Para demo simple, dejamos kid-demo y public.pem local.
            let data = verifier.verify(&jwt).await?;
            println!(
                "OK sub={} iss={} exp={}",
                data.claims.sub, data.claims.iss, data.claims.exp
            );
        }

        "push-pubkey-redis" => {
            let redis_url = redis_url.unwrap_or_else(|| {
                eprintln!("REDIS_URL is required for push-pubkey-redis");
                std::process::exit(2);
            });
            let redis_pubkey_key = redis_pubkey_key.unwrap_or_else(|| {
                eprintln!("REDIS_PUBKEY_KEY is required for push-pubkey-redis");
                std::process::exit(2);
            });

            let keystore: Arc<KeyStore> =
                Arc::new(KeyStore::new(&issuer, "keys/public.pem", Some(redis_url)).await?);

            let pub_pem = std::fs::read("keys/public.pem")?;
            keystore
                .redis_set_pubkey_single(&redis_pubkey_key, &pub_pem)
                .await?;

            println!("Pushed public key to redis key={redis_pubkey_key}");
        }

        _ => usage(),
    }

    Ok(())
}
