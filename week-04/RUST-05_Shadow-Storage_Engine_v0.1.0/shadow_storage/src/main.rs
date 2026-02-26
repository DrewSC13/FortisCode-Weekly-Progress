mod crypto;

// ✅ sin crear src/repository/mod.rs
mod repository {
    pub mod user_repo;
}

use crate::crypto::aes_engine;
use crate::repository::user_repo::{
    EncryptedUserRow, POSTGRES_SCHEMA_SQL, User, from_encrypted_row, load_key_32, to_encrypted_row,
};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "encrypt" => {
            let plaintext = must_arg(&args, 2)?;
            let key = load_key_32()?;
            let blob = aes_engine::encrypt(&plaintext, key)?;
            println!("{}", hex::encode(blob));
        }
        "decrypt" => {
            let hex_blob = must_arg(&args, 2)?;
            let blob = hex::decode(hex_blob.trim())?;
            let key = load_key_32()?;
            let plaintext = aes_engine::decrypt(&blob, key)?;
            println!("{plaintext}");
        }
        "flipbit" => {
            let hex_blob = must_arg(&args, 2)?;
            let blob = hex::decode(hex_blob.trim())?;
            let tampered = aes_engine::flip_one_bit(blob);
            println!("{}", hex::encode(tampered));
        }
        "demo-row" => {
            let email = must_arg(&args, 2)?;
            let national_id = must_arg(&args, 3)?;

            let u = User {
                id: 1,
                username: "demo".to_string(),
                email,
                national_id,
            };

            let row = to_encrypted_row(&u)?;
            println!("username={}", row.username);
            println!("email_enc_hex={}", hex::encode(&row.email_blob));
            println!("national_id_enc_hex={}", hex::encode(&row.national_id_blob));

            let back = from_encrypted_row(&EncryptedUserRow { ..row })?;
            println!("roundtrip_email={}", back.email);
            println!("roundtrip_national_id={}", back.national_id);
        }
        "print-schema" => {
            // evita dead_code warning y además es útil para la tarea
            println!("{POSTGRES_SCHEMA_SQL}");
        }
        _ => print_help(),
    }

    Ok(())
}

fn must_arg(args: &[String], idx: usize) -> anyhow::Result<String> {
    args.get(idx)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("missing argument at position {idx}"))
}

fn print_help() {
    eprintln!("Usage:");
    eprintln!("  shadow_storage encrypt <plaintext>");
    eprintln!("  shadow_storage decrypt <hex_blob>");
    eprintln!("  shadow_storage flipbit <hex_blob>");
    eprintln!("  shadow_storage demo-row <email> <national_id>");
    eprintln!("  shadow_storage print-schema");
    eprintln!();
    eprintln!("Key env (32 bytes):");
    eprintln!("  SHADOW_KEY_FILE=/run/secrets/shadow_key  (recommended)");
    eprintln!("  SHADOW_KEY_B64=...  or SHADOW_KEY_HEX=...");
}
