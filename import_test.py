import sys
import os
sys.path.insert(0, os.path.abspath("."))
try:
    from sigma_core.security.integrity import IntegrityGuard
    print("SUCCESS: IntegrityGuard imported.")
except Exception as e:
    print(f"FAILED: {e}")
    import traceback
    traceback.print_exc()
