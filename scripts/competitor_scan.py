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

WATCHLIST = [
    {"name": "Redox OS", "repo": "redox-os/redox", "focus": "Microkernel Rust OS & Userspace Drivers"},
    {"name": "seL4 Microkernel", "repo": "seL4/seL4", "focus": "Formal Verification & Capability Security"},
    {"name": "Tock OS", "repo": "tock/tock", "focus": "Capsule Drivers & Embedded Memory Protection"},
    {"name": "Fuchsia Zircon", "repo": "fuchsia-mirror/fuchsia", "focus": "Component Isolation & Capability Manifests"},
    {"name": "Wasmtime / WASI", "repo": "bytecodealliance/wasmtime", "focus": "WASM Sandboxed Application Runtime"},
    {"name": "smoltcp Network Stack", "repo": "smoltcp-rs/smoltcp", "focus": "Event-Driven no_std Networking"}
]

def scan_competitors():
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

    output_path = os.path.join("build", "competitor_scan_report.json")
    os.makedirs("build", exist_ok=True)
    with open(output_path, "w") as f:
        json.dump(results, f, indent=2)

    print(f"SUCCESS: Scanned {len(WATCHLIST)} upstream projects. Report written to {output_path}.")

if __name__ == "__main__":
    scan_competitors()
