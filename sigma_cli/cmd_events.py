"""
Auto-split from sigma_cli.py — cmd_events
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_events(kernel: SigmaKernel, args):
    hdr('EVENT BUS HISTORY')
    history = kernel.bus.get_history(20)
    if not history:
        warn('No events recorded yet.')
        return
    for e in history:
        info(f"{_ansi(C.CYAN, e['event'])}: {e['payload']}")
