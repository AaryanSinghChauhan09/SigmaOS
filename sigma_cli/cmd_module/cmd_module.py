# Generated file: cmd_module
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def cmd_module(kernel: SigmaKernel, args):
    sub = args.subcommand
    if sub == 'list':
        cmd_status(kernel, args)
    elif sub == 'call':
        result = kernel.registry.call(args.module, args.method)
        hdr(f'MODULE CALL: {args.module}.{args.method}')
        if isinstance(result, dict):
            for k, v in result.items():
                info(f'{k}: {v}')
        else:
            ok(str(result))
    elif sub == 'health':
        health = kernel.registry.health_check()
        if args.json:
            print(json.dumps(health, indent=2))
            return
        hdr('MODULE HEALTH CHECK')
        for mod, status in health.items():
            ok(f'{mod}: {status}')