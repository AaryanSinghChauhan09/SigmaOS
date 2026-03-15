# Generated file: interactive_shell
import sys
import os
import argparse
import json
import textwrap
import time
from sigma_core import SigmaKernel, SigmaConfig

def interactive_shell(kernel: SigmaKernel):
    print(BANNER)
    ok(f'SigmaOS Sovereign v{kernel.version} — Interactive Shell READY')
    info("Type 'help' for commands. Type 'exit' to quit.\n")
    while True:
        try:
            raw = input(f"{_ansi(C.BOLD + C.CYAN, 'σ')} {_ansi(C.GREEN, 'SigmaOS')} {_ansi(C.DIM, '>')} ").strip()
        except (EOFError, KeyboardInterrupt):
            print('\nExiting SigmaOS shell. Stay sovereign.')
            break
        if not raw:
            continue
        parts = raw.split()
        cmd = parts[0].lower()
        if cmd in ('exit', 'quit', 'q'):
            print('Exiting SigmaOS shell. Stay sovereign.')
            break
        elif cmd == 'help':
            print(REPL_HELP)
        elif cmd == 'ghostchat':
            if len(parts) < 2:
                print('Usage: ghostchat send <text> | ghostchat status')
                continue
            sub = parts[1]
            gc = kernel.registry.get('ghostchat')
            if gc is None:
                err('GhostChat module not loaded.')
                continue
            if sub == 'send':
                txt = ' '.join(parts[2:])
                res = gc.send_message(txt)
                print(f'[GHOST] {res}')
            elif sub == 'status':
                print(gc.health_check())
        elif cmd == 'boot':

            class _A:
                pass
            cmd_boot(kernel, _A())
        elif cmd == 'ai':
            cmd_ai(kernel, parts[1:])
        elif cmd == 'status' or cmd == 'modules':

            class _A:
                pass
            cmd_status(kernel, _A())
        elif cmd == 'security':

            class _A:
                pass
            cmd_security(kernel, _A())
        elif cmd == 'browser':

            class _A:
                pass
            cmd_browser(kernel, _A())
        elif cmd == 'perf':

            class _A:
                pass
            cmd_perf(kernel, _A())
        elif cmd == 'events':

            class _A:
                pass
            cmd_events(kernel, _A())
        elif cmd == 'health':
            hdr('MODULE HEALTH CHECK')
            for mod, status in kernel.registry.health_check().items():
                ok(f'{mod}: {status}')
        elif cmd == 'pdf':
            path = parts[1] if len(parts) > 1 else 'unknown.pdf'
            action = parts[2] if len(parts) > 2 else 'Audit'

            class _A:
                pass
            a = _A()
            a.path = path
            a.action = action
            cmd_pdf(kernel, a)
        elif cmd == 'capture':
            mode = parts[1] if len(parts) > 1 else 'Standard'

            class _A:
                pass
            a = _A()
            a.mode = mode
            cmd_capture(kernel, a)
        elif cmd == 'convert':

            class _A:
                pass
            a = _A()
            a.input = parts[1] if len(parts) > 1 else None
            cmd_convert(kernel, a)
        elif cmd == 'profile':
            sel = kernel.registry.get('boot_selector')
            if sel is None:
                err('Boot Selector not loaded.')
                continue
            if len(parts) == 1:
                info('Available Profiles:')
                for p in sel.list_available_profiles():
                    print(f'    → {_ansi(C.CYAN, p)}')
            elif parts[1] == 'ai':
                ctx = ' '.join(parts[2:]) if len(parts) > 2 else ''
                rec = sel.ai_recommendation(ctx)
                ok(f'AI Recommends: {_ansi(C.GREEN + C.BOLD, rec)}')
            else:
                ok(sel.select_profile(parts[1]))
        elif cmd == 'call':
            if len(parts) < 3:
                warn('Usage: call <module_name> <method_name>')
                continue
            result = kernel.registry.call(parts[1], parts[2])
            hdr(f'RESULT: {parts[1]}.{parts[2]}')
            if isinstance(result, dict):
                for k, v in result.items():
                    info(f'{k}: {v}')
            else:
                ok(str(result))
        elif cmd == 'automate':

            class _A:
                pass
            a = _A()
            a.goal = ' '.join(parts[1:]) if len(parts) > 1 else None
            cmd_automate(kernel, a)
        elif cmd == 'customize':

            class _A:
                pass
            a = _A()
            a.theme = parts[1] if len(parts) > 1 else None
            cmd_customize(kernel, a)
        elif cmd == 'repair':

            class _A:
                pass
            cmd_repair(kernel, _A())
        elif cmd == 'apex':

            class _A:
                pass
            cmd_apex(kernel, _A())
        elif cmd == 'crush':

            class _A:
                pass
            cmd_crush(kernel, _A())
        else:
            warn(f"Unknown command: '{cmd}'. Type 'help' for usage.")