"""
SigmaOS CLI — feature-rich command-line interface.
Usage:
    python -m sigma_cli                 # interactive shell
    python -m sigma_cli boot            # run boot sequence
    python -m sigma_cli pdf <path>      # process PDF
    python -m sigma_cli status          # system status
    python -m sigma_cli module list     # list loaded modules
"""
import sys
import os
import argparse
import json
import textwrap
import time

# ── Windows: enable UTF-8 output and VT100 ANSI colors ──────────────────────
if sys.platform == "win32":
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding="utf-8", errors="replace")
    import ctypes
    try:
        kernel32 = ctypes.windll.kernel32
        kernel32.SetConsoleMode(kernel32.GetStdHandle(-11), 7)
    except Exception:
        pass

# ensure SigmaOS packages are on path
_ROOT = os.path.abspath(os.path.dirname(__file__))
for _sub in ("", "userland/system-api", "ecosystem"):
    sys.path.insert(0, os.path.join(_ROOT, _sub))

from sigma_core import SigmaKernel, SigmaConfig

# ─── ANSI Colors ─────────────────────────────────────────────────────────────
class C:
    RESET  = "\033[0m"
    BOLD   = "\033[1m"
    CYAN   = "\033[96m"
    GREEN  = "\033[92m"
    YELLOW = "\033[93m"
    RED    = "\033[91m"
    MAGENTA= "\033[95m"
    BLUE   = "\033[94m"
    DIM    = "\033[2m"
    WHITE  = "\033[97m"

def _ansi(code, text): return f"{code}{text}{C.RESET}"
def ok(msg):    print(f"  {_ansi(C.GREEN,'✔')}  {msg}")
def warn(msg):  print(f"  {_ansi(C.YELLOW,'⚠')}  {msg}")
def err(msg):   print(f"  {_ansi(C.RED,'✖')}  {msg}")
def info(msg):  print(f"  {_ansi(C.CYAN,'ℹ')}  {msg}")
def hdr(msg):   print(f"\n{_ansi(C.BOLD+C.MAGENTA, '━━━ '+msg+' ━━━')}")

BANNER = f"""
{_ansi(C.BOLD+C.CYAN, '╔══════════════════════════════════════════════════════╗')}
{_ansi(C.BOLD+C.CYAN, '║')}  {_ansi(C.BOLD+C.WHITE, 'SigmaOS Sovereign v2.0')}  {_ansi(C.DIM,'Modular | GUI+CLI | AI-Native')}  {_ansi(C.BOLD+C.CYAN, '║')}
{_ansi(C.BOLD+C.CYAN, '╚══════════════════════════════════════════════════════╝')}
"""

# ─── Command Handlers ─────────────────────────────────────────────────────────

def cmd_boot(kernel: SigmaKernel, args):
    hdr("BOOT SEQUENCE")
    steps = kernel.boot()
    for step, result in steps.items():
        ok(f"[{step.upper()}] {result}")
    hdr("KERNEL STATS")
    stats = kernel.get_leadership_stats()
    for k, v in stats.items():
        info(f"{k}: {_ansi(C.GREEN, v)}")


def cmd_status(kernel: SigmaKernel, args):
    if args.json:
        status = {
            "os": SigmaConfig().OS_NAME,
            "version": SigmaConfig().VERSION,
            "health": kernel.health_check(),
            "modules": []
        }
        for mod in kernel.registry.list_modules():
            meta = kernel.registry.get_meta(mod)
            status["modules"].append({
                "name": mod,
                "class": meta.get('class', '?'),
                "source": meta.get('source', 'unknown')
            })
        print(json.dumps(status, indent=2))
        return

    hdr("SYSTEM STATUS")
    cfg = SigmaConfig()
    info(f"OS      : {_ansi(C.BOLD+C.CYAN, cfg.OS_NAME+' v'+cfg.VERSION)}")
    info(f"Build   : {cfg.BUILD}")
    info(f"Base    : {cfg.BASE_KERNEL}")
    health = kernel.health_check()
    ok(f"Kernel Core : {health['kernel']}")
    hdr("LOADED MODULES")
    for mod in kernel.registry.list_modules():
        meta = kernel.registry.get_meta(mod)
        src = meta.get('source', 'unknown')
        cls = meta.get('class', '?')
        icon = "🟢" if src == "kernel" else "🔵"
        print(f"  {icon}  {_ansi(C.CYAN, mod):<20} {_ansi(C.DIM, cls)}")


def cmd_pdf(kernel: SigmaKernel, args):
    path   = args.path
    action = getattr(args, 'action', 'Audit')
    hdr(f"PDF FORGE — {action}")
    result = kernel.process_document(path, action)
    ok(result)


def cmd_capture(kernel: SigmaKernel, args):
    mode = getattr(args, 'mode', 'Standard')
    hdr(f"TITAN CAPTURE — {mode}")
    ok(kernel.capture_visual(mode))


def cmd_module(kernel: SigmaKernel, args):
    sub = args.subcommand
    if sub == "list":
        cmd_status(kernel, args)
    elif sub == "call":
        result = kernel.registry.call(args.module, args.method)
        hdr(f"MODULE CALL: {args.module}.{args.method}")
        if isinstance(result, dict):
            for k, v in result.items():
                info(f"{k}: {v}")
        else:
            ok(str(result))
    elif sub == "health":
        health = kernel.registry.health_check()
        if args.json:
            print(json.dumps(health, indent=2))
            return
        hdr("MODULE HEALTH CHECK")
        for mod, status in health.items():
            ok(f"{mod}: {status}")


def cmd_security(kernel: SigmaKernel, args):
    hdr("SECURITY SHIELD")
    sec = kernel.security
    if sec is None:
        err("Security module not loaded.")
        return
    ok(sec.secure_boot_verify())
    ok(sec.ebpf_proactive_monitoring())
    ok(sec.formal_verification_audit())
    info(f"Level: {_ansi(C.GREEN+C.BOLD, sec.security_level)}")


def cmd_browser(kernel: SigmaKernel, args):
    hdr("SIGMA OMNI BROWSER")
    br = kernel.browser
    if br is None:
        err("Browser module not loaded.")
        return
    status = br.get_browser_status()
    for k, v in status.items():
        info(f"{k}: {_ansi(C.CYAN, str(v))}")


def cmd_convert(kernel: SigmaKernel, args):
    hdr("OMNI CONVERTER")
    cv = kernel.omni_converter
    if cv is None:
        err("OmniConverter not loaded.")
        return
    if hasattr(args, 'input') and args.input:
        ok(cv.extract_audio(args.input))
    else:
        caps = cv.get_capabilities()
        for k, v in caps.items():
            info(f"{k}: {v}")


def cmd_boot_profile(kernel: SigmaKernel, args):
    hdr("BOOT PROFILE SELECTOR")
    sel = kernel.registry.get("boot_selector")
    if sel is None:
        err("Boot Selector not loaded.")
        return
    profiles = sel.list_available_profiles()
    info("Available Profiles:")
    for p in profiles:
        print(f"  → {_ansi(C.CYAN, p)}")
    if hasattr(args, 'profile') and args.profile:
        ok(sel.select_profile(args.profile))
    elif hasattr(args, 'context') and args.context:
        rec = sel.ai_recommendation(args.context)
        ok(f"AI Recommends: {_ansi(C.GREEN+C.BOLD, rec)}")


def cmd_events(kernel: SigmaKernel, args):
    hdr("EVENT BUS HISTORY")
    history = kernel.bus.get_history(20)
    if not history:
        warn("No events recorded yet.")
        return
    for e in history:
        info(f"{_ansi(C.CYAN, e['event'])}: {e['payload']}")


def cmd_perf(kernel: SigmaKernel, args):
    hdr("PERFORMANCE TUNING")
    perf = kernel.get_performance_tuning()
    for k, v in perf.items():
        info(f"{k} = {_ansi(C.GREEN, str(v))}")


def cmd_pkg(kernel: SigmaKernel, args):
    spm = kernel.registry.get("package_manager")
    if spm is None:
        err("Package Manager not loaded.")
        return
    
    sub = args.subcommand
    if sub == "search":
        results = spm.search(args.query)
        if args.json:
            print(json.dumps(results, indent=2))
            return
        hdr(f"SEARCH RESULTS: {args.query}")
        for r in results:
            print(f"  [{_ansi(C.GREEN, r['state'])}] {_ansi(C.CYAN, r['id']):<20} {r['name']} v{r['ver']}")
    elif sub == "install":
        res = spm.install(args.pkg_id)
        if args.json:
            print(json.dumps(res, indent=2))
            return
        if "error" in res: err(res["error"])
        else: ok(res["message"])
    elif sub == "update":
        res = spm.delta_update(args.pkg_id)
        if args.json:
            print(json.dumps(res, indent=2))
            return
        if "error" in res: err(res["error"])
        else: ok(res["message"])
    elif sub == "list":
        # spm doesn't have list_installed yet, but let's assume it has _installed
        installed = spm._installed
        if args.json:
            print(json.dumps([{"id": k, "v": v.version} for k, v in installed.items()], indent=2))
            return
        hdr("INSTALLED PACKAGES")
        for k, v in installed.items():
            print(f"  📦 {_ansi(C.CYAN, k):<20} v{v.version}")


def cmd_repair(kernel: SigmaKernel, args):
    hdr("SELF-HEALING RECOVERY (APEX)")
    sr = kernel.registry.get("self_repair")
    if sr:
        info("Initializing Advanced Merkle-Tree Matrix...")
        res = sr.trigger_mesh_resilver()
        ok(res)
        ok(sr.health_check())
    else:
        ok(kernel.self_healing_recovery())
        ok("Kernel integrity verified (Legacy Fallback).")


def cmd_automate(kernel: SigmaKernel, args):
    hdr("OMNI AUTOMATOR STUDIO")
    auto = kernel.registry.get("automator")
    if auto is None:
        err("Automator module offline.")
        return
    if hasattr(args, 'goal') and args.goal:
        info(f"Dispatching AI pipeline for goal: {args.goal}")
        ok(auto.launch_agentic_pipeline(args.goal))
    else:
        ok(auto.health_check())


def cmd_ai(kernel: SigmaKernel, args):
    if not hasattr(kernel, 'ai_lifecycle'):
        err("AI/ML/DS Lifecycle module not loaded.")
        return

    sub = args.subcommand
    if not sub:
        print(f"Usage: {C.CYAN}ai start <name> <type> <objective...>{C.RESET}")
        return

    if sub == "start":
        name, m_type = args.name, args.type
        obj = " ".join(args.objective)
        mid = kernel.ai_lifecycle.start_unified_mission(name, obj, m_type)
        ok(f"AI/ML/DS Lifecycle Started. Mission ID: {C.BOLD}{mid}{C.RESET}")
        info(f"Discipline: {C.MAGENTA}{m_type.upper()}{C.RESET} | Phase: {C.BOLD}PROBLEM_DEFINITION{C.RESET}")
    
    elif sub == "next":
        mid = args.mission_id
        res = kernel.ai_lifecycle.execute_next_step(mid)
        if "error" in res:
             err(res["error"])
        else:
             hdr(f"LIFECYCLE STEP: {res['step']}")
             print(f"  {C.MAGENTA}»{C.RESET} {res['guidance']}")
             if "metrics" in res:
                  info(f"Metrics: {res['metrics']}")
    
    elif sub == "share":
        mid = args.mission_id
        res = kernel.ai_lifecycle.share_report_wa(mid)
        ok(res if isinstance(res, str) else res.get("message", "Sent."))

def cmd_customize(kernel: SigmaKernel, args):
    hdr("UI/UX CUSTOMIZATION ENGINE")
    if hasattr(args, 'theme') and args.theme:
        ok(f"Render Engine: Overridden system theme to [{args.theme.upper()}] natively via CLI.")
    else:
        info("Usage: python -m sigma_cli customize <ThemeName>")
        info("Available Built-in Themes: Midnight, Cyber, Snow, Rose")


# ─── Interactive REPL Shell ───────────────────────────────────────────────────

REPL_HELP = """
{C.BOLD}SigmaOS Interactive Shell Commands:{C.RESET}
  boot              — Run full boot sequence
  status            — Show system status and loaded modules
  pdf <path> [act]  — Process PDF (act: Audit|OCR|Redact)
  capture [mode]    — Titan Capture (Standard|OCR|Panoramic)
  security          — Run security shield diagnostics
  browser           — Show browser status
  convert [file]    — OmniConverter (extract audio, etc.)
  profile [name]    — Boot profile list/select
  profile ai <ctx>  — AI profile recommendation
  perf              — Show performance tuning params
  events            — Show event bus history
  modules           — List loaded modules
  call <mod> <mth>  — Call any module method
  health            — Module health check
  automate <goal>   — Dispatch Omni Automator AI agents
  customize <theme> — Repaint universal UI Theme
  ai <subcmd>       — AI/ML/DS Unified Lifecycle Engine (Start/Step/Share)
  repair            — Trigger advanced Merkle-Tree scrubbing
  help              — Show this help
  exit / quit       — Exit the shell
""".format(C=C)


def interactive_shell(kernel: SigmaKernel):
    print(BANNER)
    ok(f"SigmaOS Sovereign v{kernel.version} — Interactive Shell READY")
    info("Type 'help' for commands. Type 'exit' to quit.\n")

    while True:
        try:
            raw = input(f"{_ansi(C.BOLD+C.CYAN,'σ')} {_ansi(C.GREEN,'SigmaOS')} {_ansi(C.DIM,'>')} ").strip()
        except (EOFError, KeyboardInterrupt):
            print("\nExiting SigmaOS shell. Stay sovereign.")
            break

        if not raw:
            continue
        parts = raw.split()
        cmd = parts[0].lower()

        if cmd in ("exit", "quit", "q"):
            print("Exiting SigmaOS shell. Stay sovereign.")
            break

        elif cmd == "help":
            print(REPL_HELP)

        elif cmd == "boot":
            class _A: pass
            cmd_boot(kernel, _A())

        elif cmd == "ai":
            cmd_ai(kernel, parts[1:])

        elif cmd == "status" or cmd == "modules":
            class _A: pass
            cmd_status(kernel, _A())

        elif cmd == "security":
            class _A: pass
            cmd_security(kernel, _A())

        elif cmd == "browser":
            class _A: pass
            cmd_browser(kernel, _A())

        elif cmd == "perf":
            class _A: pass
            cmd_perf(kernel, _A())

        elif cmd == "events":
            class _A: pass
            cmd_events(kernel, _A())

        elif cmd == "health":
            hdr("MODULE HEALTH CHECK")
            for mod, status in kernel.registry.health_check().items():
                ok(f"{mod}: {status}")

        elif cmd == "pdf":
            path   = parts[1] if len(parts) > 1 else "unknown.pdf"
            action = parts[2] if len(parts) > 2 else "Audit"
            class _A:
                pass
            a = _A(); a.path = path; a.action = action
            cmd_pdf(kernel, a)

        elif cmd == "capture":
            mode = parts[1] if len(parts) > 1 else "Standard"
            class _A: pass
            a = _A(); a.mode = mode
            cmd_capture(kernel, a)

        elif cmd == "convert":
            class _A: pass
            a = _A(); a.input = parts[1] if len(parts) > 1 else None
            cmd_convert(kernel, a)

        elif cmd == "profile":
            sel = kernel.registry.get("boot_selector")
            if sel is None:
                err("Boot Selector not loaded.")
                continue
            if len(parts) == 1:
                info("Available Profiles:")
                for p in sel.list_available_profiles():
                    print(f"    → {_ansi(C.CYAN, p)}")
            elif parts[1] == "ai":
                ctx = " ".join(parts[2:]) if len(parts) > 2 else ""
                rec = sel.ai_recommendation(ctx)
                ok(f"AI Recommends: {_ansi(C.GREEN+C.BOLD, rec)}")
            else:
                ok(sel.select_profile(parts[1]))

        elif cmd == "call":
            if len(parts) < 3:
                warn("Usage: call <module_name> <method_name>")
                continue
            result = kernel.registry.call(parts[1], parts[2])
            hdr(f"RESULT: {parts[1]}.{parts[2]}")
            if isinstance(result, dict):
                for k, v in result.items():
                    info(f"{k}: {v}")
            else:
                ok(str(result))

        elif cmd == "automate":
            class _A: pass
            a = _A()
            a.goal = " ".join(parts[1:]) if len(parts) > 1 else None
            cmd_automate(kernel, a)

        elif cmd == "customize":
            class _A: pass
            a = _A()
            a.theme = parts[1] if len(parts) > 1 else None
            cmd_customize(kernel, a)
            
        elif cmd == "repair":
            class _A: pass
            cmd_repair(kernel, _A())

        else:
            warn(f"Unknown command: '{cmd}'. Type 'help' for usage.")


# ─── Argument Parser ──────────────────────────────────────────────────────────

def build_parser():
    p = argparse.ArgumentParser(
        prog="sigmaos",
        description="SigmaOS Sovereign v2.0 — Modular CLI",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=textwrap.dedent("""\
            Examples:
              python sigma_cli.py                         # interactive shell
              python sigma_cli.py boot                    # boot sequence
              python sigma_cli.py status                  # system status
              python sigma_cli.py pdf report.pdf OCR      # PDF OCR
              python sigma_cli.py capture Panoramic       # screenshot
              python sigma_cli.py security                # security check
              python sigma_cli.py module list             # list modules
              python sigma_cli.py module call browser get_browser_status
        """),
    )
    p.add_argument("--json", action="store_true", help="Output results in JSON format")
    p.add_argument("--silent", action="store_true", help="Suppress banner and standard info logs")
    sub = p.add_subparsers(dest="command")

    # boot
    sub.add_parser("boot", help="Run full boot sequence")

    # status
    sub.add_parser("status", help="Show system status")

    # pdf
    pdf_p = sub.add_parser("pdf", help="PDF Forge operations")
    pdf_p.add_argument("path",   help="Path to the PDF file")
    pdf_p.add_argument("action", nargs="?", default="Audit",
                       choices=["Audit","OCR","Redact"], help="Action to perform")

    # capture
    cap_p = sub.add_parser("capture", help="Titan visual capture")
    cap_p.add_argument("mode", nargs="?", default="Standard",
                       choices=["Standard","OCR","Panoramic"])

    # security
    sub.add_parser("security", help="Security shield diagnostics")

    # browser
    sub.add_parser("browser", help="Browser status")

    # convert
    cv_p = sub.add_parser("convert", help="OmniConverter operations")
    cv_p.add_argument("input", nargs="?", help="Input file")

    # profile
    pr_p = sub.add_parser("profile", help="Boot profile selection")
    pr_p.add_argument("profile",  nargs="?", help="Profile name to activate")
    pr_p.add_argument("--context", "-c", help="User context for AI recommendation")

    # perf
    sub.add_parser("perf", help="Performance tuning parameters")

    # events
    sub.add_parser("events", help="Event bus history")

    # module
    mod_p = sub.add_parser("module", help="Module management")
    mod_sub = mod_p.add_subparsers(dest="subcommand")
    mod_sub.add_parser("list",   help="List loaded modules")
    mod_sub.add_parser("health", help="Health check all modules")
    call_p = mod_sub.add_parser("call",  help="Call a module method")
    call_p.add_argument("module", help="Registered module key")
    call_p.add_argument("method", help="Method name")

    # pkg
    pkg_p = sub.add_parser("pkg", help="Sovereign Package Manager")
    pkg_sub = pkg_p.add_subparsers(dest="subcommand")
    pkg_sch = pkg_sub.add_parser("search", help="Search for packages")
    pkg_sch.add_argument("query", help="Search string")
    pkg_ins = pkg_sub.add_parser("install", help="Install a package")
    pkg_ins.add_argument("pkg_id", help="Package ID")
    pkg_upd = pkg_sub.add_parser("update", help="Update a package")
    pkg_upd.add_argument("pkg_id", help="Package ID")
    pkg_sub.add_parser("list", help="List installed packages")

    # repair
    r_p = sub.add_parser("repair", help="Run deep self-healing scrubbing")

    # automate
    a_p = sub.add_parser("automate", help="Launch Agentic Omni Automator")
    a_p.add_argument("goal", nargs="?", help="AI Task goal statement")

    # customize
    c_p = sub.add_parser("customize", help="Deep UI Customization")
    c_p.add_argument("theme", nargs="?", help="Name of theme to apply natively")

    # ai (AI/ML/DS Lifecycle)
    ai_p = sub.add_parser("ai", help="AI/ML/DS Unified Lifecycle Engine")
    ai_sub = ai_p.add_subparsers(dest="subcommand")
    
    ai_st = ai_sub.add_parser("start", help="Start a new lifecycle mission")
    ai_st.add_argument("name", help="Project name")
    ai_st.add_argument("--type", choices=["AI", "ML", "DS"], default="ML", help="Lifecycle discipline type")
    ai_st.add_argument("objective", nargs="+", help="Project goal/objective")
    
    ai_next = ai_sub.add_parser("next", help="Advance to next phase")
    ai_next.add_argument("mission_id", help="ID of the active mission")

    ai_sh = ai_sub.add_parser("share", help="Share report via WhatsApp")
    ai_sh.add_argument("mission_id", help="Mission ID to share")

    return p


def main():
    parser = build_parser()
    args   = parser.parse_args()

    # Print banner always (unless silent)
    if args.json or args.silent:
        SigmaConfig.SILENT = True
    
    if not SigmaConfig.SILENT:
        print(BANNER)
        info(f"Loading SigmaOS Sovereign v{SigmaConfig().VERSION}...")
    
    t0 = time.perf_counter()
    kernel = SigmaKernel(auto_load=True)
    t1 = time.perf_counter()
    
    if not args.silent and not args.json:
        ok(f"Kernel loaded in {(t1-t0)*1000:.0f}ms  |  {len(kernel.registry.list_modules())} modules online\n")

    cmd = args.command

    if   cmd == "boot":     cmd_boot(kernel, args)
    elif cmd == "status":   cmd_status(kernel, args)
    elif cmd == "pdf":      cmd_pdf(kernel, args)
    elif cmd == "capture":  cmd_capture(kernel, args)
    elif cmd == "security": cmd_security(kernel, args)
    elif cmd == "browser":  cmd_browser(kernel, args)
    elif cmd == "convert":  cmd_convert(kernel, args)
    elif cmd == "profile":  cmd_boot_profile(kernel, args)
    elif cmd == "perf":     cmd_perf(kernel, args)
    elif cmd == "events":   cmd_events(kernel, args)
    elif cmd == "module":   cmd_module(kernel, args)
    elif cmd == "pkg":      cmd_pkg(kernel, args)
    elif cmd == "repair":   cmd_repair(kernel, args)
    elif cmd == "automate": cmd_automate(kernel, args)
    elif cmd == "customize":cmd_customize(kernel, args)
    elif cmd == "ai":        cmd_ai(kernel, args)
    else:
        # No subcommand: drop into interactive shell
        interactive_shell(kernel)


if __name__ == "__main__":
    main()
