"""
SigmaOS Sovereign v2.0 — Unified Launcher
==========================================
Usage:
  python sigma.py           — Auto-detect: GUI if tkinter available, else CLI shell
  python sigma.py --gui     — Force GUI
  python sigma.py --cli     — Force interactive CLI shell
  python sigma.py --cli boot         — CLI: run boot sequence
  python sigma.py --cli status       — CLI: system status
  python sigma.py --cli pdf doc.pdf  — CLI: PDF operation
  python sigma.py --help             — Show help
"""
import sys
import os
import argparse
import time
import io

# ── Windows: enable UTF-8 output ──────────────────────
if sys.platform == "win32":
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding="utf-8", errors="replace")

_ROOT = os.path.abspath(os.path.dirname(__file__))
for _sub in ("", "userland/system-api", "ecosystem"):
    sys.path.insert(0, os.path.join(_ROOT, _sub))

from sigma_core import SigmaKernel, SigmaConfig


def _banner():
    lines = [
        "",
        "  ╔═══════════════════════════════════════════════════╗",
        "  ║   Σ  SigmaOS Sovereign v2.0  |  Apex Release     ║",
        "  ║      Distributed Mesh  |  Cognitive Intent       ║",
        "  ╚═══════════════════════════════════════════════════╝",
        "",
    ]
    print("\n".join(lines))


def main():
    parser = argparse.ArgumentParser(
        prog="sigma",
        description="SigmaOS Sovereign v2.0 Unified Launcher",
        add_help=True,
    )
    parser.add_argument("--gui",  action="store_true", help="Force GUI mode")
    parser.add_argument("--cli",  action="store_true", help="Force CLI mode")
    parser.add_argument("--json", action="store_true", help="Output results in JSON format")
    parser.add_argument("--silent", action="store_true", help="Suppress banner and info logs")
    parser.add_argument("--intent", type=str, help="Persona intent (e.g., 'lawyer', 'security')")
    parser.add_argument("cli_args", nargs=argparse.REMAINDER,
                        help="CLI subcommands (boot, status, pdf, …)")
    args = parser.parse_args()

    # Pass silent flag to Config
    if args.json or args.silent:
        SigmaConfig.SILENT = True
    
    if not SigmaConfig.SILENT:
        _banner()
        print(f"  Loading SigmaOS Kernel...")
    t0 = time.perf_counter()
    kernel = SigmaKernel(auto_load=True)
    t1 = time.perf_counter()
    n  = len(kernel.registry.list_modules())
    if not SigmaConfig.SILENT:
        print(f"  ✓ Sovereign API online in {(t1-t0)*1000:.0f}ms | {n} services loaded")
        print(f"  ⚡ Native Kernel Linked: C/Rust/ASM Core ready. | Fabric Ready.\n")

    # Mode selection
    if args.gui:
        _launch_gui(kernel, args.intent)
    elif args.cli or args.cli_args:
        _launch_cli(kernel, args.cli_args, args.json, args.silent)
    else:
        # Auto-detect
        try:
            import tkinter
            tkinter.Tk().destroy()  # test availability
            _launch_gui(kernel, args.intent)
        except Exception:
            if not SigmaConfig.SILENT: print("  [INFO] tkinter not available — launching CLI shell.")
            _launch_cli(kernel, [], args.json, args.silent)


def _launch_gui(kernel: SigmaKernel, intent: str = None):
    try:
        from sigma_gui import launch_gui
        print(f"  Launching SigmaOS GUI Dashboard{' with intent: ' + intent if intent else ''}…\n")
        if not launch_gui(kernel, intent=intent):
            _launch_cli(kernel, [])
    except ImportError as ie:
        print(f"  GUI import error: {ie}")
        _launch_cli(kernel, [])


def _launch_cli(kernel: SigmaKernel, extra_args: list, json_mode: bool = False, silent: bool = False):
    try:
        old_argv = sys.argv
        # Reconstruct argv for sigma_cli parser
        cli_argv = ["sigma_cli"] 
        if json_mode: cli_argv.append("--json")
        if silent: cli_argv.append("--silent")
        
        # Filter out flags that were already added to cli_argv
        cleaned_extra = [a for a in extra_args if a not in ("--json", "--silent")]
        cli_argv.extend(cleaned_extra)
        
        sys.argv = cli_argv
        from sigma_cli import main as cli_main
        cli_main()
        sys.argv = old_argv
    except ImportError as ie:
        print(f"  CLI import error: {ie}")


if __name__ == "__main__":
    main()
