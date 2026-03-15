# Generated method: SigmaSovereignZenith._init_nodes
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import random
import json
import os
from pathlib import Path

class SigmaSovereignZenith:
    def _init_nodes(self):
        """Standard AI nodes available in the Sovereign ecosystem."""
        raw_nodes = [{'name': 'ChatGPT', 'url': 'https://chatgpt.com/', 'color': '#10a37f'}, {'name': 'Claude', 'url': 'https://claude.ai/new', 'color': '#d97757'}, {'name': 'Gemini', 'url': 'https://gemini.google.com/', 'color': '#4285f4'}, {'name': 'Copilot', 'url': 'https://copilot.microsoft.com/', 'color': '#00a4ef'}, {'name': 'Perplexity', 'url': 'https://www.perplexity.ai/', 'color': '#22b8cf'}, {'name': 'Grok', 'url': 'https://grok.x.ai/', 'color': '#1DA1F2'}]
        for n in raw_nodes:
            self.nodes[n['name']] = AINode(**n)