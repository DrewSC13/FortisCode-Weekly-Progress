use lazy_loader_project::{KeyLoader, store::ColdStore};
use std::time::Instant;

#[test]
fn integration_first_call_is_slow_second_is_fast_and_key_is_correct() {
    let store = ColdStore::new(b"ENCRYPTED:my-private-key");
    let loader = KeyLoader::from_store(&store);

    // 1) Primera llamada: debe ser lenta (porque hace sleep).
    let t1 = Instant::now();
    let key1 = loader.get_key();
    let elapsed1 = t1.elapsed();

    // 2) Segunda llamada: debe ser "instantanea" (cache hit).
    let t2 = Instant::now();
    let key2 = loader.get_key();
    let elapsed2 = t2.elapsed();

    // 3) Clave correcta
    assert_eq!(key1, "my-private-key");
    assert_eq!(key2, "my-private-key");

    // Validacion de tiempo:
    // La primera deberia ser mas lente (>= ~70ms por el sleep de 80ms).
    assert!(elapsed1.as_millis() >= 70);

    // La segunda deberia ser mas lenta (< 1ms).
    assert!(elapsed2.as_micros() < 1000);

    // El descifrado se ejecuta solo 1 vez
    assert_eq!(loader.decrypt_call_count(), 1);
}
