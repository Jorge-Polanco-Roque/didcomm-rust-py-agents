// did_registry.rs

use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DID_KEYS: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

pub fn load_did_key(did: &str, key_path: &str) {
    let key_content = fs::read_to_string(key_path)
        .unwrap_or_else(|_| panic!("Could not read key file: {}", key_path));

    DID_KEYS.write().unwrap().insert(did.to_string(), key_content);
}

pub fn get_key_for_did(did: &str) -> Option<String> {
    DID_KEYS.read().unwrap().get(did).cloned()
}
