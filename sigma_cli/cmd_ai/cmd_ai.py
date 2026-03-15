# Generated file: cmd_ai
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def cmd_ai(kernel: SigmaKernel, args):
    if not hasattr(kernel, 'ai_lifecycle'):
        err('AI/ML/DS Lifecycle module not loaded.')
        return
    sub = args.subcommand
    if not sub:
        print(f'Usage: {C.CYAN}ai start <name> <type> <objective...>{C.RESET}')
        return
    if sub == 'start':
        name, m_type = (args.name, args.type)
        obj = ' '.join(args.objective)
        mid = kernel.ai_lifecycle.start_unified_mission(name, obj, m_type)
        ok(f'AI/ML/DS Lifecycle Started. Mission ID: {C.BOLD}{mid}{C.RESET}')
        info(f'Discipline: {C.MAGENTA}{m_type.upper()}{C.RESET} | Phase: {C.BOLD}PROBLEM_DEFINITION{C.RESET}')
    elif sub == 'next':
        mid = args.mission_id
        res = kernel.ai_lifecycle.execute_next_step(mid)
        if 'error' in res:
            err(res['error'])
        else:
            hdr(f"LIFECYCLE STEP: {res['step']}")
            print(f"  {C.MAGENTA}»{C.RESET} {res['guidance']}")
            if 'metrics' in res:
                info(f"Metrics: {res['metrics']}")
    elif sub == 'share':
        mid = args.mission_id
        res = kernel.ai_lifecycle.share_report_wa(mid)
        ok(res if isinstance(res, str) else res.get('message', 'Sent.'))