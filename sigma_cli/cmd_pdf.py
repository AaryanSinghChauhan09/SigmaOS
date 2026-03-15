"""
Auto-split from sigma_cli.py — cmd_pdf
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_pdf(kernel: SigmaKernel, args):
    path = args.path
    action = getattr(args, 'action', 'Audit')
    hdr(f'PDF FORGE — {action}')
    result = kernel.process_document(path, action)
    ok(result)
