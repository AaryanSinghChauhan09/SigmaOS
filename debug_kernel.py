import sys
import os

_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__)))
sys.path.insert(0, os.path.join(_ROOT, "userland/system_api"))

from sigma_core.kernel import SigmaKernel

try:
    k = SigmaKernel(auto_load=True)
    print("Kernel Initialized.")
except Exception as e:
    print(f"FAILED: {e}")
