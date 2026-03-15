# Generated method: SigmaMath.l2_norm
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
    def l2_norm(vector: list):
        return sum((x * x for x in vector)) ** 0.5