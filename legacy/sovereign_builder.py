#!/usr/bin/env python3
"""
SigmaOS Sovereign Build Orchestrator v4 (Modular)
- High-performance, multi-arch build engine
- Declarative feature-flag integration
- Topological dependency resolution
- Industrial-grade CLI
"""

import sys
import os
import argparse
import time

# Add lib directory to path
sys.path.append(os.path.join(os.path.dirname(__file__), "lib"))

from config import SigmaConfig
from module_manager import ModuleManager
from builder import SigmaBuilder

def print_banner():
    banner = r"""
    SigmaOS Sovereign Build Orchestrator v4
    =======================================
    [Sovereign Lattice] -> [Pure Silicon]
    """
    print(banner)

def main():
    parser = argparse.ArgumentParser(description="SigmaOS Sovereign Build System")
    parser.add_argument("command", choices=["build", "clean", "graph", "config", "list"], default="build", nargs="?")
    parser.add_argument("--arch", help="Target architecture (x86_64, aarch64, riscv64)")
    parser.add_argument("--config", default="sigma_features.json", help="Path to feature config")
    parser.add_argument("--out", default="build", help="Output directory")
    parser.add_argument("--feature", action="append", help="Override feature (e.g. --feature network=true)")
    parser.add_argument("--verbose", action="store_true", help="Enable verbose output")

    args = parser.parse_args()

    print_banner()

    # 1. Load Configuration
    cfg = SigmaConfig(args.config)
    
    # Apply CLI overrides
    if args.arch:
        cfg.data["arch"] = args.arch
    
    if args.feature:
        for f in args.feature:
            if "=" in f:
                k, v = f.split("=", 1)
                val = v.lower() == "true" if v.lower() in ["true", "false"] else v
                cfg.update_feature(k, val)

    arch = cfg.get_arch()
    cflags = cfg.get_cflags()

    # 2. Module Discovery
    mgr = ModuleManager()
    mgr.discover()

    if args.command == "list":
        print(f"[*] Discovered {len(mgr.modules)} modules:")
        for m in mgr.modules:
            print(f"  - {m['module']} v{m.get('version', '1.0.0')} ({m['_dir']})")
        return

    if args.command == "graph":
        import json
        print(json.dumps(mgr.get_graph(), indent=2))
        return

    if args.command == "clean":
        builder = SigmaBuilder(arch, cflags, args.out)
        builder.clean()
        return

    if args.command == "config":
        import json
        print(json.dumps(cfg.data, indent=2))
        return

    # 3. Build Logic
    if args.command == "build":
        start_time = time.time()
        print(f"[*] Target Arch: {arch}")
        print(f"[*] CFLAGS:      {cflags}\n")

        ordered_modules = mgr.get_ordered_modules(arch=arch)
        print(f"[*] Resolved build order for {len(ordered_modules)} modules...\n")

        builder = SigmaBuilder(arch, cflags, args.out)
        all_objects = []

        for mod in ordered_modules:
            print(f"[*] Building {mod['module']}...")
            objs = builder.build_module(mod)
            all_objects.extend(objs)

        if not all_objects:
            print("[!] No object files generated. Build failed.")
            sys.exit(1)

        kernel_bin = builder.link_image(all_objects)
        builder.package_iso(kernel_bin)

        duration = time.time() - start_time
        print(f"\n[✓] Build Success! ({duration:.2f}s)")
        print(f"[+] Artifacts: {args.out}/")

if __name__ == "__main__":
    main()
