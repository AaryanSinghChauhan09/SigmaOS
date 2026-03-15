"""
Auto-split from sigma_cli.py — cmd_automate
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def cmd_automate(kernel: SigmaKernel, args):
    hdr('OMNI AUTOMATOR STUDIO')
    auto = kernel.registry.get('automator')
    if auto is None:
        err('Automator module offline.')
        return
    if hasattr(args, 'goal') and args.goal:
        info(f'Dispatching AI pipeline for goal: {args.goal}')
        ok(auto.launch_agentic_pipeline(args.goal))
    else:
        ok(auto.health_check())
