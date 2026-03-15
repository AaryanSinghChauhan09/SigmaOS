"""
Auto-split from sigma_cli.py — cmd_browser
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_browser(kernel: SigmaKernel, args):
    hdr('SIGMA OMNI BROWSER')
    br = kernel.browser
    if br is None:
        err('Browser module not loaded.')
        return
    status = br.get_browser_status()
    for k, v in status.items():
        info(f'{k}: {_ansi(C.CYAN, str(v))}')
