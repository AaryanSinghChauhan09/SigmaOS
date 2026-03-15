# Generated method: SigmaMath.cosine_similarity
import sys
import os
import time
import json
import hashlib
import hmac
import urllib.request
import subprocess

class SigmaMath:
    @staticmethod
    def cosine_similarity(v1: list, v2: list):
        dot = sum((a * b for a, b in zip(v1, v2)))
        return dot / (SigmaMath.l2_norm(v1) * SigmaMath.l2_norm(v2) + 1e-09)