"""
Auto-split from userland\system_api\sovereign_utility_suite.py — SovereignUtilitySuite.base64_toolkit
"""

import os
import random
import time
import json
import hashlib
import re
import difflib
import base64
import statistics
from typing import Dict, Any, List, Optional
from datetime import datetime



class SovereignUtilitySuite:
    def base64_toolkit(self, data: str, mode: str='encode') -> str:
        """USP: Img to Base64 / EZGIF Parity."""
        try:
            if mode == 'encode':
                return base64.b64encode(data.encode()).decode()
            return base64.b64decode(data).decode()
        except:
            return 'Error: Invalid data for codec operation.'
