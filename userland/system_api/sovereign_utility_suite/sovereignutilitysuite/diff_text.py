"""
Auto-split from userland\system_api\sovereign_utility_suite.py — SovereignUtilitySuite.diff_text
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
    def diff_text(self, text_a: str, text_b: str) -> str:
        """USP: CompareText.io Parity. Visualizing diffs locally."""
        d = difflib.HtmlDiff()
        return d.make_table(text_a.splitlines(), text_b.splitlines())
