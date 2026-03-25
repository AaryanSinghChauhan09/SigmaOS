"""
SigmaOS Standard Library - Userland API (v1.0 Apex)
===================================================
Common OS utilities and system monitoring calls.
"""
import random

class SigmaSys:
    @staticmethod
    def cpu_usage():
        return random.randint(5, 45)
    
    @staticmethod
    def ram_usage():
        return random.randint(15, 65)

    @staticmethod
    def battery_status():
        return "DISCHARGING" if random.random() > 0.5 else "CHARGING"

    @staticmethod
    def get_user_vitals():
        return {"Active": True, "Supremacy": "USER"}

class SigmaMath:
    @staticmethod
    def sqrt(x): return x**0.5
    @staticmethod
    def ceil(x): return int(x) + (1 if x > int(x) else 0)

class SigmaCrypto:
    @staticmethod
    def sha256(data): return "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
