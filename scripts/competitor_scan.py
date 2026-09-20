#!/usr/bin/env python3
"""
SigmaOS Competitor & Upstream Innovation Scanner
Periodically checks releases and updates from key open-source OS projects
(Redox, seL4, Tock OS, Fuchsia, Linux Minimal, WASI/Wasmtime)
and generates summary issues or report digests for continuous OS improvement.
"""

import json
import urllib.request
import os
import sys

WATCHLIST = [
    {"name": "Redox OS", "repo": "redox-os/redox", "focus": "Microkernel Rust OS & Userspace Drivers"},
    {"name": "seL4 Microkernel", "repo": "seL4/seL4", "focus": "Formal Verification & Capability Security"},
    {"name": "Tock OS", "repo": "tock/tock", "focus": "Capsule Drivers & Embedded Memory Protection"},
    {"name": "Fuchsia Zircon", "repo": "fuchsia-mirror/fuchsia", "focus": "Component Isolation & Capability Manifests"},
    {"name": "Wasmtime / WASI", "repo": "bytecodealliance/wasmtime", "focus": "WASM Sandboxed Application Runtime"},
    {"name": "smoltcp Network Stack", "repo": "smoltcp-rs/smoltcp", "focus": "Event-Driven no_std Networking"}
]

def scan_competitors(create_issue: bool = False):
    print(":: Scanning Open-Source OS Upstream Repositories for Continuous Absorption...")
    results = []

    for item in WATCHLIST:
        repo_name = item["repo"]
        url = f"https://api.github.com/repos/{repo_name}/releases/latest"
        req = urllib.request.Request(url, headers={"User-Agent": "SigmaOS-Competitor-Scan"})
        try:
            with urllib.request.urlopen(req, timeout=5) as response:
                if response.status == 200:
                    data = json.loads(response.read().decode())
                    tag_name = data.get("tag_name", "N/A")
                    published_at = data.get("published_at", "N/A")
                    results.append({
                        "name": item["name"],
                        "repo": repo_name,
                        "focus": item["focus"],
                        "latest_release": tag_name,
                        "published_at": published_at,
                        "status": "Active"
                    })
        except Exception as e:
            results.append({
                "name": item["name"],
                "repo": repo_name,
                "focus": item["focus"],
                "latest_release": "Unknown / Rate-limited",
                "published_at": "N/A",
                "status": f"Checked ({e})"
            })

    output_dir = "build"
    os.makedirs(output_dir, exist_ok=True)

    json_path = os.path.join(output_dir, "competitor_scan_report.json")
    with open(json_path, "w") as f:
        json.dump(results, f, indent=2)

    md_path = os.path.join(output_dir, "competitor_scan_report.md")
    md_content = "# SigmaOS Open-Source OS Competitor & Upstream Innovation Digest\n\n"
    md_content += "| Project | Focus | Latest Release | Published At | Status |\n"
    md_content += "| --- | --- | --- | --- | --- |\n"
    for r in results:
        md_content += f"| **{r['name']}** (`{r['repo']}`) | {r['focus']} | `{r['latest_release']}` | {r['published_at']} | {r['status']} |\n"

    with open(md_path, "w") as f:
        f.write(md_content)

    print(f"SUCCESS: Scanned {len(WATCHLIST)} upstream projects. JSON report: {json_path}, Markdown report: {md_path}")

    if create_issue:
        token = os.environ.get("GITHUB_TOKEN")
        repo = os.environ.get("GITHUB_REPOSITORY")
        if token and repo:
            issue_url = f"https://api.github.com/repos/{repo}/issues"
            issue_data = {
                "title": "Monthly Upstream OS Competitor Innovation Digest",
                "body": md_content,
                "labels": ["enhancement", "documentation"]
            }
            req = urllib.request.Request(
                issue_url,
                data=json.dumps(issue_data).encode("utf-8"),
                headers={
                    "Authorization": f"token {token}",
                    "Accept": "application/vnd.github.v3+json",
                    "User-Agent": "SigmaOS-Competitor-Scan"
                },
                method="POST"
            )
            try:
                with urllib.request.urlopen(req) as resp:
                    print(f"Created GitHub Issue for upstream digest (HTTP {resp.status}).")
            except Exception as e:
                print(f"Notice: Could not post GitHub issue automatically: {e}")
        else:
            print("Notice: GITHUB_TOKEN or GITHUB_REPOSITORY not configured. Markdown report available in build/competitor_scan_report.md.")

if __name__ == "__main__":
    create_issue_flag = "--issue" in sys.argv
    scan_competitors(create_issue=create_issue_flag)
