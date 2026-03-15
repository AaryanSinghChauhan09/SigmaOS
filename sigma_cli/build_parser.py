"""
Auto-split from sigma_cli.py — build_parser
"""

import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig



def build_parser():
    p = argparse.ArgumentParser(prog='sigmaos', description='SigmaOS Sovereign v2.0 — Modular CLI', formatter_class=argparse.RawDescriptionHelpFormatter, epilog=textwrap.dedent('            Examples:\n              python sigma_cli.py                         # interactive shell\n              python sigma_cli.py boot                    # boot sequence\n              python sigma_cli.py status                  # system status\n              python sigma_cli.py pdf report.pdf OCR      # PDF OCR\n              python sigma_cli.py capture Panoramic       # screenshot\n              python sigma_cli.py security                # security check\n              python sigma_cli.py module list             # list modules\n              python sigma_cli.py module call browser get_browser_status\n        '))
    p.add_argument('--json', action='store_true', help='Output results in JSON format')
    p.add_argument('--silent', action='store_true', help='Suppress banner and standard info logs')
    sub = p.add_subparsers(dest='command')
    sub.add_parser('boot', help='Run full boot sequence')
    sub.add_parser('status', help='Show system status')
    pdf_p = sub.add_parser('pdf', help='PDF Forge operations')
    pdf_p.add_argument('path', help='Path to the PDF file')
    pdf_p.add_argument('action', nargs='?', default='Audit', choices=['Audit', 'OCR', 'Redact'], help='Action to perform')
    cap_p = sub.add_parser('capture', help='Titan visual capture')
    cap_p.add_argument('mode', nargs='?', default='Standard', choices=['Standard', 'OCR', 'Panoramic'])
    sub.add_parser('security', help='Security shield diagnostics')
    sub.add_parser('browser', help='Browser status')
    cv_p = sub.add_parser('convert', help='OmniConverter operations')
    cv_p.add_argument('input', nargs='?', help='Input file')
    pr_p = sub.add_parser('profile', help='Boot profile selection')
    pr_p.add_argument('profile', nargs='?', help='Profile name to activate')
    pr_p.add_argument('--context', '-c', help='User context for AI recommendation')
    sub.add_parser('perf', help='Performance tuning parameters')
    sub.add_parser('events', help='Event bus history')
    mod_p = sub.add_parser('module', help='Module management')
    mod_sub = mod_p.add_subparsers(dest='subcommand')
    mod_sub.add_parser('list', help='List loaded modules')
    mod_sub.add_parser('health', help='Health check all modules')
    call_p = mod_sub.add_parser('call', help='Call a module method')
    call_p.add_argument('module', help='Registered module key')
    call_p.add_argument('method', help='Method name')
    pkg_p = sub.add_parser('pkg', help='Sovereign Package Manager')
    pkg_sub = pkg_p.add_subparsers(dest='subcommand')
    pkg_sch = pkg_sub.add_parser('search', help='Search for packages')
    pkg_sch.add_argument('query', help='Search string')
    pkg_ins = pkg_sub.add_parser('install', help='Install a package')
    pkg_ins.add_argument('pkg_id', help='Package ID')
    pkg_upd = pkg_sub.add_parser('update', help='Update a package')
    pkg_upd.add_argument('pkg_id', help='Package ID')
    pkg_sub.add_parser('list', help='List installed packages')
    r_p = sub.add_parser('repair', help='Run deep self-healing scrubbing')
    a_p = sub.add_parser('automate', help='Launch Agentic Omni Automator')
    a_p.add_argument('goal', nargs='?', help='AI Task goal statement')
    c_p = sub.add_parser('customize', help='Deep UI Customization')
    c_p.add_argument('theme', nargs='?', help='Name of theme to apply natively')
    sub.add_parser('apex', help='Engage System Singularity (Ultimate Performance)')
    sub.add_parser('crush', help='Run Competitor Crush automation')
    ai_p = sub.add_parser('ai', help='AI/ML/DS Unified Lifecycle Engine')
    ai_sub = ai_p.add_subparsers(dest='subcommand')
    ai_st = ai_sub.add_parser('start', help='Start a new lifecycle mission')
    ai_st.add_argument('name', help='Project name')
    ai_st.add_argument('--type', choices=['AI', 'ML', 'DS'], default='ML', help='Lifecycle discipline type')
    ai_st.add_argument('objective', nargs='+', help='Project goal/objective')
    ai_next = ai_sub.add_parser('next', help='Advance to next phase')
    ai_next.add_argument('mission_id', help='ID of the active mission')
    ai_sh = ai_sub.add_parser('share', help='Share report via WhatsApp')
    ai_sh.add_argument('mission_id', help='Mission ID to share')
    return p
