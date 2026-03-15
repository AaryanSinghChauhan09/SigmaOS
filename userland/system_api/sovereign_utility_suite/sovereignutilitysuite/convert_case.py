"""
Auto-split from userland\system_api\sovereign_utility_suite.py — SovereignUtilitySuite.convert_case
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
    def convert_case(self, text: str, mode: str='sentence') -> str:
        """USP: ConvertCase.net Parity."""
        if mode == 'upper':
            return text.upper()
        if mode == 'lower':
            return text.lower()
        if mode == 'title':
            return text.title()
        if mode == 'sentence':
            return '. '.join([s.strip().capitalize() for s in text.split('.')])
        return text
