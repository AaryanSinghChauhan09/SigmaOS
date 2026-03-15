# Generated file: cmd_perf
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def cmd_perf(kernel: SigmaKernel, args):
    hdr('PERFORMANCE TUNING')
    perf = kernel.get_performance_tuning()
    for k, v in perf.items():
        info(f'{k} = {_ansi(C.GREEN, str(v))}')