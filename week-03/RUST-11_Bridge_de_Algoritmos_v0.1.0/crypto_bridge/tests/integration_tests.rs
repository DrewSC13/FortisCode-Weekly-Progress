use crypto_bridge::{DigitalSignature, NativeRustBackend, OpenSSLBackend};

#[test]
fn digital_signature_allows_backend_injection_and_runtime_swap() {
    let msg = b"mensaje-firma";

    // 1) DigitalSignature con OpenSSLBackend
    let ds_openssl = DigitalSignature::new(Box::new(OpenSSLBackend));
    let sig_openssl = ds_openssl.sign(msg);
    assert!(ds_openssl.verify(msg, &sig_openssl));

    // 2) DigitalSignature con NativeRustBackend
    let ds_native = DigitalSignature::new(Box::new(NativeRustBackend));
    let sig_native = ds_native.sign(msg);
    assert!(ds_native.verify(msg, &sig_native));

    // “Resultados coherentes con su implementación”:
    // ambos verifican correctamente, pero sus bytes difieren (distinto backend)
    assert_ne!(sig_openssl, sig_native);

    // 3) Intercambio de backend en tiempo de ejecución
    let mut ds_swap = DigitalSignature::new(Box::new(OpenSSLBackend));
    let s1 = ds_swap.sign(msg);
    assert!(ds_swap.verify(msg, &s1));

    ds_swap.set_backend(Box::new(NativeRustBackend));
    let s2 = ds_swap.sign(msg);
    assert!(ds_swap.verify(msg, &s2));

    // evidencia del cambio de backend
    assert_ne!(s1, s2);
}
