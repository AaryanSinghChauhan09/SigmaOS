# Generated file: stress_fs
import time
import sys
import os
import threading
import random
from sigma_core.kernel import SigmaKernel

def stress_fs(kernel, file_count=50):
    print(f'  [FS] Concurrent I/O Shredding on {file_count} nodes...')
    fs = kernel.registry.get('sigma_fs')
    if fs:

        def _shred(idx):
            path = f'/vault/shred_{idx}.tmp'
            fs.create(path, b'SHRED' * 10000)
            fs.delete(path)
        threads = []
        for i in range(file_count):
            t = threading.Thread(target=_shred, args=(i,))
            t.start()
            threads.append(t)
        for t in threads:
            t.join()
        print('  ✔ FS Shredding Complete.')