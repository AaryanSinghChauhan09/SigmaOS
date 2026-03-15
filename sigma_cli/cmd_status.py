"""
Auto-split from sigma_cli.py — cmd_status
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_status(kernel: SigmaKernel, args):
    if args.json:
        status = {'os': SigmaConfig().OS_NAME, 'version': SigmaConfig().VERSION, 'health': kernel.health_check(), 'modules': []}
        for mod in kernel.registry.list_modules():
            meta = kernel.registry.get_meta(mod)
            status['modules'].append({'name': mod, 'class': meta.get('class', '?'), 'source': meta.get('source', 'unknown')})
        print(json.dumps(status, indent=2))
        return
    hdr('SYSTEM STATUS')
    cfg = SigmaConfig()
    info(f"OS      : {_ansi(C.BOLD + C.CYAN, cfg.OS_NAME + ' v' + cfg.VERSION)}")
    info(f'Build   : {cfg.BUILD}')
    info(f'Base    : {cfg.BASE_KERNEL}')
    health = kernel.health_check()
    ok(f"Kernel Core : {health['kernel']}")
    hdr('LOADED MODULES')
    for mod in kernel.registry.list_modules():
        meta = kernel.registry.get_meta(mod)
        src = meta.get('source', 'unknown')
        cls = meta.get('class', '?')
        icon = '🟢' if src == 'kernel' else '🔵'
        print(f'  {icon}  {_ansi(C.CYAN, mod):<20} {_ansi(C.DIM, cls)}')
