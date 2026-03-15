# Generated method: SigmaBharatLawBridge.generate_external_search_url
from typing import Dict, List, Any, Optional
import datetime

class SigmaBharatLawBridge:
    def generate_external_search_url(self, platform: str, query: str) -> str:
        """Generates deep-links to Indian legal databases."""
        import urllib.parse
        q = urllib.parse.quote(query)
        if platform == 'IndianKanoon':
            return f'https://indiankanoon.org/search/?formInput={q}'
        if platform == 'IndiaCode':
            return f'https://www.indiacode.nic.in/handle/123456789/1362/simple-search?query={q}'
        return f'Searching OS for local context of {query}...'