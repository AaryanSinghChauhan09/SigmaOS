"""
Auto-split from sigma_cli.py — cmd_repair
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_repair(kernel: SigmaKernel, args):
    hdr('SELF-HEALING RECOVERY (APEX)')
    sr = kernel.registry.get('self_repair')
    if sr:
        info('Initializing Advanced Merkle-Tree Matrix...')
        res = sr.trigger_mesh_resilver()
        ok(res)
        ok(sr.health_check())
    else:
        ok(kernel.self_healing_recovery())
        ok('Kernel integrity verified (Legacy Fallback).')
