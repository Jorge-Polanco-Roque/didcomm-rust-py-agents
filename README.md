# README.md

## Next Steps:
* Real DID + Cryptography Integration
* Llenar todos los scripts del proyecto
* Documentar el proyecto (usar project2pdf)


## Agents
* gpt_client.py:
    1) Abrir python (terminal): 
        Python
    2) Probar código:
        from gpt_client import ask_gpt
        response = ask_gpt("Hello GPT, how are you?")
        print(response)

* Llave de los agentes:
    didkit generate-ed25519-key > agent1.key
    didkit generate-ed25519-key > agent2.key

* dwd


## Rustcore
* Activar el servicio: 
    cargo run
* Probar servicio (en otra terminal): 
    curl -X POST -H "Content-Type: application/json" \
    -d '{"sender_did":"did:example:agent1","recipient_did":"did:example:agent2","plaintext":"Hello from Rust"}' \
    http://127.0.0.1:3000/didcomm/pack

* Actualizar y correr Cargo:
    cargo clean
    cargo build
    cargo run

* qdw


## Didkit:
* Genera llaves digitales usando JWK (Json Web Key): didkit key generate ed25519
* Ejemplo:
    {
     "kty": "OKP", #Es un tipo de clave para algoritmos de curvas elípticas con clave pública de longitud fija.
     "crv": "Ed25519", #Especifica el algoritmo que se utiliza para la clave.
     "x": "KjxNyeCf5Ayk484-mky3s_uLyKAgJM_22Qn6c9Dly-M", #Esta es la clave pública en formato codificado ("coordenada x").
     "d": "CRP2yYaR5-wec5mS9u-m8XI8fVmpboKaqx_nIVKPtb0" #Esta es la clave privada en formato codificado.
     }
* dqwd
