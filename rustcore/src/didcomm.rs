//! didcomm.rs – XC20P anon-crypt con didcomm-rs 0.7.2

use didcomm_rs::{crypto::CryptoAlgorithm, Message};
use bs58::decode as b58;
use serde_json::json;

// ===== 1.  Sustituye por las NUEVAS claves X25519 =====

// Agent 1  (Alice)
const A1_DID  : &str = "did:key:z6Mkw8NrAshhXaY67KSL5Kaedf5qPLVUPEB3ntnZDJXytAgZ";
const A1_PRIV : &str = "Dx6AaE2TTaYkjiWNhvfetsJ16KVNRCLaeyQ8VXxMW5Aa";   // base58
const A1_PUB  : &str = "JAHqyFgUAcWtG62jEWMRd3p62qwYuErwWxvWJYBVhr43";   // base58

// Agent 2  (Bob)
const A2_DID  : &str = "did:key:z6MkmDkti6WsQqRLeY9BgKdF61hbV9qncAiZFv3jysnsAZ1t";
const A2_PRIV : &str = "F3waHjMNTgRgVLnMzbveP6mBpar2HJw8anU5TYi7tjjn";   // base58
const A2_PUB  : &str = "DEKu7MBjS3QpZ4mh4ZAcGCzvNMe8YA9itYnVtyTKPPHy";   // base58

// ===== 2.  Helpers: todo en base58 =====
fn priv_bytes(b58s: &str) -> Vec<u8> { b58(b58s).into_vec().unwrap() }
fn pub_bytes (b58s: &str) -> Vec<u8> { b58(b58s).into_vec().unwrap() }

// ===== 3.  Empaquetar =====
pub fn pack_message(sender: &str, recipient: &str, text: &str) -> String {
    let (my_priv, recip_pk) = match (sender, recipient) {
        (A1_DID, A2_DID) => (priv_bytes(A1_PRIV), pub_bytes(A2_PUB)),
        (A2_DID, A1_DID) => (priv_bytes(A2_PRIV), pub_bytes(A1_PUB)),
        _ => panic!("unknown DID pair"),
    };

    let body_str = serde_json::to_string(&json!({ "content": text })).unwrap();

    Message::new()
        .from(sender)
        .to(&[recipient])
        .body(&body_str)
        .as_jwe(&CryptoAlgorithm::XC20P, Some(recip_pk.clone()))
        .seal(&my_priv, Some(vec![Some(recip_pk)]))     // anon-crypt (1 receptor)
        .unwrap()
}

// ===== 4.  Desempaquetar =====
pub fn unpack_message(cipher: &str, my_did: &str) -> String {
    let my_priv = match my_did {
        A1_DID => priv_bytes(A1_PRIV),
        A2_DID => priv_bytes(A2_PRIV),
        _ => panic!("unknown DID"),
    };

    let msg = Message::receive(cipher, Some(&my_priv), None, None)
        .expect("decrypt");

    let body_json = msg.get_body().unwrap_or_default();
    let v: serde_json::Value = serde_json::from_str(&body_json).unwrap_or_default();
    v["content"].as_str().unwrap_or_default().to_string()
}
