"""
Auto-split from userland\system_api\sigma_std.py — SigmaCrypto.sign
"""

import sys
import os
import time
import json
import hashlib
import hmac
import urllib.request
import subprocess



class SigmaCrypto:
    @staticmethod
    def sign(data: str, key: str='SOVEREIGN_KEY'):
        return hmac.new(key.encode(), data.encode(), hashlib.sha256).hexdigest()
