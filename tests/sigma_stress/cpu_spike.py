# Generated file: cpu_spike
import time
import threading
import sys
import os
from sigma_core.kernel import SigmaKernel

def cpu_spike():
    print('[STRESS] Initiating CPU Thermal Spike...')
    end = time.time() + 15
    while time.time() < end:
        _ = 2 ** 1000