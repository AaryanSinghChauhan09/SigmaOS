# Generated method: AntigravityEngine.__init__
import urllib.parse
import webbrowser
import threading
import time
import json
import os
from typing import List, Dict, Any, Optional
from sigma_core.ai.antigravity_manifest import PLATFORMS, QUOTA_DEFAULTS

class AntigravityEngine:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.platforms = PLATFORMS
        self.quotas = dict(QUOTA_DEFAULTS)
        self.history: List[Dict[str, Any]] = []
        self.server_online = False