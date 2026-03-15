# Generated file: _section
import sys
import os
import time
from sigma_core import SigmaKernel

def _section(title: str, num: int, total: int):
    print(f'\n\x1b[96m[{num}/{total}]\x1b[0m {title}...')