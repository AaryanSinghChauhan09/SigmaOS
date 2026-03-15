# Generated file: ram_bloat
import time
import threading
import sys
import os
from sigma_core.kernel import SigmaKernel

def ram_bloat(kernel):
    print('[STRESS] Bloating Process Table...')
    for i in range(50):
        kernel.process.spawn(f'zombie_shard_{i}', cgroup='system.slice')
    print('[STRESS] 50 Shards injected.')