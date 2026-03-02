use serde_json::Value;
use sha2::{Digest, Sha256};

pub fn canonical_json(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            let mut new_map = serde_json::Map::new();
            for k in keys {
                new_map.insert(k.clone(), canonical_json(&map[&k]));
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(canonical_json).collect()),
        other => other.clone(),
    }
}

/// data = event_type + '\n' + timestamp + '\n' + payload_json (canonical)
pub fn audit_data_bytes(event_type: &str, payload: &Value, timestamp_rfc3339: &str) -> Vec<u8> {
    let canonical_payload = canonical_json(payload);
    let payload_str =
        serde_json::to_string(&canonical_payload).expect("payload JSON serialization must work");

    let mut s = String::new();
    s.push_str(event_type);
    s.push('\n');
    s.push_str(timestamp_rfc3339);
    s.push('\n');
    s.push_str(&payload_str);

    s.into_bytes()
}

/// current_hash = SHA256(data || prev_hash)
pub fn compute_current_hash_hex(data_bytes: &[u8], prev_hash_hex: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data_bytes);
    hasher.update(prev_hash_hex.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn genesis_prev_hash() -> String {
    "0".repeat(64)
}
