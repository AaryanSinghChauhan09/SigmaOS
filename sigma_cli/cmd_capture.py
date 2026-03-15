"""
Auto-split from sigma_cli.py — cmd_capture
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_capture(kernel: SigmaKernel, args):
    mode = getattr(args, 'mode', 'Standard')
    hdr(f'TITAN CAPTURE — {mode}')
    ok(kernel.capture_visual(mode))
