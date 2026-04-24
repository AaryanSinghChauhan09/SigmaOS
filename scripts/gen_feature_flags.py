#!/usr/bin/env python3
"""
SigmaOS: Feature Flag Compiler (reads sigma_features.json → emits -D flags)

Usage:
    python3 scripts/gen_feature_flags.py          # stdout: -DSIGMA_...
    eval $(python3 scripts/gen_feature_flags.py)   # shell integration

Called automatically by build_sovereign.sh and sovereign_builder.py.
"""

import json
import os
import sys

CONFIG_PATH = os.path.join(os.path.dirname(__file__), '..', 'sigma_features.json')

def main():
    if not os.path.exists(CONFIG_PATH):
        print("", end="")
        return

    with open(CONFIG_PATH) as f:
        cfg = json.load(f)

    flags = []

    # §1 Architecture
    arch = cfg.get("arch", "x86_64").upper()
    flags.append(f"-DSIGMA_ARCH_{arch}")

    # §2 Drivers
    drivers = cfg.get("drivers", {})
    for subsystem in ["display", "storage", "network"]:
        drv = drivers.get(subsystem)
        if drv:
            flags.append(f"-DSIGMA_DRIVER_{drv.upper()}")

    # §3 Feature toggles
    features = cfg.get("features", {})
    for key, val in features.items():
        flags.append(f"-DSIGMA_FEATURE_{key.upper()}={1 if val else 0}")

    # §4 Memory config
    memory = cfg.get("memory", {})
    for key, val in memory.items():
        flags.append(f"-DSIGMA_{key.upper()}={val}")

    # §5 Build metadata
    build = cfg.get("build", {})
    for key, val in build.items():
        if isinstance(val, int):
            flags.append(f"-DSIGMA_VERSION_{key.upper()}={val}")
        elif isinstance(val, str):
            flags.append(f'-DSIGMA_BUILD_{key.upper()}=\\"{val}\\"')

    print(" ".join(flags))

if __name__ == "__main__":
    main()
