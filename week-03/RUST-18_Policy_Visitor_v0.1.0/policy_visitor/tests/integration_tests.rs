use policy_visitor::{AccessLogs, KeyRecord, KeyStorage, SecurityElement, SizeAuditor};

#[test]
fn size_auditor_totals_match_sum_for_mixed_alements() {
    let storage = KeyStorage::new(vec![
        KeyRecord::new("a", 2030),
        KeyRecord::new("bbbb", 2031),
    ]);

    let logs = AccessLogs::new(vec![
        "login ok".to_string(),
        "invalid signature".to_string(),
    ]);

    let elements: Vec<Box<dyn SecurityElement>> = vec![Box::new(storage), Box::new(logs)];

    // Calculo esperado manual (misma logica del estimate)
    let expected_storage_size = "a".len() + 4 + "bbbb".len() + 4;
    let expected_logs_size = "login ok".len() + "invalid signature".len();
    let expected_total = expected_storage_size + expected_logs_size;

    let auditor = SizeAuditor::new();
    for el in &elements {
        el.accept(&auditor);
    }

    assert_eq!(auditor.total_bytes(), expected_total);
}
