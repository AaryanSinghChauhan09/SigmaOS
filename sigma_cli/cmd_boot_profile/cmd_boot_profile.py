# Generated file: cmd_boot_profile
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def cmd_boot_profile(kernel: SigmaKernel, args):
    hdr('BOOT PROFILE SELECTOR')
    sel = kernel.registry.get('boot_selector')
    if sel is None:
        err('Boot Selector not loaded.')
        return
    profiles = sel.list_available_profiles()
    info('Available Profiles:')
    for p in profiles:
        print(f'  → {_ansi(C.CYAN, p)}')
    if hasattr(args, 'profile') and args.profile:
        ok(sel.select_profile(args.profile))
    elif hasattr(args, 'context') and args.context:
        rec = sel.ai_recommendation(args.context)
        ok(f'AI Recommends: {_ansi(C.GREEN + C.BOLD, rec)}')