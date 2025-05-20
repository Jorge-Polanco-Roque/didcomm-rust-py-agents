# agent2.py ─ recibe mensajes de Agent1 y contesta con GPT-4

import os
import uvicorn
import yaml
from fastapi import FastAPI, Request

from comms.didcomm_service import DidCommService
from comms.transport import Transport
from gpt_client import ask_gpt  # ⬅️  llamada al LLM
from dotenv import load_dotenv  # opcional: lee .env si existe

load_dotenv()  # carga OPENAI_API_KEY del entorno o de un .env local

app = FastAPI()

# ─────────────────────── Configuración ────────────────────────
with open("config/agent2_config.yaml", "r") as f:
    config = yaml.safe_load(f)

AGENT_PORT      = config.get("port", 8001)
RUSTCORE_URL    = config.get("rustcore_url", "http://localhost:3000")
AGENT1_ENDPOINT = config.get("agent1_endpoint", "http://localhost:8000/messages")

AGENT2_DID      = config.get("did")
AGENT2_KEY_PATH = config.get("key_path")
AGENT1_DID      = config.get("recipient_did")

didcomm_service = DidCommService(RUSTCORE_URL, AGENT2_DID, AGENT2_KEY_PATH)
transport       = Transport()

# ────────────────────────── Handler ───────────────────────────
@app.post("/messages")
async def receive_message(request: Request):
    ciphertext = await request.body()
    plaintext  = didcomm_service.unpack_message(ciphertext)
    print(f"[Agent2] 📩 Received: {plaintext}")

    # ---------- GPT-4 genera la respuesta ----------
    prompt = (
        "You are Bob, a DIDComm agent. "
        "Reply politely, using no more than 30 words, "
        f"to the message you just received from Alice:\n«{plaintext}»"
    )
    reply = ask_gpt(prompt).strip()

    # fallback si la llamada falló
    if reply.startswith("Error:") or reply.lower().startswith("an error"):
        reply = f"Thanks, Agent1. I received your message: {plaintext}"

    # ---------- Cifra y reenvía ----------
    reply_cipher = didcomm_service.pack_message(
        sender_did    = AGENT2_DID,
        recipient_did = AGENT1_DID,
        plaintext     = reply
    )

    try:
        response = transport.send_message(AGENT1_ENDPOINT, reply_cipher)
        print(f"[Agent2] 🔁 Sent reply: {reply}")
        return {
            "status"     : "received_by_agent2",
            "plaintext"  : plaintext,
            "reply_sent" : reply,
            "agent1_resp": response,
        }
    except Exception as e:
        print(f"[Agent2] ❌ Failed to send reply: {e}")
        return {
            "status"     : "received_by_agent2",
            "plaintext"  : plaintext,
            "reply_sent" : "failed",
            "error"      : str(e),
        }

# ────────────────────────── Main ──────────────────────────────
def main():
    print(f"[Agent2] 🌐 Listening on port {AGENT_PORT}")
    uvicorn.run(app, host="0.0.0.0", port=AGENT_PORT)

if __name__ == "__main__":
    main()
