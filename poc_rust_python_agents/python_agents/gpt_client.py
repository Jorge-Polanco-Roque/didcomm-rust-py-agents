"""
gpt_client.py

Provides a function to communicate with the OpenAI GPT-4 API.

Requirements:
- openai library
- OPENAI_API_KEY environment variable
"""

import openai
import os
import sys

# Retrieve the API key from environment variables.
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    print("Warning: The environment variable 'OPENAI_API_KEY' is not set.")
    print("Calls to ask_gpt() will fail unless you set the key.")
openai.api_key = api_key

def ask_gpt(prompt: str) -> str:
    """
    Sends a prompt to GPT-4 and returns the response as a string.

    :param prompt: The user prompt or query.
    :return: The response from GPT-4 as a string.
    """
    if not api_key:
        # No API key, just return a placeholder response.
        return "Error: Missing OPENAI_API_KEY."
    try:
        response = openai.ChatCompletion.create(
            model="gpt-4",
            messages=[{"role": "user", "content": prompt}],
            temperature=0.7
        )
        return response.choices[0].message.content.strip()
    except Exception as e:
        return f"An error occurred calling GPT-4: {e}"
