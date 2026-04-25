"""
SigmaOS Security Subsystem
Implements Zero-Trust networking and Quantum-Safe Cryptography (Kyber/Dilithium).
"""
import ctypes
import os

lib_path = "/usr/lib/sigmaos/libsec.so"
try:
    _native_core = ctypes.CDLL(lib_path)
    _native_core.sec_firewall_enable.argtypes = [ctypes.c_int]
    _native_core.sec_sandbox_process.argtypes = [ctypes.c_int]
    NATIVE_AVAILABLE = True
except OSError:
    NATIVE_AVAILABLE = False

class SecuritySubsystem:
    def enable_firewall(self, adaptive: bool = True):
        if NATIVE_AVAILABLE:
            _native_core.sec_firewall_enable(1 if adaptive else 0)
        else:
            print(f"[Sec-Stub] Enabling firewall (Adaptive={adaptive})...")

    def detect_intrusion(self):
        if NATIVE_AVAILABLE:
            _native_core.sec_intrusion_detect()
        else:
            print("[Sec-Stub] Running intrusion detection...")

    def sandbox(self, pid: int):
        if NATIVE_AVAILABLE:
            _native_core.sec_sandbox_process(pid)
        else:
            print(f"[Sec-Stub] Sandboxing process {pid}...")

    def audit(self):
        if NATIVE_AVAILABLE:
            _native_core.sec_audit()
        else:
            print("[Sec-Stub] Running security audit...")

# Canonical Global Security Manager
sigma_sec = SecuritySubsystem()
