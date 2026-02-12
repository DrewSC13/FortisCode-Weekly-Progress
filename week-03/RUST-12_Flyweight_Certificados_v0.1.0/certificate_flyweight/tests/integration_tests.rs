use certificate_flyweight::CertificateFactory;
use std::sync::Arc;

#[test]
fn thousand_certificates_share_same_issuer_data_pointer() {
    let mut factory = CertificateFactory::new();

    // Creamos 1000 certificados del mismo emisor
    let mut issuers: Vec<Arc<_>> = Vec::with_capacity(1000);

    for i in 0..1000u64 {
        let cert = factory.create_certificate(
            "FortisCA",
            "BO",
            "RSA-2048",
            i,
            vec![1, 2, 3, 4],
            format!("CN=user-{i}"),
        );

        issuers.push(Arc::clone(cert.issuer()));
    }

    // Verificar que la dirección del IssuerData es la misma para todos
    let first = &issuers[0];
    assert!(issuers.iter().all(|arc| Arc::ptr_eq(first, arc)));

    assert_eq!(factory.issuer_count(), 1);
}
