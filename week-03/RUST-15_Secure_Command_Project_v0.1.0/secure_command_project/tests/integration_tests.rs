use secure_command_project::{
    CommandController, RotateKeysCommand, SystemState, WipeSensitiveDataCommand,
};

#[test]
fn controller_executes_commands_and_records_exact_history() {
    // Estado inicial del sistema
    let original_key = b"KEY_V1".to_vec();
    let audit_buf = b"VERY_SENSITIVE_LOGS".to_vec();
    let mut system = SystemState::new(original_key.clone(), audit_buf.clone());

    // 1) Añadir varios comandos al controlador
    let mut controller = CommandController::new();

    controller.enqueue(Box::new(RotateKeysCommand::new(b"KEY_V2".to_vec())));
    controller.enqueue(Box::new(WipeSensitiveDataCommand::new()));

    // 2) Ejecutarlos todos en secuencia
    controller.execute_all(&mut system).unwrap();

    // 3) Verificar que el estado cambió según lo esperado
    assert_eq!(system.master_key(), b"KEY_V2");
    assert!(system.audit_buffer().iter().all(|&b| b == 0));

    // 4) Verificar historial exacto
    assert_eq!(controller.history_len(), 2);
    assert_eq!(
        controller.history(),
        &[
            "RotateKeysCommand".to_string(),
            "WipeSensitiveDataCommand".to_string()
        ]
    );
}
