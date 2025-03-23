# didcomm_service.py
import requests
import subprocess
import json

class DidCommService:
    def __init__(self, rustcore_url: str, did: str, key_path: str):
        self.rustcore_url = rustcore_url
        self.did = did
        self.key_path = key_path

    def pack_message(self, sender_did: str, recipient_did: str, plaintext: str) -> bytes:
        payload = {
            "sender_did": sender_did,
            "recipient_did": recipient_did,
            "plaintext": plaintext
        }
        resp = requests.post(f"{self.rustcore_url}/didcomm/pack", json=payload)
        resp.raise_for_status()
        data = resp.json()
        return data["ciphertext"].encode("utf-8")

    def unpack_message(self, ciphertext: bytes) -> str:
        payload = {"ciphertext": ciphertext.decode("utf-8")}
        resp = requests.post(f"{self.rustcore_url}/didcomm/unpack", json=payload)
        resp.raise_for_status()
        data = resp.json()
        return data["plaintext"]

    def sign_payload(self, message: str) -> str:
        """Sign a message using the agent's private key and DID"""
        try:
            result = subprocess.run(
                ["didkit", "did-auth", "--key-path", self.key_path, "--did", self.did],
                input=message.encode(),
                capture_output=True,
                check=True
            )
            return result.stdout.decode().strip()
        except subprocess.CalledProcessError as e:
            print(f"[DidCommService] ❌ Failed to sign: {e.stderr.decode().strip()}")
            return ""
