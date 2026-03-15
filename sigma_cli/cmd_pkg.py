"""
Auto-split from sigma_cli.py — cmd_pkg
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_pkg(kernel: SigmaKernel, args):
    spm = kernel.registry.get('package_manager')
    if spm is None:
        err('Package Manager not loaded.')
        return
    sub = args.subcommand
    if sub == 'search':
        results = spm.search(args.query)
        if args.json:
            print(json.dumps(results, indent=2))
            return
        hdr(f'SEARCH RESULTS: {args.query}')
        for r in results:
            print(f"  [{_ansi(C.GREEN, r['state'])}] {_ansi(C.CYAN, r['id']):<20} {r['name']} v{r['ver']}")
    elif sub == 'install':
        res = spm.install(args.pkg_id)
        if args.json:
            print(json.dumps(res, indent=2))
            return
        if 'error' in res:
            err(res['error'])
        else:
            ok(res['message'])
    elif sub == 'update':
        res = spm.delta_update(args.pkg_id)
        if args.json:
            print(json.dumps(res, indent=2))
            return
        if 'error' in res:
            err(res['error'])
        else:
            ok(res['message'])
    elif sub == 'list':
        installed = spm._installed
        if args.json:
            print(json.dumps([{'id': k, 'v': v.version} for k, v in installed.items()], indent=2))
            return
        hdr('INSTALLED PACKAGES')
        for k, v in installed.items():
            print(f'  📦 {_ansi(C.CYAN, k):<20} v{v.version}')
