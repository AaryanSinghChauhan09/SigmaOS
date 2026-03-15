# Generated file: main
import sys
import os
import argparse
import time
import io
from sigma_core import SigmaKernel, SigmaConfig

def main():
    parser = argparse.ArgumentParser(prog='sigma', description='SigmaOS Sovereign v2.0 Unified Launcher', add_help=True)
    parser.add_argument('--gui', action='store_true', help='Force GUI mode')
    parser.add_argument('--cli', action='store_true', help='Force CLI mode')
    parser.add_argument('--json', action='store_true', help='Output results in JSON format')
    parser.add_argument('--silent', action='store_true', help='Suppress banner and info logs')
    parser.add_argument('--intent', type=str, help="Persona intent (e.g., 'lawyer', 'security')")
    parser.add_argument('cli_args', nargs=argparse.REMAINDER, help='CLI subcommands (boot, status, pdf, …)')
    args = parser.parse_args()
    if args.json or args.silent:
        SigmaConfig.SILENT = True
    if not SigmaConfig.SILENT:
        _banner()
        print(f'  Loading SigmaOS Kernel...')
    t0 = time.perf_counter()
    kernel = SigmaKernel(auto_load=True)
    t1 = time.perf_counter()
    n = len(kernel.registry.list_modules())
    if not SigmaConfig.SILENT:
        print(f'  ✓ Sovereign API online in {(t1 - t0) * 1000:.0f}ms | {n} services loaded')
        print(f'  ⚡ Native Kernel Linked: C/Rust/ASM Core ready. | Fabric Ready.\n')
    if args.gui:
        _launch_gui(kernel, args.intent)
    elif args.cli or args.cli_args:
        _launch_cli(kernel, args.cli_args, args.json, args.silent)
    else:
        try:
            import tkinter
            tkinter.Tk().destroy()
            _launch_gui(kernel, args.intent)
        except Exception:
            if not SigmaConfig.SILENT:
                print('  [INFO] tkinter not available — launching CLI shell.')
            _launch_cli(kernel, [], args.json, args.silent)