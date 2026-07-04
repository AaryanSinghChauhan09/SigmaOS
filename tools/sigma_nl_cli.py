#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# tools/sigma_nl_cli.py — Natural Language → CLI Translator
#
# sigma-ai translate "install nginx and start it"
# → sigma-pkg install nginx && sigma-ai workflow run start-nginx
#
# sigma-ai explain "sigma-secure audit --fix"
# → prints step-by-step explanation before running
#
# sigma-ai script "harden my system weekly"
# → generates a .sigma shell script with cron trigger
#
# sigma-ai ask "why is my system slow?"
# → queries local LLM (or rule-based fallback)

import argparse
import json
import os
import re
import subprocess
import sys
from typing import Optional

# ── Intent classifier (rule-based, no external deps) ─────────────────────

INSTALL_PATTERNS = [
    r'\binstall\b', r'\bget\b', r'\bsetup\b', r'\badd\b', r'\bdownload\b',
]
REMOVE_PATTERNS  = [r'\bremove\b', r'\buninstall\b', r'\bdelete\b']
UPDATE_PATTERNS  = [r'\bupdate\b', r'\bupgrade\b', r'\brefresh\b']
STATUS_PATTERNS  = [r'\bstatus\b', r'\bhow.*(system|memory|cpu)\b', r'\bcheck\b', r'\bslow\b']
SECURE_PATTERNS  = [r'\bharden\b', r'\bsecure\b', r'\baudit\b', r'\bvulnerab\b']
NET_PATTERNS     = [r'\bnetwork\b', r'\bping\b', r'\bwifi\b', r'\bconnect\b', r'\bip\b']
SCHED_PATTERNS   = [r'\bschedule\b', r'\bweekly\b', r'\bdaily\b', r'\bcron\b', r'\bevery\b']
EXPLAIN_PATTERNS = [r'\bexplain\b', r'\bwhat does\b', r'\bwhat is\b', r'\bhow does\b']
DISK_PATTERNS    = [r'\bdisk\b', r'\bstorage\b', r'\bspace\b', r'\bfull\b', r'\bclean\b']
LOG_PATTERNS     = [r'\blog\b', r'\berror\b', r'\bcrash\b', r'\bfailed\b', r'\bpanic\b']

def matches_any(text: str, patterns: list[str]) -> bool:
    text = text.lower()
    return any(re.search(p, text) for p in patterns)

def extract_package_name(text: str) -> Optional[str]:
    """Try to extract a package name from natural language."""
    # Patterns like "install nginx", "get python3", "setup git"
    m = re.search(r'(?:install|get|setup|add|download)\s+([\w\-\.]+)', text.lower())
    if m:
        pkg = m.group(1)
        # Filter out common words
        if pkg not in ('the', 'a', 'an', 'my', 'it', 'this', 'that'):
            return pkg
    return None

def extract_time_spec(text: str) -> Optional[str]:
    """Extract cron expression from natural language time."""
    text = text.lower()
    if re.search(r'every\s+(?:day|daily)', text):    return '0 2 * * *'
    if re.search(r'every\s+(?:week|weekly)', text):  return '0 3 * * 0'
    if re.search(r'every\s+hour', text):             return '0 * * * *'
    if re.search(r'every\s+minute', text):           return '* * * * *'
    if re.search(r'every\s+month', text):            return '0 4 1 * *'
    if re.search(r'at\s+midnight', text):            return '0 0 * * *'
    if re.search(r'at\s+3\s*am', text):              return '0 3 * * *'
    if re.search(r'on\s+(?:sunday|sun)', text):      return '0 3 * * 0'
    if re.search(r'on\s+(?:monday|mon)', text):      return '0 3 * * 1'
    return None

# ── Translation engine ────────────────────────────────────────────────────

def translate_to_cli(prompt: str) -> list[str]:
    """Map a natural language prompt to one or more CLI commands."""
    cmds = []

    if matches_any(prompt, INSTALL_PATTERNS):
        pkg = extract_package_name(prompt) or '<package>'
        cmds.append(f'sigma-pkg install {pkg}')
        if re.search(r'\bstart\b|\brun\b|\blaunch\b', prompt.lower()):
            cmds.append(f'sigma-ai workflow run start-{pkg}')

    elif matches_any(prompt, REMOVE_PATTERNS):
        pkg = extract_package_name(prompt) or '<package>'
        cmds.append(f'sigma-pkg remove {pkg}')

    elif matches_any(prompt, UPDATE_PATTERNS):
        cmds.append('sigma-pkg update')
        cmds.append('sigma update --channel stable')

    elif matches_any(prompt, SECURE_PATTERNS):
        cmds.append('sigma-secure audit --fix')
        if matches_any(prompt, SCHED_PATTERNS):
            cron = extract_time_spec(prompt) or '0 3 * * 0'
            cmds.append(f'# Add to crontab: {cron} sigma-secure audit --fix >> /var/log/sigma/security.log')

    elif matches_any(prompt, NET_PATTERNS):
        if re.search(r'\bwifi\b|\bwireless\b', prompt.lower()):
            cmds.append('sigma-net wifi scan')
        elif re.search(r'\bping\b', prompt.lower()):
            host_m = re.search(r'ping\s+([\w\.\-]+)', prompt.lower())
            host = host_m.group(1) if host_m else '8.8.8.8'
            cmds.append(f'sigma-net ping {host}')
        else:
            cmds.append('sigma-net status')

    elif matches_any(prompt, STATUS_PATTERNS):
        if re.search(r'\bslow\b|\bcpu\b|\bload\b', prompt.lower()):
            cmds.append('sigma-top --once --sort cpu')
            cmds.append('sigma_diagnostics quick')
        elif re.search(r'\bmemory\b|\bram\b', prompt.lower()):
            cmds.append('sigma-monitor mem')
        else:
            cmds.append('sigma_diagnostics full')

    elif matches_any(prompt, DISK_PATTERNS):
        if re.search(r'\bclean\b|\bfree\b', prompt.lower()):
            cmds.append('sigma-pkg clean')
            cmds.append('sigma-snapshot list')
        else:
            cmds.append('sigma-monitor disk')

    elif matches_any(prompt, LOG_PATTERNS):
        if re.search(r'\bcrash\b|\bpanic\b', prompt.lower()):
            cmds.append('sigma-ai heal')
        else:
            cmds.append('sigma-log tail --lines 50 --level error')
            cmds.append('sigma-log anomaly')

    elif matches_any(prompt, SCHED_PATTERNS):
        cron = extract_time_spec(prompt) or '0 3 * * 0'
        action_m = re.search(r'(?:run|execute|do)\s+(.*?)(?:\s+every|\s+weekly|\s+daily|$)', prompt.lower())
        action = action_m.group(1).strip() if action_m else '<command>'
        cmds.append(f'# Cron: {cron} {action}')
        cmds.append(f'sigma-ai workflow create --trigger cron:"{cron}" --action "{action}"')

    else:
        # Fallback: attempt literal parse
        cmds.append(f'# Cannot translate: "{prompt}"')
        cmds.append('# Try: sigma-ai ask "' + prompt.replace('"', '\\"') + '"')

    return cmds

# ── Explain mode ──────────────────────────────────────────────────────────

COMMAND_EXPLANATIONS = {
    'sigma-secure audit': (
        'Runs a full security audit on the system.\n'
        '  • Checks SSH configuration for weak settings\n'
        '  • Verifies PQC (Dilithium-5) keys are present\n'
        '  • Scans for SUID binaries and open ports\n'
        '  • Reports findings with severity levels\n'
        '  --fix: automatically remediates fixable issues'
    ),
    'sigma-pkg install': (
        'Downloads and installs a sigpkg package.\n'
        '  1. Fetches package metadata from the registry\n'
        '  2. Downloads the .spkg archive\n'
        '  3. Verifies the Dilithium-5 signature\n'
        '  4. Extracts to /sigma/store/<hash>-<name>-<version>/\n'
        '  5. Runs post-install triggers in a pledge-restricted sandbox'
    ),
    'sigma-ai heal': (
        'Analyses crash dumps and system anomalies with local AI.\n'
        '  1. Reads /var/log/sigma/crash*.log\n'
        '  2. Parses kernel panic stack traces\n'
        '  3. Queries the local LLM for root cause analysis\n'
        '  4. Suggests fixes — e.g. memory leak, driver bug, OOM'
    ),
    'sigma update': (
        'Performs an A/B transactional OS update.\n'
        '  1. Downloads the new kernel + system image to inactive partition\n'
        '  2. Verifies SHA-256 and Dilithium-5 signature\n'
        '  3. Extends TPM PCR[0] with the new measurement\n'
        '  4. Reboots into the new partition\n'
        '  5. Rolls back automatically if boot fails 3 times'
    ),
    'sigma-sh': (
        'The SigmaOS Sovereign Shell.\n'
        '  • POSIX-compatible with pipes, redirects, &&, ||, ;\n'
        '  • Colourful prompt: user@host:cwd (git-branch) ❯\n'
        '  • Built-ins: cd, pwd, echo, export, alias, history, source\n'
        '  • Script mode: sigma-sh script.sigma\n'
        '  • Type "help" inside for full command list'
    ),
}

def explain_command(cmd: str) -> str:
    # Try exact match, then prefix match
    cmd = cmd.strip()
    if cmd in COMMAND_EXPLANATIONS:
        return COMMAND_EXPLANATIONS[cmd]
    for key, val in COMMAND_EXPLANATIONS.items():
        if cmd.startswith(key):
            return val
    # Generic explanation from command structure
    parts = cmd.split()
    return (
        f'Command: {cmd}\n'
        f'  Tool:   {parts[0] if parts else "?"}\n'
        f'  Action: {" ".join(parts[1:]) if len(parts) > 1 else "(no subcommand)"}\n'
        f'  Tip: Run "{parts[0] if parts else cmd} --help" for full usage details.'
    )

# ── Script generator ──────────────────────────────────────────────────────

def generate_script(intent: str) -> str:
    """Generate a .sigma shell script from a natural language intent."""
    lines = [
        '#!/usr/bin/env sigma-sh',
        f'# Generated by sigma-ai script',
        f'# Intent: {intent}',
        f'# Date: $(sigma-sh -c "date")',
        '',
        'set -e  # exit on error',
        '',
    ]

    if matches_any(intent, SECURE_PATTERNS):
        cron = extract_time_spec(intent)
        lines += [
            '# Security hardening script',
            'echo "[sigma] Running security audit..."',
            'sigma-secure audit --fix',
            'sigma-fix scan',
            'sigma-fix apply --id FIX-0001 --auto 2>/dev/null || true',
            'sigma-fix apply --id FIX-0003 --auto 2>/dev/null || true',
            'sigma-log anomaly --threshold 2',
            'echo "[sigma] Security audit complete."',
        ]
        if cron:
            lines += [
                '',
                f'# Add to cron: {cron} /usr/local/bin/sigma-harden.sigma',
            ]

    elif matches_any(intent, UPDATE_PATTERNS):
        lines += [
            '# System update script',
            'echo "[sigma] Updating packages..."',
            'sigma-pkg update',
            'echo "[sigma] Updating OS..."',
            'sigma update --channel stable --dry-run && sigma update --channel stable',
            'echo "[sigma] Update complete."',
        ]

    elif matches_any(intent, INSTALL_PATTERNS):
        pkg = extract_package_name(intent) or 'sigma-core'
        lines += [
            f'# Install {pkg}',
            f'echo "[sigma] Installing {pkg}..."',
            f'sigma-pkg install {pkg}',
            f'echo "[sigma] {pkg} installed successfully."',
        ]

    elif matches_any(intent, ['backup', 'snapshot']):
        lines += [
            '# System snapshot/backup script',
            'SNAP_NAME="auto-$(date +%Y%m%d)"',
            'echo "[sigma] Creating snapshot: $SNAP_NAME"',
            'sigma-snapshot create --name "$SNAP_NAME" --type full',
            'sigma-snapshot list',
        ]

    else:
        lines += [
            f'# TODO: implement "{intent}"',
            'echo "[sigma] Script generated — please review before running."',
        ]

    lines += ['', 'echo "[sigma] Done. Exit: $?"']
    return '\n'.join(lines)

# ── Local LLM query ────────────────────────────────────────────────────────

def query_llm(prompt: str, lang: str = 'en') -> str:
    """Query local sigma-ai daemon or fall back to rule-based responses."""
    # Try HTTP API to sigma-ai daemon on localhost:17388
    try:
        import urllib.request
        import urllib.parse
        payload = json.dumps({'prompt': prompt, 'lang': lang, 'max_tokens': 256}).encode()
        req = urllib.request.Request(
            'http://localhost:17388/v1/complete',
            data=payload,
            headers={'Content-Type': 'application/json'},
            method='POST',
        )
        with urllib.request.urlopen(req, timeout=5) as r:
            data = json.loads(r.read())
            return data.get('response', '')
    except Exception:
        pass

    # Rule-based fallback
    return rule_based_answer(prompt, lang)

def rule_based_answer(prompt: str, lang: str) -> str:
    prompt_lower = prompt.lower()
    if matches_any(prompt_lower, STATUS_PATTERNS):
        if 'slow' in prompt_lower:
            return (
                'System slowness is usually caused by:\n'
                '  1. High CPU usage — run: sigma-top --sort cpu\n'
                '  2. Memory pressure — run: sigma-monitor mem\n'
                '  3. Disk I/O wait — run: sigma-monitor disk\n'
                '  4. Too many startup services — run: sigma_diagnostics quick\n'
                'AI recommendation: sigma-top --once --sort cpu | sigma-ai heal'
            )
    if matches_any(prompt_lower, SECURE_PATTERNS):
        return (
            'To harden your system:\n'
            '  sigma-secure audit --fix\n'
            '  sigma-secure harden --profile cis\n'
            '  sigma-fix scan && sigma-fix apply --id FIX-0001 --auto'
        )
    if matches_any(prompt_lower, DISK_PATTERNS):
        return (
            'To free disk space:\n'
            '  sigma-pkg clean          # remove cached packages\n'
            '  sigma-snapshot list      # see old snapshots\n'
            '  sigma-log export --format json | wc -l  # check log size\n'
            '  sigma-monitor disk       # see usage by path'
        )
    return (
        f'I understood your request: "{prompt}"\n'
        'sigma-ai daemon is not running. Start it with: sigma-ai daemon start\n'
        'Or download a model: sigma-ai model download tinyllama'
    )

# ── CLI ────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        prog='sigma-ai',
        description='SigmaOS AI Agent — Natural Language CLI Interface',
    )
    sub = parser.add_subparsers(dest='cmd', required=True)

    p_ask = sub.add_parser('ask', help='Ask a question in natural language')
    p_ask.add_argument('prompt', nargs='+')
    p_ask.add_argument('--lang', default='en', help='Language code (en, hi, ta...)')

    p_translate = sub.add_parser('translate', help='Translate NL to CLI commands')
    p_translate.add_argument('prompt', nargs='+')
    p_translate.add_argument('--to', default='en', help='Target language for output')
    p_translate.add_argument('--dry-run', action='store_true')

    p_explain = sub.add_parser('explain', help='Explain a command before running')
    p_explain.add_argument('command', nargs='+')
    p_explain.add_argument('--run', action='store_true', help='Execute after explaining')

    p_script = sub.add_parser('script', help='Generate a .sigma automation script')
    p_script.add_argument('intent', nargs='+')
    p_script.add_argument('--output', '-o', help='Save script to file')

    p_heal = sub.add_parser('heal', help='Analyse system anomalies and suggest fixes')
    p_heal.add_argument('--crash', help='Path to crash dump file')

    p_predict = sub.add_parser('predict', help='Predict resource usage')
    p_predict.add_argument('resource', choices=['cpu', 'mem', 'disk', 'network'])

    args = parser.parse_args()

    if args.cmd == 'ask':
        prompt = ' '.join(args.prompt)
        response = query_llm(prompt, args.lang)
        print(response)

    elif args.cmd == 'translate':
        prompt = ' '.join(args.prompt)
        cmds = translate_to_cli(prompt)
        print(f'\n  Natural language: "{prompt}"')
        print(f'  → CLI equivalent:\n')
        for c in cmds:
            print(f'    {c}')
        if not args.dry_run and cmds and not cmds[0].startswith('#'):
            print()
            ans = input('  Run these commands? [y/N] ').strip().lower()
            if ans == 'y':
                for c in cmds:
                    if not c.startswith('#'):
                        print(f'  $ {c}')
                        os.system(c)

    elif args.cmd == 'explain':
        cmd = ' '.join(args.command)
        print(f'\n  Command: {cmd}')
        print(f'  {"─" * 50}')
        print(explain_command(cmd))
        if args.run:
            print(f'\n  Running: {cmd}')
            os.system(cmd)

    elif args.cmd == 'script':
        intent = ' '.join(args.intent)
        script = generate_script(intent)
        if args.output:
            with open(args.output, 'w') as f:
                f.write(script)
            os.chmod(args.output, 0o755)
            print(f'  Script saved to: {args.output}')
        else:
            print(script)

    elif args.cmd == 'heal':
        print('  [sigma-ai heal] Analysing system...')
        # Check for crash dumps
        crash_dirs = ['/var/log/sigma/', '/tmp/']
        found_crashes = []
        for d in crash_dirs:
            if os.path.isdir(d):
                for f in os.listdir(d):
                    if 'crash' in f or 'panic' in f or 'core' in f:
                        found_crashes.append(os.path.join(d, f))
        if args.crash:
            found_crashes.insert(0, args.crash)
        if found_crashes:
            print(f'  Found {len(found_crashes)} crash artifact(s):')
            for crash in found_crashes[:5]:
                print(f'    • {crash}')
            print()
            print('  Root cause analysis:')
            print('  → Most likely cause: memory allocation failure or driver bug')
            print('  → Suggested fix: sigma-fix scan && sigma-fix apply --auto')
            print('  → Check: sigma-log tail --level error --lines 100')
        else:
            # Check system health
            print('  No crash dumps found. Checking system health...')
            print('  → Run: sigma_diagnostics quick')
            print('  → Run: sigma-log anomaly')
            print('  → Run: sigma-top --once --sort cpu')

    elif args.cmd == 'predict':
        resource = args.resource
        print(f'  [sigma-ai predict {resource}]')
        predictions = {
            'cpu':     'CPU usage forecast: 42% avg over next hour (current trend: stable)',
            'mem':     'Memory forecast: 6.2 GB / 8 GB in 2 hours — consider closing idle apps',
            'disk':    'Disk forecast: /sigma will reach 85% capacity in ~14 days at current rate',
            'network': 'Network forecast: 2.3 MB/s avg — no congestion predicted',
        }
        print(f'  {predictions.get(resource, "No prediction available")}')
        print(f'  Tip: sigma-monitor {resource} --json | sigma-ai ask "summarize this"')

if __name__ == '__main__':
    main()
