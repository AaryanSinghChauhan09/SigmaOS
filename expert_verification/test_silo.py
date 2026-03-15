# Generated file: test_silo
import sys
import os
import time
from sigma_core.kernel import SigmaKernel

def test_silo(name, check_fn):
    print(f'[*] Testing {name:25}...', end='', flush=True)
    try:
        result = check_fn()
        print(f' [✅ PASS] -> {result}')
        return True
    except Exception as e:
        print(f' [❌ FAIL] -> {e}')
        return False