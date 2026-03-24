"""
Launch readiness checker for SigmaOS.
"""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
REPORT_PATH = ROOT / "docs" / "LAUNCH_READINESS_REPORT.md"


@dataclass
class CheckResult:
    name: str
    passed: bool
    detail: str


def run_command(cmd: list[str]) -> tuple[bool, str]:
    proc = subprocess.run(cmd, cwd=ROOT, text=True, capture_output=True, check=False)
    ok = proc.returncode == 0
    detail = (proc.stdout or proc.stderr).strip()
    return ok, detail


def check_file_exists(rel_path: str) -> CheckResult:
    p = ROOT / rel_path
    return CheckResult(
        name=f"File exists: {rel_path}",
        passed=p.exists(),
        detail="present" if p.exists() else "missing",
    )


def check_readme_clone_url() -> CheckResult:
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    expected = "https://github.com/AaryanSinghChauhan09/SigmaOS.git"
    return CheckResult(
        name="README clone URL is correct",
        passed=expected in readme,
        detail="matched expected repo URL" if expected in readme else "README has stale clone URL",
    )


def check_how_to_run_has_machine_paths() -> CheckResult:
    text = (ROOT / "HOW_TO_RUN_SIGMAOS.md").read_text(encoding="utf-8")
    bad = ("file:///" in text) or ("C:/Users/SigmaUser" in text)
    return CheckResult(
        name="HOW_TO_RUN uses portable paths",
        passed=not bad,
        detail="portable" if not bad else "contains machine-specific path(s)",
    )


def main() -> int:
    results: list[CheckResult] = []

    # Core runtime checks
    ok, out = run_command([sys.executable, "sigma_health_check.py"])
    results.append(CheckResult("sigma_health_check.py passes", ok, out.splitlines()[-1] if out else ""))

    ok, out = run_command([sys.executable, "-m", "pytest", "tests", "-q"])
    results.append(CheckResult("pytest tests pass", ok, out.splitlines()[-1] if out else ""))

    # Docs + launch asset checks
    results.append(check_file_exists("README.md"))
    results.append(check_file_exists("HOW_TO_RUN_SIGMAOS.md"))
    results.append(check_file_exists("LAUNCH_STATUS.md"))
    results.append(check_file_exists("setup.ps1"))
    results.append(check_file_exists("setup.sh"))
    results.append(check_file_exists("boot.py"))
    results.append(check_file_exists(".github/workflows/sigma-health.yml"))
    results.append(check_readme_clone_url())
    results.append(check_how_to_run_has_machine_paths())

    passed = sum(1 for r in results if r.passed)
    total = len(results)
    status = "READY_WITH_BASELINE_CHECKS" if passed == total else "NOT_READY"
    now = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")

    lines = [
        "# SigmaOS Launch Readiness Report",
        "",
        f"- Generated: **{now}**",
        f"- Result: **{status}**",
        f"- Score: **{passed}/{total}**",
        "",
        "| Check | Status | Detail |",
        "|---|---|---|",
    ]
    for r in results:
        lines.append(f"| {r.name} | {'PASS' if r.passed else 'FAIL'} | {r.detail.replace('|', '/')} |")
    lines.append("")
    REPORT_PATH.write_text("\n".join(lines), encoding="utf-8")
    print(f"Wrote {REPORT_PATH}")
    print(f"Score: {passed}/{total}")
    return 0 if passed == total else 1


if __name__ == "__main__":
    raise SystemExit(main())
