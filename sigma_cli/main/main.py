# Generated file: main
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def main():
    parser = build_parser()
    args = parser.parse_args()
    if args.json or args.silent:
        SigmaConfig.SILENT = True
    if not SigmaConfig.SILENT:
        print(BANNER)
        info(f'Loading SigmaOS Sovereign v{SigmaConfig().VERSION}...')
    t0 = time.perf_counter()
    kernel = SigmaKernel(auto_load=True)
    t1 = time.perf_counter()
    if not args.silent and (not args.json):
        ok(f'Kernel loaded in {(t1 - t0) * 1000:.0f}ms  |  {len(kernel.registry.list_modules())} modules online\n')
    cmd = args.command
    if cmd == 'boot':
        cmd_boot(kernel, args)
    elif cmd == 'status':
        cmd_status(kernel, args)
    elif cmd == 'pdf':
        cmd_pdf(kernel, args)
    elif cmd == 'capture':
        cmd_capture(kernel, args)
    elif cmd == 'security':
        cmd_security(kernel, args)
    elif cmd == 'browser':
        cmd_browser(kernel, args)
    elif cmd == 'convert':
        cmd_convert(kernel, args)
    elif cmd == 'profile':
        cmd_boot_profile(kernel, args)
    elif cmd == 'perf':
        cmd_perf(kernel, args)
    elif cmd == 'events':
        cmd_events(kernel, args)
    elif cmd == 'module':
        cmd_module(kernel, args)
    elif cmd == 'pkg':
        cmd_pkg(kernel, args)
    elif cmd == 'repair':
        cmd_repair(kernel, args)
    elif cmd == 'automate':
        cmd_automate(kernel, args)
    elif cmd == 'customize':
        cmd_customize(kernel, args)
    elif cmd == 'ai':
        cmd_ai(kernel, args)
    elif cmd == 'apex':
        cmd_apex(kernel, args)
    elif cmd == 'crush':
        cmd_crush(kernel, args)
    else:
        interactive_shell(kernel)