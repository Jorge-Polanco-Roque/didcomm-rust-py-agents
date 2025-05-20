# agent1.py - Starts Agent1, sends a message, and listens for responses.

import uvicorn
import threading
import time
from fastapi import FastAPI, Request
import yaml
from comms.didcomm_service import DidCommService
from comms.transport import Transport

app = FastAPI()

# Load config
with open("config/agent1_config.yaml", "r") as f:
    config = yaml.safe_load(f)

AGENT_PORT = config.get("port", 8000)
RUSTCORE_URL = config.get("rustcore_url", "http://localhost:3000")
AGENT2_ENDPOINT = config.get("agent2_endpoint", "http://localhost:8001/messages")

# Real DIDs and Key Paths
AGENT1_DID = config.get("did")
AGENT1_KEY_PATH = config.get("key_path")
AGENT2_DID = config.get("recipient_did")

didcomm_service = DidCommService(RUSTCORE_URL, AGENT1_DID, AGENT1_KEY_PATH)
transport = Transport()

@app.post("/messages")
async def receive_message(request: Request):
    ciphertext = await request.body()
    plaintext = didcomm_service.unpack_message(ciphertext)
    print(f"[Agent1] ✅ Received reply: {plaintext}")
    return {"status": "received_by_agent1", "plaintext": plaintext}

def start_server():
    print(f"[Agent1] 🌐 Starting server on port {AGENT_PORT}")
    uvicorn.run(app, host="0.0.0.0", port=AGENT_PORT)

def main():
    server_thread = threading.Thread(target=start_server, daemon=True)
    server_thread.start()

    time.sleep(2)

    # Send initial message
    initial_message = "I'm Alice, a DIDComm agent. I wanto to start a conversation with you."

    print(f"[Agent1] 🚀 Sending: {initial_message}")
    ciphertext = didcomm_service.pack_message(
        sender_did=AGENT1_DID,
        recipient_did=AGENT2_DID,
        plaintext=initial_message
    )

    try:
        response = transport.send_message(AGENT2_ENDPOINT, ciphertext)
        print(f"[Agent1] ✅ Agent2 responded: {response}")
    except Exception as e:
        print(f"[Agent1] ❌ Failed to send: {e}")

    server_thread.join()

if __name__ == "__main__":
    main()
