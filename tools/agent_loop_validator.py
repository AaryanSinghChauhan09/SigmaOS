#!/usr/bin/env python3
"""
tools/agent_loop_validator.py
Validates the repeatable agent operating loop artifacts in SigmaOS:
- Verifies .agents/registry.toml exists and contains required agent roles.
- Verifies .agents/tasks/open contains structured task files.
- Verifies .agents/runbooks/ contains required runbooks.
- Verifies reports/ evidence directories and files.
"""

import os
import sys

def validate_agent_operating_loop():
    errors = []

    # Check registry.toml
    registry_path = ".agents/registry.toml"
    if not os.path.exists(registry_path):
        errors.append(f"Missing {registry_path}")
    else:
        with open(registry_path, "r", encoding="utf-8") as f:
            content = f.read()
            required_agents = ["steward", "sentinel", "boot", "capability-auditor", "kernel", "package-compat", "docs"]
            for agent in required_agents:
                if agent not in content:
                    errors.append(f"Agent keyword '{agent}' missing in {registry_path}")

    # Check tasks directory
    tasks_open_dir = ".agents/tasks/open"
    if not os.path.exists(tasks_open_dir):
        errors.append(f"Missing directory {tasks_open_dir}")
    else:
        task_files = [f for f in os.listdir(tasks_open_dir) if f.endswith(".toml")]
        if len(task_files) < 3:
            errors.append(f"Expected at least 3 open task files in {tasks_open_dir}, found {len(task_files)}")

    # Check runbooks directory
    runbooks_dir = ".agents/runbooks"
    if not os.path.exists(runbooks_dir):
        errors.append(f"Missing directory {runbooks_dir}")
    else:
        required_runbooks = ["build-test.md", "qemu-boot.md", "capability-audit.md"]
        for rb in required_runbooks:
            if not os.path.exists(os.path.join(runbooks_dir, rb)):
                errors.append(f"Missing runbook: {rb}")

    # Check reports directory
    reports_dir = "reports"
    if not os.path.exists(reports_dir):
        errors.append(f"Missing directory {reports_dir}")

    if errors:
        print("=== Agent Operating Loop Validation FAILED ===")
        for err in errors:
            print(f"  - {err}")
        return False
    else:
        print("=== Agent Operating Loop Validation PASSED ===")
        print("All required registry entries, task files, runbooks, and report structures verified.")
        return True

if __name__ == "__main__":
    success = validate_agent_operating_loop()
    sys.exit(0 if success else 1)
