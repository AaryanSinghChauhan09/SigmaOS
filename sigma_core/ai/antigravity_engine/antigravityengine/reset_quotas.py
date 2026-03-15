# Generated method: AntigravityEngine.reset_quotas
import urllib.parse
import webbrowser
import threading
import time
import json
import os
from typing import List, Dict, Any, Optional
from sigma_core.ai.antigravity_manifest import PLATFORMS, QUOTA_DEFAULTS

class AntigravityEngine:
    def reset_quotas(self):
        self.quotas = dict(QUOTA_DEFAULTS)