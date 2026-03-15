"""
Auto-split from userland\system_api\ag_enterprise.py — TextCleaner.normalize
"""

import os
import re
import json
import time
from typing import List, Dict, Any, Optional



class TextCleaner:
    def normalize(self, text: str) -> str:
        text = ' '.join(text.split())
        return text.strip()
