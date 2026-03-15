"""
Auto-split from sigma_cli.py — cmd_convert
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_convert(kernel: SigmaKernel, args):
    hdr('OMNI CONVERTER')
    cv = kernel.omni_converter
    if cv is None:
        err('OmniConverter not loaded.')
        return
    if hasattr(args, 'input') and args.input:
        ok(cv.extract_audio(args.input))
    else:
        caps = cv.get_capabilities()
        for k, v in caps.items():
            info(f'{k}: {v}')
