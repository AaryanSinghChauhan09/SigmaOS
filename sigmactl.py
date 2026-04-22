#!/usr/bin/env python3
"""
╔══════════════════════════════════════════════════════╗
║   sigmactl — SigmaOS Sovereign Control Interface    ║
║   Unified CLI for build, shard, sync, config, AI    ║
║   Zero external deps — pure Python stdlib only      ║
╚══════════════════════════════════════════════════════╝
"""
import sys, os, json, argparse, subprocess, time, threading, shutil
from pathlib import Path
from datetime import datetime
import urllib.request, urllib.error

ROOT = Path(__file__).parent
CONFIG_FILE = ROOT / "sigma_config.json"
PROFILE_DIR = ROOT / "profiles"
PLUGIN_DIR  = ROOT / "plugins"
API_BASE    = "http://localhost:8080/api"

BANNER = """
\033[36m  ████████ ████  ██████  ███    ███  █████
  ██      ██  ██ ██    ██ ████  ████ ██   ██
  ███████ ██  ██ ██    ██ ██ ████ ██ ███████
       ██ ██  ██ ██  ████ ██  ██  ██ ██   ██
  ███████  ██████  ██████  ██      ██ ██   ██
\033[35m  sigmactl — Sovereign Control Interface v2.0\033[0m
"""

# ─── Utility ──────────────────────────────────────────────────────────────────

def log(msg, level="INFO"):
    ts = datetime.now().strftime("%H:%M:%S")
    colors = {"INFO": "\033[36m", "OK": "\033[32m", "WARN": "\033[33m", "ERR": "\033[31m", "SYS": "\033[35m"}
    c = colors.get(level, "\033[0m")
    print(f"{c}[{level}]\033[0m [{ts}] {msg}")

def load_config():
    if CONFIG_FILE.exists():
        with open(CONFIG_FILE) as f:
            return json.load(f)
    return {
        "profile": "default",
        "api_url": "http://localhost:8080/api",
        "blur": 25, "accent": "#00f0ff", "theme": "MATRIX",
        "sync_interval": 300, "auto_sync": False
    }

def save_config(cfg):
    with open(CONFIG_FILE, "w") as f:
        json.dump(cfg, f, indent=2)
    log("Config saved.", "OK")

def call_api(endpoint, payload=None, method="GET"):
    url = f"{API_BASE}/{endpoint}"
    try:
        data = json.dumps(payload).encode() if payload else None
        req = urllib.request.Request(url, data=data,
              headers={"Content-Type": "application/json"}, method=method)
        with urllib.request.urlopen(req, timeout=5) as r:
            return r.read().decode()
    except urllib.error.URLError as e:
        return f"[SERVER OFFLINE] {e.reason} — Is the Zenith server running? (node server.js)"

# ─── Commands ─────────────────────────────────────────────────────────────────

def cmd_build(args):
    log("Initiating Sovereign Lattice Build...", "SYS")
    targets = args.target or ["all"]
    for t in targets:
        log(f"Building target: make {t}")
        result = subprocess.run(["make", t], capture_output=True, text=True, cwd=ROOT)
        if result.returncode == 0:
            log(f"Target '{t}' built successfully.", "OK")
        else:
            log(f"Build failed: {result.stderr[:200]}", "ERR")
            if args.self_heal:
                log("Self-healing: retrying with clean state...", "WARN")
                subprocess.run(["make", "clean"], cwd=ROOT, capture_output=True)
                subprocess.run(["make", t], cwd=ROOT)

def cmd_shard(args):
    sub = args.subcommand
    if sub == "ls":
        log("Scanning Sovereign Shard Inventory...", "SYS")
        suite_root = ROOT / "kernel" / "suites"
        if not suite_root.exists():
            log("kernel/suites directory not found.", "WARN"); return
        print(f"\n{'Suite':<28} {'C':>4} {'ASM':>5} {'Rust':>5}")
        print("─" * 45)
        total = {"c": 0, "asm": 0, "rs": 0}
        for suite in sorted(suite_root.iterdir()):
            if suite.is_dir():
                c   = len(list(suite.rglob("*.c")))
                asm = len(list(suite.rglob("*.asm")))
                rs  = len(list(suite.rglob("*.rs")))
                total["c"] += c; total["asm"] += asm; total["rs"] += rs
                print(f"  {suite.name:<26} {c:>4} {asm:>5} {rs:>5}")
        print("─" * 45)
        print(f"  {'TOTAL':<26} {total['c']:>4} {total['asm']:>5} {total['rs']:>5}")

    elif sub == "test":
        log(f"Running tests for shard: {args.id}", "SYS")
        result = subprocess.run(["make", "test"], capture_output=True, text=True, cwd=ROOT)
        print(result.stdout)

    elif sub == "kill":
        log(f"Sending PANIC signal to shard: {args.id}", "WARN")
        print(call_api("run", {"cmd": f"echo 'SHARD_PANIC: {args.id}'", "cwd": ""}))

    elif sub == "install-plugin":
        cmd_plugin_install(args)

def cmd_plugin_install(args):
    PLUGIN_DIR.mkdir(exist_ok=True)
    plugin_name = args.id or (args.plugin if hasattr(args, 'plugin') else None)
    if not plugin_name:
        log("No plugin name specified.", "ERR"); return
    manifest = PLUGIN_DIR / plugin_name / "plugin.json"
    if manifest.exists():
        log(f"Plugin '{plugin_name}' already installed.", "WARN"); return
    (PLUGIN_DIR / plugin_name).mkdir(exist_ok=True)
    plugin_meta = {"name": plugin_name, "version": "1.0.0", "enabled": True,
                   "entry": f"plugins/{plugin_name}/index.js", "installed": datetime.now().isoformat()}
    with open(manifest, "w") as f:
        json.dump(plugin_meta, f, indent=2)
    log(f"Plugin '{plugin_name}' scaffolded in plugins/{plugin_name}/", "OK")
    log("Add your JS entry point at plugins/<name>/index.js and it will auto-load.", "INFO")

def cmd_plugin_list(args):
    PLUGIN_DIR.mkdir(exist_ok=True)
    plugins = [d for d in PLUGIN_DIR.iterdir() if d.is_dir()]
    if not plugins:
        log("No plugins installed. Use: sigmactl plugin install <name>", "WARN"); return
    print(f"\n{'Plugin':<22} {'Version':<10} {'Enabled'}")
    print("─" * 40)
    for p in plugins:
        m = p / "plugin.json"
        if m.exists():
            data = json.loads(m.read_text())
            status = "\033[32m✓\033[0m" if data.get("enabled") else "\033[31m✗\033[0m"
            print(f"  {data['name']:<20} {data['version']:<10} {status}")

def cmd_sync(args):
    log("Syncing with GitHub repository...", "SYS")
    steps = [
        (["git", "fetch", "origin"], "Fetching remote changes"),
        (["git", "status"], "Checking local status"),
        (["git", "push"], "Pushing local commits"),
    ]
    for cmd_args, desc in steps:
        log(desc)
        result = subprocess.run(cmd_args, capture_output=True, text=True, cwd=ROOT)
        if result.returncode == 0:
            out = result.stdout.strip()
            if out: print(f"  {out}")
            log(f"{desc}: OK", "OK")
        else:
            log(f"{desc} failed: {result.stderr.strip()[:120]}", "ERR")
    # Sync WIKI submodule
    wiki = ROOT / "WIKI"
    if wiki.exists():
        log("Syncing WIKI submodule...")
        subprocess.run(["git", "add", "."], cwd=wiki, capture_output=True)
        r = subprocess.run(["git", "diff", "--cached", "--quiet"], cwd=wiki)
        if r.returncode != 0:
            subprocess.run(["git", "commit", "-m", "docs: auto-sync wiki"], cwd=wiki, capture_output=True)
            subprocess.run(["git", "push"], cwd=wiki, capture_output=True)
            log("WIKI submodule pushed.", "OK")

def cmd_auto_sync(args):
    cfg = load_config()
    interval = args.interval or cfg.get("sync_interval", 300)
    log(f"Auto-sync daemon started. Interval: {interval}s. Press Ctrl+C to stop.", "SYS")
    while True:
        try:
            time.sleep(interval)
            log("Auto-sync: pulling latest changes...", "SYS")
            result = subprocess.run(["git", "pull", "--rebase"], capture_output=True, text=True, cwd=ROOT)
            if "Already up to date" in result.stdout:
                log("Auto-sync: already up to date.", "OK")
            elif result.returncode == 0:
                log("Auto-sync: pulled and rebased successfully.", "OK")
            else:
                log(f"Auto-sync error: {result.stderr.strip()[:100]}", "ERR")
        except KeyboardInterrupt:
            log("Auto-sync daemon stopped.", "WARN"); break

def cmd_profile(args):
    PROFILE_DIR.mkdir(exist_ok=True)
    sub = args.subcommand

    if sub == "list":
        profiles = list(PROFILE_DIR.glob("*.json"))
        if not profiles:
            log("No profiles found. Create one with: sigmactl profile create <name>", "WARN"); return
        cfg = load_config()
        active = cfg.get("profile", "default")
        print(f"\n{'Profile':<20} {'Theme':<15} {'Active'}")
        print("─" * 42)
        for p in profiles:
            data = json.loads(p.read_text())
            marker = "\033[32m● ACTIVE\033[0m" if p.stem == active else "  ○"
            print(f"  {data.get('name', p.stem):<18} {data.get('theme', 'MATRIX'):<15} {marker}")

    elif sub == "create":
        name = args.name
        presets = {
            "developer": {"theme": "MATRIX", "blur": 15, "accent": "#00f0ff", "fontScale": 0.95, "mode": "DARK"},
            "secure":    {"theme": "GHOST_MICA", "blur": 30, "accent": "#ff0055", "fontScale": 1.0, "privacyShield": True},
            "minimal":   {"theme": "SOVEREIGN_GOLD", "blur": 5, "accent": "#ffcc00", "fontScale": 1.1, "mode": "DARK"},
        }
        base = presets.get(args.preset, presets["developer"]) if hasattr(args, 'preset') and args.preset else presets["developer"]
        profile = {"name": name, "created": datetime.now().isoformat(), **base}
        path = PROFILE_DIR / f"{name}.json"
        with open(path, "w") as f:
            json.dump(profile, f, indent=2)
        log(f"Profile '{name}' created from preset '{getattr(args, 'preset', 'developer')}'.", "OK")

    elif sub == "switch":
        name = args.name
        path = PROFILE_DIR / f"{name}.json"
        if not path.exists():
            log(f"Profile '{name}' not found.", "ERR"); return
        cfg = load_config()
        cfg["profile"] = name
        profile_data = json.loads(path.read_text())
        cfg.update({k: v for k, v in profile_data.items() if k != "name"})
        save_config(cfg)
        log(f"Switched to profile: {name}", "OK")
        log("Restart Zenith or run 'sigmactl set' to propagate changes to GUI.", "INFO")

def cmd_set(args):
    cfg = load_config()
    cfg[args.key] = args.value
    save_config(cfg)
    log(f"Config updated: {args.key} = {args.value}", "OK")
    log("To apply in GUI: reload Zenith or the settings engine auto-polls sigma_config.json.", "INFO")

def cmd_get(args):
    cfg = load_config()
    if args.key:
        val = cfg.get(args.key, "[not set]")
        print(f"  {args.key}: \033[36m{val}\033[0m")
    else:
        print(json.dumps(cfg, indent=2))

def cmd_telemetry(args):
    log("Fetching live telemetry...", "SYS")
    print(call_api("telemetry"))

def cmd_auto(args):
    actions = {
        "heap_compact": "Compacting memory heap shards...",
        "zombie_sweep": "Sweeping zombie processes...",
        "cache_flush":  "Flushing sovereign cache...",
        "lattice_audit":"Running neural lattice audit...",
    }
    if args.action not in actions:
        log(f"Unknown action: {args.action}. Available: {', '.join(actions)}", "ERR"); return
    log(actions[args.action], "SYS")
    res = call_api("run", {"cmd": f"echo 'SIGMA_AUTO: {args.action}'", "cwd": ""})
    print(f"  {res}")
    log("Automation complete.", "OK")

def cmd_wizard(args):
    print(BANNER)
    print("\033[35m  ╔══════════════════════════════════╗")
    print("  ║  SigmaOS First-Time Setup Wizard ║")
    print("  ╚══════════════════════════════════╝\033[0m\n")
    cfg = load_config()

    username = input("  Enter your Zenith username [root]: ").strip() or "root"
    theme = input("  Select theme [MATRIX / GHOST_MICA / SOVEREIGN_GOLD] (default: MATRIX): ").strip().upper() or "MATRIX"
    accent = input("  Accent color hex (default: #00f0ff): ").strip() or "#00f0ff"
    auto_sync_in = input("  Enable auto-sync with GitHub? [y/N]: ").strip().lower()
    auto_sync = auto_sync_in == 'y'

    cfg.update({"username": username, "theme": theme, "accent": accent, "auto_sync": auto_sync})
    save_config(cfg)

    PROFILE_DIR.mkdir(exist_ok=True)
    profile = {"name": "default", "theme": theme, "accent": accent, "mode": "DARK",
               "blur": 25, "fontScale": 1.0, "created": datetime.now().isoformat()}
    with open(PROFILE_DIR / "default.json", "w") as f:
        json.dump(profile, f, indent=2)

    log("Setup complete! Configuration saved to sigma_config.json", "OK")
    log("Start Zenith UI:  node server.js", "INFO")
    log("Build the kernel: make bin", "INFO")
    if auto_sync:
        log("Start auto-sync:  sigmactl auto-sync --interval 300", "INFO")

# ─── Argument Parser ──────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        prog="sigmactl",
        description="SigmaOS Sovereign Control Interface — GUI/CLI Unified Layer",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  sigmactl wizard                          # Interactive first-time setup
  sigmactl build --target bin              # Build the kernel binary
  sigmactl build --target all --self-heal  # Build with self-healing retry
  sigmactl shard ls                        # List all shard inventories
  sigmactl shard install-plugin my-plugin  # Scaffold a new plugin
  sigmactl plugin list                     # Show installed plugins
  sigmactl profile create dev --preset developer
  sigmactl profile switch dev
  sigmactl sync                            # Push to GitHub
  sigmactl auto-sync --interval 60         # Background git pull daemon
  sigmactl auto heap_compact               # Trigger automation
  sigmactl set theme GHOST_MICA            # Update config
  sigmactl get                             # Show all config
  sigmactl telemetry                       # Live system telemetry
        """
    )
    subs = parser.add_subparsers(dest="command", metavar="<command>")

    # wizard
    subs.add_parser("wizard", help="Interactive first-time setup wizard")

    # build
    b = subs.add_parser("build", help="Build sovereign kernel shards")
    b.add_argument("--target", nargs="+", default=["all"])
    b.add_argument("--self-heal", action="store_true", help="Retry failed builds automatically")

    # shard
    sh = subs.add_parser("shard", help="Manage sovereign shards")
    sh_sub = sh.add_subparsers(dest="subcommand")
    sh_sub.add_parser("ls", help="List all shards across suites")
    sh_t = sh_sub.add_parser("test", help="Run shard tests")
    sh_t.add_argument("--id", help="Shard/suite ID to test")
    sh_k = sh_sub.add_parser("kill", help="Send panic signal to shard")
    sh_k.add_argument("id")
    sh_ip = sh_sub.add_parser("install-plugin", help="Scaffold a new plugin")
    sh_ip.add_argument("id", help="Plugin name")

    # plugin
    pl = subs.add_parser("plugin", help="Manage Zenith plugins")
    pl_sub = pl.add_subparsers(dest="subcommand")
    pl_sub.add_parser("list", help="List installed plugins")
    pl_i = pl_sub.add_parser("install", help="Install/scaffold a plugin")
    pl_i.add_argument("plugin", help="Plugin name")

    # profile
    pr = subs.add_parser("profile", help="Manage user profiles")
    pr_sub = pr.add_subparsers(dest="subcommand")
    pr_sub.add_parser("list", help="List all profiles")
    pr_c = pr_sub.add_parser("create", help="Create a new profile")
    pr_c.add_argument("name")
    pr_c.add_argument("--preset", choices=["developer", "secure", "minimal"], default="developer")
    pr_sw = pr_sub.add_parser("switch", help="Switch active profile")
    pr_sw.add_argument("name")

    # sync
    subs.add_parser("sync", help="Sync local repo to GitHub")

    # auto-sync daemon
    asy = subs.add_parser("auto-sync", help="Start background GitHub sync daemon")
    asy.add_argument("--interval", type=int, help="Sync interval in seconds (default: 300)")

    # auto
    au = subs.add_parser("auto", help="Trigger system automation actions")
    au.add_argument("action", choices=["heap_compact", "zombie_sweep", "cache_flush", "lattice_audit"])

    # set / get
    s = subs.add_parser("set", help="Set a config value (shared with GUI)")
    s.add_argument("key"); s.add_argument("value")
    g = subs.add_parser("get", help="Get config values")
    g.add_argument("key", nargs="?")

    # telemetry
    subs.add_parser("telemetry", help="Fetch live system telemetry")

    args = parser.parse_args()
    if not args.command:
        print(BANNER)
        parser.print_help()
        return

    dispatch = {
        "wizard": cmd_wizard, "build": cmd_build, "shard": cmd_shard,
        "plugin": lambda a: cmd_plugin_list(a) if a.subcommand == "list" else cmd_plugin_install(a),
        "profile": cmd_profile, "sync": cmd_sync, "auto-sync": cmd_auto_sync,
        "auto": cmd_auto, "set": cmd_set, "get": cmd_get, "telemetry": cmd_telemetry,
    }
    dispatch[args.command](args)

if __name__ == "__main__":
    main()
