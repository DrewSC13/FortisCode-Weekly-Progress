//! Signature Observer (v0.1.0)
//!
//! Patrón Observer: notificación uno-a-muchos cuando se detecta una firma inválida.

/// Trait Observer: define update(&self, event_details: &str).
pub trait SecurityObserver {
    fn update(&self, event_details: &str);
}

#[path = "observers/logger.rs"]
pub mod logger;

#[path = "observers/alerts.rs"]
pub mod alerts;

#[path = "observers/ips.rs"]
pub mod ips;

pub use alerts::AlertSystem;
pub use ips::IntrusionPreventer;
pub use logger::AuditLogger;

use std::sync::Arc;

/// Sujeto (Subject): SignatureValidator
/// - Permite registrar observers
/// - Notifica a todos cuando validate_signature(false)
pub struct SignatureValidator {
    observers: Vec<Box<dyn SecurityObserver>>,
    event_details: String,
}

impl SignatureValidator {
    pub fn new(event_details: impl Into<String>) -> Self {
        Self {
            observers: Vec::new(),
            event_details: event_details.into(),
        }
    }

    pub fn add_observer(&mut self, observer: Box<dyn SecurityObserver>) {
        self.observers.push(observer);
    }

    /// Si is_valid es false, notifica a todos los observers.
    pub fn validate_signature(&self, is_valid: bool) {
        if !is_valid {
            for obs in &self.observers {
                obs.update(&self.event_details);
            }
        }
    }
}

impl<T: SecurityObserver + ?Sized> SecurityObserver for Arc<T> {
    fn update(&self, event_details: &str) {
        (**self).update(event_details);
    }
}
