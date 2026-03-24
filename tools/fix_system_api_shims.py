"""
Bulk-fix malformed SigmaOS system_api package shims.

This tool normalizes __init__.py files that were generated with invalid
self-referential imports such as:
    from .module_name.something import X
"""

from __future__ import annotations

from pathlib import Path
import re


ROOT = Path(__file__).resolve().parents[1]
SYSTEM_API_DIR = ROOT / "userland" / "system_api"
REPORT_FILE = ROOT / "docs" / "SYSTEM_API_SHIM_REPAIR_REPORT.md"

SELF_IMPORT_RE = re.compile(r"^\s*from\s+\.([A-Za-z0-9_]+)\.", re.MULTILINE)


def build_stub(module_name: str) -> str:
    return (
        '"""\n'
        f"SigmaOS Modular Shim for {module_name}.py\n"
        '"""\n'
        "__all__: list[str] = []\n"
    )


def main() -> int:
    changed: list[Path] = []
    scanned = 0
    for package_dir in sorted(SYSTEM_API_DIR.iterdir()):
        if not package_dir.is_dir():
            continue
        init_file = package_dir / "__init__.py"
        if not init_file.exists():
            continue
        scanned += 1
        text = init_file.read_text(encoding="utf-8")
        if SELF_IMPORT_RE.search(text):
            init_file.write_text(build_stub(package_dir.name), encoding="utf-8")
            changed.append(init_file)

    report_lines = [
        "# System API Shim Repair Report",
        "",
        f"- Scanned packages: **{scanned}**",
        f"- Repaired shims: **{len(changed)}**",
        "",
        "## Repaired Files",
        "",
    ]
    if changed:
        report_lines.extend(f"- `{p.relative_to(ROOT).as_posix()}`" for p in changed)
    else:
        report_lines.append("- None")
    report_lines.append("")
    REPORT_FILE.write_text("\n".join(report_lines), encoding="utf-8")

    print(f"Scanned: {scanned}")
    print(f"Repaired: {len(changed)}")
    print(f"Report: {REPORT_FILE}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
