use signature_observer::{AlertSystem, AuditLogger, IntrusionPreventer, SignatureValidator};
use std::sync::Arc;

#[test]
fn signature_invalid_notifies_all_observers_in_one_call() {
    // Event contiene la IP para que IntrusionPreventer la extraiga
    let mut validator = SignatureValidator::new("INVALID_SIGNATURE ip=10.0.0.7");

    let logger = Arc::new(AuditLogger::new());
    let alerts = Arc::new(AlertSystem::new());
    let ips = Arc::new(IntrusionPreventer::new());

    // Registramos los observers
    validator.add_observer(Box::new(Arc::clone(&logger)));
    validator.add_observer(Box::new(Arc::clone(&alerts)));
    validator.add_observer(Box::new(Arc::clone(&ips)));

    validator.validate_signature(false);

    // Logger reaccionó
    let logs = logger.logs();
    assert_eq!(logs.len(), 1);
    assert!(logs[0].contains("INVALID_SIGNATURE"));

    // AlertSystem reaccionó
    assert!(alerts.is_red_alert());

    // IntrusionPreventer reaccionó (bloqueó la IP)
    let blocked = ips.blacklist();
    assert_eq!(blocked, vec!["10.0.0.7".to_string()]);
}
