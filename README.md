# 🚀 DIDComm + GPT-4 Proof of Concept

This repo shows two Python agents (“Alice” and “Bob”) that exchange **DIDComm v2** messages.

A tiny Rust service (`rustcore`) performs encryption/decryption, while **Bob uses GPT-4** to generate his reply.

---

## 📁 Repository Layout

```text
poc_rust_python_agents/
├─ python_agents/            # Python 3.11 virtual-env & code
│  ├─ agent1.py              # Alice – fixed greeting
│  ├─ agent2.py              # Bob – GPT-4 reply
│  ├─ comms/                 # HTTP transport + DIDComm helper
│  │  ├─ didcomm_service.py
│  │  └─ transport.py
│  ├─ config/                # YAML configs
│  │  ├─ agent1_config.yaml
│  │  ├─ agent2_config.yaml
│  │  └─ agents_catalog.json
│  ├─ keys/                  # Ed25519 JWKs
│  │  ├─ agent1.key
│  │  └─ agent2.key
│  ├─ gpt_client.py          # Thin OpenAI wrapper
│  ├─ requirements.txt
│  └─ venv/                  # local virtual-env
├─ rustcore/                 # Rust micro-service
│  ├─ Cargo.toml
│  └─ src/
│     ├─ didcomm.rs          # XC20P pack/unpack
│     └─ main.rs             # axum REST server
└─ tests/                    # helper zsh scripts
   ├─ service.zsh
   └─ test.zsh
```

---

## 1 · Prerequisites

| Tool | Version (tested) | Notes |
|------|-----------------|-------|
| **Python** | 3.11.x | Activate the local venv in `python_agents/` and run:<br>`pip install -r requirements.txt` |
| **Rust / Cargo** | ≥ 1.85 | Needed to build & run `rustcore` (`cargo run`) |
| **OpenAI API key** | GPT-4 access | Export `OPENAI_API_KEY` or place it in a `.env` file inside `python_agents/` |

---

## 2 · Verify GPT-4 Connectivity

```bash
cd python_agents
source venv/bin/activate

export OPENAI_API_KEY="sk-XXXXXXXXXXXXXXXXXXXXXXXX"

python - <<'PY'
from gpt_client import ask_gpt
print( ask_gpt("Write a 5-word poem about autumn.") )
PY
```

If you receive a poem `(not Error: Missing OPENAI_API_KEY)` you’re ready.

---

## 3 · Run the Demo (three terminals)

> **All terminals** must activate the same `python_agents/venv`.

| Terminal            | Commands                                                                 |
|---------------------|--------------------------------------------------------------------------|
| A – Rust core        | `bash cd rustcore && cargo run`                                          |
| B – Agent 2 (Bob)    | `bash cd python_agents && source venv/bin/activate && python agent2.py` |
| C – Agent 1 (Alice)  | `bash cd python_agents && source venv/bin/activate && python agent1.py` |

**Expected output**:
```text
[Agent1] 🚀 Sending: Hello from Agent1!
[Agent2] 📩 Received: Hello from Agent1!
[Agent2] 💬 Sent reply: (GPT-4 answer, ≤30 words)
[Agent1] ✅ Agent2 responded: {...}
```

Stop `rustcore` with **Ctrl-C** (or `kill <PID>`).

---

## 4 · Component Details

### 4.1 `rustcore`

- **axum** REST endpoints: `/didcomm/pack` and `/didcomm/unpack`.
- **didcomm-rs 0.7** anon-crypt (X25519 / XC20P).
- Keys are hard-coded base58 X25519 (derived from the JWKs in `python_agents/keys/`).

### 4.2 `python_agents`

| File                       | Responsibility                                                                 |
|----------------------------|---------------------------------------------------------------------------------|
| `agent1.py`               | Sends a fixed greeting to Bob.                                                  |
| `agent2.py`               | Decrypts Alice’s message → calls GPT-4 → encrypts reply.                        |
| `comms/didcomm_service.py`| Thin wrapper that calls `rustcore` HTTP endpoints.                              |
| `gpt_client.py`           | Minimal OpenAI ChatCompletion helper (`dotenv` auto-loads `.env`).              |

---

## 5 · Next Steps

1. **Real DID resolution** – replace hard-coded keys with DID Documents from an external resolver (e.g. `did:web` or `did:ion`).

2. **Auth-crypt & Ed25519 signatures** – move beyond anon-crypt to authenticated encryption and message signatures.

3. **Docker Compose** – containerize `rustcore` and the Python agents for one-command startup.

4. **Web dashboard** – live view of plaintext ↔ ciphertext plus GPT prompt tweaking.

5. **Persistent chat history** – store each exchange in SQLite/PostgreSQL for auditing and analytics.
