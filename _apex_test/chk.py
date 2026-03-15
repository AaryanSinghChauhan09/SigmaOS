# Generated file: chk
import sys
from sigma_core import SigmaKernel

def chk(key, obj, method='health_check'):
    try:
        val = getattr(obj, method)() if obj else 'NOT LOADED'
        results[key] = val
    except Exception as e:
        results[key] = f'ERROR: {e}'