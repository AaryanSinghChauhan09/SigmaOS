# Generated method: SigmaNetwork.fetch
import sys
import os
import time
import json
import hashlib
import hmac
import urllib.request
import subprocess

class SigmaNetwork:
    @staticmethod
    def fetch(url: str, data: dict=None, timeout: int=10):
        try:
            if data:
                req_data = json.dumps(data).encode('utf-8')
                req = urllib.request.Request(url, data=req_data, method='POST')
            else:
                req = urllib.request.Request(url, method='GET')
            with urllib.request.urlopen(req, timeout=timeout) as response:
                return response.read().decode('utf-8')
        except Exception as e:
            return f'Error: {e}'