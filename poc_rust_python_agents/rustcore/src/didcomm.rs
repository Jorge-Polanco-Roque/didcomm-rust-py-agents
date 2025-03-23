// src/didcomm.rs
use std::process::{Command, Stdio};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Empaqueta (cifra) un mensaje usando "didkit didcomm pack-encrypted" CLI
pub fn pack_message(sender_did: &str, recipient_did: &str, plaintext: &str) -> String {
    // 1) obtenemos la ruta de la key del sender para firmar
    let key_path = get_key_path_from_did(sender_did);
    let jwk = match fs::read_to_string(&key_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Failed to read sender JWK at {}: {}", key_path.display(), e);
            return "ERROR".to_string();
        }
    };

    // 2) Formamos el JSON de plaintext
    // didkit v0.3.2 no requiere un JSON en particular, pero solemos poner "type", "body", etc.
    let message = format!(
        r#"{{"type":"msg","from":"{}","to":"{}","body":"{}"}}"#,
        sender_did, recipient_did, plaintext
    );

    // 3) Invocamos la CLI
    let child = Command::new("didkit")
        .arg("didcomm")
        .arg("pack-encrypted")
        .arg("--jwk").arg(&jwk)
        .arg("--from").arg(sender_did)
        .arg("--to").arg(recipient_did)
        .arg("--plaintext").arg(&message)
        .stdout(Stdio::piped())
        .output();

    match child {
        Ok(output) => {
            if output.status.success() {
                // Devuelve el ciphertext (JWE)
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                eprintln!("❌ Pack error: {}", String::from_utf8_lossy(&output.stderr));
                "ERROR".to_string()
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to run didkit pack-encrypted: {}", e);
            "ERROR".to_string()
        }
    }
}

/// Desempaqueta (descifra) un mensaje usando "didkit didcomm unpack" CLI
pub fn unpack_message(ciphertext: &str) -> String {
    // Asumimos que el receptor es agent2 => su private key
    let key_path = Path::new("../python_agents/keys/agent2.key");
    let jwk = match fs::read_to_string(&key_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Failed to read recipient JWK at {}: {}", key_path.display(), e);
            return "ERROR".to_string();
        }
    };

    // Invocamos la CLI
    let mut child = match Command::new("didkit")
        .arg("didcomm")
        .arg("unpack")
        .arg("--jwk").arg(&jwk)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to run didkit unpack: {}", e);
            return "ERROR".to_string();
        }
    };

    // Pasamos el ciphertext por stdin
    {
        let stdin = child.stdin.as_mut().expect("❌ Failed to open stdin");
        if let Err(e) = stdin.write_all(ciphertext.as_bytes()) {
            eprintln!("❌ Error writing to stdin: {}", e);
            return "ERROR".to_string();
        }
    }

    // Esperamos output
    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("❌ Error reading output: {}", e);
            return "ERROR".to_string();
        }
    };

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        eprintln!("❌ Unpack error: {}", String::from_utf8_lossy(&output.stderr));
        "ERROR".to_string()
    }
}

/// Mapear DID => ruta de archivo con su JWK
fn get_key_path_from_did(did: &str) -> PathBuf {
    match did {
        "did:key:z6Mkw8NrAshhXaY67KSL5Kaedf5qPLVUPEB3ntnZDJXytAgZ" => {
            Path::new("../python_agents/keys/agent1.key").to_path_buf()
        }
        "did:key:z6MkmDkti6WsQqRLeY9BgKdF61hbV9qncAiZFv3jysnsAZ1t" => {
            Path::new("../python_agents/keys/agent2.key").to_path_buf()
        }
        _ => panic!("❌ Unknown DID: {}", did),
    }
}
