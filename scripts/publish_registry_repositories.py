#!/usr/bin/env python3
"""Publish prepared public SigmaOS checkouts declared in COMPONENTS.toml.

This command never invents repository contents. It is a read-only plan unless
--apply is supplied, and it refuses dirty, ambiguous, or mismatched checkouts.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
import time
import tomllib
from pathlib import Path
from urllib.parse import urlsplit


def run(args: tuple[str, ...], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(args, text=True, capture_output=True, check=False)
    if check and result.returncode:
        detail = result.stderr.strip() or result.stdout.strip()
        raise RuntimeError(f"{' '.join(args)} failed: {detail}")
    return result


def normalize_remote(remote: str) -> str:
    if remote.startswith("git@github.com:"):
        host, path = "github.com", remote.removeprefix("git@github.com:")
    else:
        parsed = urlsplit(remote)
        host, path = parsed.hostname or "", parsed.path
    path = path.strip("/")
    if path.endswith(".git"):
        path = path[:-4]
    return f"{host.lower()}/{path}".casefold() if host and path else ""


def remote_path(remote: str) -> str:
    """Return the URL path with its original repository-name casing."""
    if remote.startswith("git@github.com:"):
        path = remote.removeprefix("git@github.com:")
    else:
        path = urlsplit(remote).path
    path = path.strip("/")
    return path[:-4] if path.endswith(".git") else path


def github_names() -> tuple[str, set[str]]:
    owner = run(("gh", "api", "user", "--jq", ".login")).stdout.strip()
    repos = run(("gh", "api", "--paginate", "user/repos", "--jq", ".[].full_name"))
    return owner, {line.strip().casefold() for line in repos.stdout.splitlines() if line.strip()}


def git(repo: Path, *args: str, check: bool = True) -> str:
    return run(("git", "-C", str(repo), *args), check=check).stdout.strip()


def candidates(root: Path, owner: str, existing: set[str]) -> tuple[list[tuple[str, Path, str]], list[str]]:
    data = tomllib.loads((root / "COMPONENTS.toml").read_text(encoding="utf-8"))
    by_url: dict[str, list[dict[str, object]]] = {}
    for item in data.get("component", []):
        url = normalize_remote(str(item.get("repo", "")))
        if url.startswith(f"github.com/{owner}/".casefold()):
            by_url.setdefault(url, []).append(item)

    plans: list[tuple[str, Path, str]] = []
    skipped: list[str] = []
    for url, entries in sorted(by_url.items()):
        full_name = remote_path(str(entries[0].get("repo", "")))
        if not full_name.casefold().startswith(f"{owner}/sigmaos-".casefold()):
            continue
        if full_name.casefold() in existing:
            continue
        names = sorted({str(entry.get("name", "")) for entry in entries if entry.get("name")})
        public = [entry for entry in entries if entry.get("visibility") == "public"]
        if not public:
            continue
        if len(names) != 1:
            skipped.append(f"{full_name}: ambiguous names {', '.join(names)}")
            continue
        if any(entry.get("visibility") not in (None, "public") for entry in entries):
            skipped.append(f"{full_name}: conflicting visibility declarations")
            continue
        path = root.parent / names[0]
        if not (path / ".git").exists():
            continue
        head = git(path, "rev-parse", "--verify", "HEAD", check=False)
        branch = git(path, "branch", "--show-current")
        if not head or branch != "main":
            skipped.append(f"{full_name}: checkout must have a commit on main")
            continue
        if git(path, "status", "--porcelain"):
            skipped.append(f"{full_name}: dirty checkout preserved")
            continue
        origin = git(path, "remote", "get-url", "origin", check=False)
        if origin and normalize_remote(origin) != url:
            skipped.append(f"{full_name}: origin does not match registry URL")
            continue
        plans.append((full_name, path, head))
    return plans, skipped


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="create missing public repos and push prepared main branches")
    parser.add_argument("--limit", type=int, help="process at most this many repositories")
    parser.add_argument("--name", action="append", help="select a component checkout by exact name (repeatable)")
    args = parser.parse_args()
    if args.limit is not None and args.limit < 1:
        parser.error("--limit must be positive")
    if args.apply and args.limit is None and not args.name:
        parser.error("--apply requires an explicit --name or --limit")

    root = Path(__file__).resolve().parent.parent
    try:
        owner, existing = github_names()
        plan, skipped = candidates(root, owner, existing)
        if args.name:
            selected = set(args.name)
            plan = [item for item in plan if item[1].name in selected]
            missing = selected - {item[1].name for item in plan}
            for name in sorted(missing):
                print(f"NOT SELECTED: {name} (not an eligible unpublished checkout)")
        if args.limit:
            plan = plan[:args.limit]
        mode = "APPLY" if args.apply else "DRY RUN"
        print(f"{mode}: {len(plan)} prepared public repositories; owner={owner}")
        for full_name, path, head in plan:
            print(f"  {full_name} <- {path} @ {head[:12]}")
        for item in skipped:
            print(f"SKIP: {item}")
        if not args.apply:
            print("No changes made. Pass --apply to create and push these repositories.")
            return 0

        for index, (full_name, path, _head) in enumerate(plan):
            print(f"[{index + 1}/{len(plan)}] Creating {full_name}", flush=True)
            run(("gh", "repo", "create", full_name, "--public"))
            origin = git(path, "remote", "get-url", "origin", check=False)
            if not origin:
                git(path, "remote", "add", "origin", f"https://github.com/{full_name}.git")
            git(path, "push", "origin", "HEAD:main")
            print(f"PUBLISHED: {full_name}", flush=True)
            if index + 1 < len(plan):
                time.sleep(2)
        return 0
    except (OSError, RuntimeError, tomllib.TOMLDecodeError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
