#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
# sigma-pkg/cbuild.py — Chimera Linux cports-inspired package build runner
#
# Usage:
#   python cbuild.py build sigma-pkg/templates/sigma-healthd/template.py
#   python cbuild.py hash  sigma-pkg/templates/sigma-healthd/template.py
#   python cbuild.py lint  sigma-pkg/templates/sigma-healthd/template.py

import importlib.util
import hashlib
import json
import subprocess
import sys
import os
import pathlib
from typing import Any

# ── Hardening flag → compiler flag mapping ────────────────────────────────
HARDENING_FLAGS = {
    "vis":   ["-fvisibility=hidden"],
    "cfi":   ["-fsanitize=cfi", "-fno-sanitize-trap=cfi"],
    "ssp":   ["-fstack-protector-strong"],
    "pie":   ["-fPIE", "-pie"],
    "relro": ["-Wl,-z,relro,-z,now"],
    "safestack": ["-fsanitize=safe-stack"],
    "fortify":   ["-D_FORTIFY_SOURCE=2"],
}


def load_template(path: str) -> Any:
    """Import a template.py as a module (Chimera cbuild pattern)."""
    spec = importlib.util.spec_from_file_location("template", path)
    tmpl = importlib.util.module_from_spec(spec)

    # Inject cbuild helpers into the template namespace
    tmpl.subpackage = _subpackage_decorator

    spec.loader.exec_module(tmpl)
    return tmpl


def _subpackage_decorator(name: str):
    """@subpackage("pkg-devel") decorator — marks a function as subpackage generator."""
    def decorator(fn):
        fn._subpackage_name = name
        return fn
    return decorator


def compute_derivation_hash(tmpl) -> str:
    """
    GNU Guix-style derivation hash.
    Hash of: name + version + source_sha256 + sorted makedepends +
             sorted hardening + tool_flags + sorted cmake_args.
    Same inputs → same hash → same output → provably reproducible.
    """
    drv = {
        "name":        getattr(tmpl, "pkgname", ""),
        "version":     getattr(tmpl, "pkgver", ""),
        "source_sha":  getattr(tmpl, "sha256", ""),
        "makedepends": sorted(getattr(tmpl, "makedepends", [])),
        "hardening":   sorted(getattr(tmpl, "hardening", [])),
        "tool_flags":  getattr(tmpl, "tool_flags", {}),
        "cmake_args":  sorted(getattr(tmpl, "cmake_args", [])),
        "build_style": getattr(tmpl, "build_style", ""),
    }
    drv_json = json.dumps(drv, sort_keys=True).encode()
    return hashlib.sha256(drv_json).hexdigest()[:32]


def install_path(tmpl) -> str:
    """
    Guix /gnu/store/<hash>-pkg-ver/ pattern.
    Same derivation hash = same path on any machine.
    """
    h = compute_derivation_hash(tmpl)
    return f"/sigma/store/{h}-{tmpl.pkgname}-{tmpl.pkgver}"


def build_env(tmpl) -> dict:
    """Build environment variables from typed template fields."""
    cflags: list[str] = []
    ldflags: list[str] = []

    # Apply hardening flags (typed list, not shell string)
    for flag in getattr(tmpl, "hardening", []):
        if flag in HARDENING_FLAGS:
            for f in HARDENING_FLAGS[flag]:
                if f.startswith("-Wl,"):
                    ldflags.append(f)
                else:
                    cflags.append(f)

    # Merge with template's explicit tool_flags
    for f in getattr(tmpl, "tool_flags", {}).get("CXXFLAGS", []):
        cflags.append(f)
    for f in getattr(tmpl, "tool_flags", {}).get("LDFLAGS", []):
        ldflags.append(f)

    env = dict(os.environ)
    env["CXXFLAGS"]         = " ".join(cflags)
    env["LDFLAGS"]          = " ".join(ldflags)
    env["SOURCE_DATE_EPOCH"] = "0"           # reproducible builds
    env["DESTDIR"]           = install_path(tmpl)

    return env


def lint_template(tmpl) -> list[str]:
    """Validate a template for required fields and best practices."""
    errors = []
    for field in ("pkgname", "pkgver", "pkgdesc", "license"):
        if not getattr(tmpl, field, ""):
            errors.append(f"Missing required field: {field}")
    if not getattr(tmpl, "hardening", []):
        errors.append("No hardening flags set — add at least 'pie' and 'ssp'")
    if not getattr(tmpl, "makedepends", []):
        errors.append("makedepends is empty — likely missing deps")
    return errors


def build_package(template_path: str) -> int:
    """Full build: pre_build → cmake → build → post_install."""
    tmpl = load_template(template_path)

    # Lint first
    errors = lint_template(tmpl)
    if errors:
        for e in errors:
            print(f"[cbuild] LINT ERROR: {e}", file=sys.stderr)
        return 1

    drv_hash = compute_derivation_hash(tmpl)
    dest = install_path(tmpl)
    env  = build_env(tmpl)

    print(f"[cbuild] Building {tmpl.pkgname}-{tmpl.pkgver}")
    print(f"[cbuild] Derivation hash: {drv_hash}")
    print(f"[cbuild] Install path:    {dest}")

    # pre_build hook
    if hasattr(tmpl, "pre_build"):
        print("[cbuild] Running pre_build...")
        tmpl.pre_build(tmpl)

    # cmake configure
    if getattr(tmpl, "build_style", "") == "cmake":
        cmake_cmd = ["cmake", "-B", "build", "-G", "Ninja",
                     *getattr(tmpl, "cmake_args", [])]
        ret = subprocess.run(cmake_cmd, env=env)
        if ret.returncode != 0:
            return ret.returncode

        ret = subprocess.run(["ninja", "-C", "build", f"-j{os.cpu_count()}"],
                             env=env)
        if ret.returncode != 0:
            return ret.returncode

    # post_install hook
    if hasattr(tmpl, "post_install"):
        print("[cbuild] Running post_install...")
        tmpl.post_install(tmpl)

    print(f"[cbuild] Done: {tmpl.pkgname}-{tmpl.pkgver} → {dest}")
    return 0


def main():
    if len(sys.argv) < 3:
        print("Usage: cbuild.py [build|hash|lint] <template.py>")
        sys.exit(1)

    cmd, path = sys.argv[1], sys.argv[2]
    tmpl = load_template(path)

    if cmd == "build":
        sys.exit(build_package(path))
    elif cmd == "hash":
        print(compute_derivation_hash(tmpl))
    elif cmd == "lint":
        errors = lint_template(tmpl)
        if errors:
            for e in errors:
                print(f"ERROR: {e}")
            sys.exit(1)
        else:
            print(f"OK: {tmpl.pkgname} template is valid")
    else:
        print(f"Unknown command: {cmd}")
        sys.exit(1)


if __name__ == "__main__":
    main()
