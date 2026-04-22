# sigma_test_all.py
# SigmaOS: Sovereign Test Orchestrator

import os
import subprocess

def run_atomic_tests():
    print("Σ [TEST]: Initiating Sovereign Atomic Test Suite...")
    
    # 1. Compile test shards
    test_files = [f for f in os.listdir('.') if f.startswith('test_') and f.endswith('.c')]
    
    for test in test_files:
        binary = test.replace('.c', '.bin')
        print(f"  Σ [RUN]: Testing {test}...")
        
        # Compile against atomic LibC shards
        subprocess.run(["gcc", "-nostdlib", test, "sigma_atoi_fixed.c", "sigma_print.c", "sigma_write.asm", "-o", binary])
        
        # Execute (simulated)
        res = subprocess.run([f"./{binary}"])
        if res.returncode == 0:
            print(f"  Σ [PASS]: {test} certified.")
        else:
            print(f"  Σ [FAIL]: {test} logic violation detected.")

if __name__ == "__main__":
    run_atomic_tests()
