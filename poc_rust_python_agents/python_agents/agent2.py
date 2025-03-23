# agent2.py - Listens for messages from Agent1 and replies.

import uvicorn
from fastapi import FastAPI, Request
import yaml
from comms.didcomm_service import DidCommService
from comms.transport import Transport

app = FastAPI()

# Load config
with open("config/agent2_config.yaml", "r") as f:
    config = yaml.safe_load(f)

AGENT_PORT = config.get("port", 8001)
RUSTCORE_URL = config.get("rustcore_url", "http://localhost:3000")
AGENT1_ENDPOINT = config.get("agent1_endpoint", "http://localhost:8000/messages")

# Real DIDs and Key Paths
AGENT2_DID = config.get("did")
AGENT2_KEY_PATH = config.get("key_path")
AGENT1_DID = config.get("recipient_did")

didcomm_service = DidCommService(RUSTCORE_URL, AGENT2_DID, AGENT2_KEY_PATH)
transport = Transport()

@app.post("/messages")
async def receive_message(request: Request):
    ciphertext = await request.body()
    plaintext = didcomm_service.unpack_message(ciphertext)
    print(f"[Agent2] 📩 Received: {plaintext}")

    reply = f"Thanks, Agent1. I received your message: {plaintext}"
    reply_cipher = didcomm_service.pack_message(
        sender_did=AGENT2_DID,
        recipient_did=AGENT1_DID,
        plaintext=reply
    )

    try:
        response = transport.send_message(AGENT1_ENDPOINT, reply_cipher)
        print(f"[Agent2] 🔁 Sent reply: {response}")
        return {
            "status": "received_by_agent2",
            "plaintext": plaintext,
            "reply_sent": reply
        }
    except Exception as e:
        print(f"[Agent2] ❌ Failed to send reply: {e}")
        return {
            "status": "received_by_agent2",
            "plaintext": plaintext,
            "reply_sent": "failed"
        }

def main():
    print(f"[Agent2] 🌐 Listening on port {AGENT_PORT}")
    uvicorn.run(app, host="0.0.0.0", port=AGENT_PORT)

if __name__ == "__main__":
    main()
