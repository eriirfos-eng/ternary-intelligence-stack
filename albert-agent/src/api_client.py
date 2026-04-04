import requests
import os

class AlbertApiClient:
    def __init__(self):
        # Base URL for the Ternlang API
        self.base_url = "https://ternlang.com"
        # API Key from environment variable
        self.api_key = os.getenv("TERNLANG_API_KEY", "19950617")

    def trit_decide(self, query, evidence=None):
        """
        Public endpoint for triadic decision gating.
        Uses JSON-RPC 2.0 format via the MCP-compatible /mcp endpoint.
        """
        if evidence is None:
            # Default neutral evidence vector (6 dimensions as per brief)
            evidence = [0.5, 0.5, 0.5, 0.5, 0.5, 0.5]
            
        payload = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "trit_decide",
                "arguments": {
                    "evidence": evidence,
                    "query": query
                }
            }
        }
        response = requests.post(f"{self.base_url}/mcp", json=payload)
        response.raise_for_status()
        return response.json()

    def moe_orchestrate(self, query):
        """
        Expert deliberation via the MoE-13 orchestrator.
        Requires X-Ternlang-Key header.
        """
        headers = {"X-Ternlang-Key": self.api_key}
        payload = {"query": query}
        response = requests.post(
            f"{self.base_url}/api/moe/orchestrate", 
            json=payload, 
            headers=headers
        )
        response.raise_for_status()
        return response.json()

    def health_check(self):
        """Standard health check endpoint."""
        response = requests.get(f"{self.base_url}/health")
        response.raise_for_status()
        return response.json()
