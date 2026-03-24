"""
SigmaOS repository health checks for import stability and baseline tests.
"""

from __future__ import annotations

import importlib
import pkgutil
import subprocess
import sys


def check_critical_imports() -> list[str]:
    critical_modules = [
        "userland.system_api.forensic_scanner",
        "userland.system_api.circuit_breaker",
        "userland.system_api.bio_lock",
        "userland.system_api.sovereign_watchdog",
        "userland.system_api.omni_search_v2",
        "userland.system_api.sovereign_clipboard_v2",
        "userland.system_api.aether_orchestrator",
        "userland.system_api.agentic_claw",
        "userland.system_api.task_scheduler",
        "userland.system_api.vanguard",
        "userland.system_api.security_warden",
        "userland.system_api.resource_orchestrator",
        "userland.system_api.sigma_omni_api",
        "userland.system_api.package_manager",
    ]
    failures: list[str] = []
    for module_name in critical_modules:
        try:
            importlib.import_module(module_name)
            print(f"[OK] import {module_name}")
        except Exception as exc:  # pragma: no cover - operational check
            failures.append(f"{module_name}: {exc}")
            print(f"[FAIL] import {module_name}: {exc}")
    return failures


def list_system_api_health() -> list[str]:
    failures: list[str] = []
    package_name = "userland.system_api"
    package = importlib.import_module(package_name)
    for _, name, _ in pkgutil.iter_modules(package.__path__):
        module_name = f"{package_name}.{name}"
        try:
            importlib.import_module(module_name)
        except Exception as exc:
            failures.append(f"{module_name}: {exc}")
    print(f"[INFO] system_api modules failing import: {len(failures)}")
    return failures


def run_pytest() -> int:
    process = subprocess.run([sys.executable, "-m", "pytest", "tests", "-q"], check=False)
    return process.returncode


def main() -> int:
    hard_failures = check_critical_imports()
    _ = list_system_api_health()
    pytest_code = run_pytest()
    if hard_failures:
        print("[ERROR] Critical import failures detected.")
        return 1
    if pytest_code != 0:
        print("[ERROR] Pytest failures detected.")
        return pytest_code
    print("[OK] SigmaOS health gate passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
