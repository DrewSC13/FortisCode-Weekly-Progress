use crypto_decorator_project::aes::CryptoError;
use crypto_decorator_project::{AesDecorator, Base64Decorator, PlainTextWriter, SecureWriter};

#[test]
fn onion_base_encrypt_base64_roundtrip_inverse_recovers_original() -> Result<(), CryptoError> {
    // Cebolla de 3 capas (Base -> Cifrado -> Base64)
    let logger = Base64Decorator(AesDecorator(PlainTextWriter));

    let original = "Mensaje-Secreto-123";
    let final_out = logger.write(original);

    // Proceso inverso:
    // 1) Base64 decode -> obtenemos el HEX del cifrado simulado
    let cipher_hex = Base64Decorator::<AesDecorator<PlainTextWriter>>::decode_to_string(&final_out)
        .expect("base64 decode must succeed");

    // 2) Decrypt HEX -> recuperamos el mensaje original
    let recovered = AesDecorator::<PlainTextWriter>::decrypt_hex(&cipher_hex)?;

    assert_eq!(recovered, original);
    Ok(())
}
