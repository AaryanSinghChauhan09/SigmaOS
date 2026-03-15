# Generated method: ZenithOrchestrator._load_nodes
import json
import time
import uuid
import base64
import random
import platform
import threading
from pathlib import Path
from typing import Dict, List, Optional
from cryptography.fernet import Fernet
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC

class ZenithOrchestrator:
    def _load_nodes(self) -> List[Dict]:
        if self.nodes_path.exists():
            try:
                with open(self.nodes_path, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except:
                pass
        return [{'name': 'ChatGPT', 'url': 'https://chatgpt.com/', 'color': '#10a37f', 'category': 'AI'}, {'name': 'Claude', 'url': 'https://claude.ai/new', 'color': '#d97757', 'category': 'AI'}, {'name': 'Gemini', 'url': 'https://gemini.google.com/app', 'color': '#4285f4', 'category': 'AI'}, {'name': 'Copilot', 'url': 'https://copilot.microsoft.com/', 'color': '#00a4ef', 'category': 'AI'}]