"""
transport.py

Handles sending messages to a remote agent via HTTP POST.
"""

import requests

class Transport:
    def send_message(self, endpoint_url: str, ciphertext: bytes) -> dict:
        """
        Sends ciphertext to another agent's /messages endpoint.
        
        :param endpoint_url: The URL (e.g., http://localhost:8001/messages) of the receiving agent.
        :param ciphertext: The encrypted message as bytes.
        :return: The JSON response from the receiving agent.
        """
        response = requests.post(endpoint_url, data=ciphertext)
        response.raise_for_status()
        return response.json()
