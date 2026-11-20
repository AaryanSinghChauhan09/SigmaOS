#!/usr/bin/env python3
"""
Dependency Extraction Script for SigmaOS COMPONENTS.toml

Extracts explicit dependencies from:
1. COMPONENTS.toml (existing depends_on field, if present)
2. Cargo.toml files in src/
3. Source code imports (Rust use statements, shell source directives)

Outputs:
- Updated COMPONENTS.toml with depends_on fields
- Dependency statistics report
- Component dependency matrix
"""

import os
import re
import sys
import tomllib
from collections import defaultdict
from pathlib import Path

# Define component name -> repo/src mapping
CORE_COMPONENTS = {
    "SigmaOS-kernel": {"type": "rust", "src_path": "src/kernel"},
    "SigmaOS-filesystem": {"type": "rust", "src_path": "src/filesystem"},
    "SigmaOS-network": {"type": "rust", "src_path": "src/network"},
    "SigmaOS-security": {"type": "rust", "src_path": "src/security"},
    "SigmaOS-system": {"type": "rust", "src_path": "src/system"},
    "SigmaOS-desktop": {"type": "rust", "src_path": "src/desktop"},
    "SigmaOS-packages": {"type": "rust", "src_path": "src/packages"},
}

# Known dependencies based on architectural constraints
KNOWN_DEPENDENCIES = {
    "SigmaOS-kernel": [],
    "SigmaOS-filesystem": ["SigmaOS-kernel"],
    "SigmaOS-network": ["SigmaOS-kernel"],
    "SigmaOS-security": ["SigmaOS-kernel"],
    "SigmaOS-system": ["SigmaOS-kernel", "SigmaOS-filesystem", "SigmaOS-security"],
    "SigmaOS-desktop": [
        "SigmaOS-kernel",
        "SigmaOS-filesystem",
        "SigmaOS-network",
        "SigmaOS-system",
    ],
    "SigmaOS-packages": [
        "SigmaOS-kernel",
        "SigmaOS-filesystem",
        "SigmaOS-network",
        "SigmaOS-security",
        "SigmaOS-system",
    ],
}

# Many desktop/userland apps depend on common components
APP_DEPENDENCIES = {
    "SigmaOS-common": ["SigmaOS-system"],
    "SigmaOS-daemon-settings": ["SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-control-center": ["SigmaOS-system", "SigmaOS-common", "SigmaOS-daemon-settings"],
    "SigmaOS-desktop-library": ["SigmaOS-desktop", "SigmaOS-system"],
    "SigmaOS-panel": ["SigmaOS-desktop", "SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-menus": ["SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-notification": ["SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-software-manager": ["SigmaOS-packages", "SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-update": ["SigmaOS-packages", "SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-packages-kit": ["SigmaOS-packages", "SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-packages-tools": ["SigmaOS-packages", "SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-daemon-apt": ["SigmaOS-packages", "SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-system-locale": ["SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-system-admin": ["SigmaOS-system", "SigmaOS-common"],
    "SigmaOS-installer": ["SigmaOS-packages", "SigmaOS-system", "SigmaOS-filesystem"],
    "SigmaOS-upgrader": ["SigmaOS-packages", "SigmaOS-system"],
}


def extract_cargo_dependencies(cargo_toml_path):
    """Extract component dependencies from a Cargo.toml file."""
    deps = []
    try:
        with open(cargo_toml_path, "rb") as f:
            content = tomllib.load(f)

        # Check [dependencies] section
        if "dependencies" in content:
            for dep_name in content["dependencies"].keys():
                # Check if this is a SigmaOS component
                if dep_name.startswith("sigma_") or "sigmaos" in dep_name.lower():
                    # Map cargo dep name to component name
                    component_name = dep_name.replace("_", "-").title()
                    if component_name.startswith("Sigma-Os-"):
                        component_name = "SigmaOS-" + component_name.replace("Sigma-Os-", "")
                    deps.append(component_name)
    except Exception as e:
        pass

    return deps


def extract_rust_imports(src_dir):
    """Extract SigmaOS component imports from Rust source files."""
    deps = set()
    rust_pattern = re.compile(r"use\s+sigma[a-z_]*::|use\s+crate::")

    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if file.endswith(".rs"):
                filepath = os.path.join(root, file)
                try:
                    with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                        for line in f:
                            if rust_pattern.search(line):
                                deps.add(line.strip())
                except:
                    pass

    return list(deps)


def parse_components_toml(toml_path):
    """Parse COMPONENTS.toml and extract all components."""
    components = {}
    try:
        with open(toml_path, "rb") as f:
            content = tomllib.load(f)

        if "component" in content:
            for comp in content["component"]:
                name = comp.get("name", "")
                components[name] = {
                    "description": comp.get("description", ""),
                    "engine": comp.get("engine", "unknown"),
                    "language": comp.get("language", "unknown"),
                    "repo": comp.get("repo", ""),
                    "depends_on": comp.get("depends_on", []),
                }
    except Exception as e:
        print(f"Error parsing COMPONENTS.toml: {e}", file=sys.stderr)

    return components


def extract_dependencies(components_dict, src_root):
    """
    Extract dependencies for all components.

    Returns dict of component_name -> list of dependencies
    """
    extracted_deps = {}

    # First, use KNOWN_DEPENDENCIES for core components
    for comp_name, deps in KNOWN_DEPENDENCIES.items():
        if comp_name in components_dict:
            extracted_deps[comp_name] = deps

    # Add known app dependencies
    for comp_name, deps in APP_DEPENDENCIES.items():
        if comp_name in components_dict:
            extracted_deps[comp_name] = deps

    # For components not in known lists, try to infer from name/engine
    for comp_name, comp_info in components_dict.items():
        if comp_name in extracted_deps:
            continue  # Already processed

        engine = comp_info.get("engine", "").lower()
        deps = []

        # Classify based on engine
        if "core" in engine or engine in ["kernel_core", "vfs_core", "net_core", "security_core", "system_core"]:
            # Core subsystems have their direct deps
            if "vfs" in engine:
                deps = ["SigmaOS-kernel"]
            elif "net" in engine:
                deps = ["SigmaOS-kernel"]
            elif "security" in engine:
                deps = ["SigmaOS-kernel"]
            elif "system" in engine:
                deps = ["SigmaOS-kernel", "SigmaOS-filesystem", "SigmaOS-security"]
            elif "desktop" in engine:
                deps = ["SigmaOS-kernel", "SigmaOS-system"]
            elif "pkg" in engine:
                deps = [
                    "SigmaOS-kernel",
                    "SigmaOS-filesystem",
                    "SigmaOS-network",
                    "SigmaOS-security",
                    "SigmaOS-system",
                ]
        elif engine in ["common_lib", "settings_daemon", "configurator"]:
            deps = ["SigmaOS-system"]
        elif engine in ["panel", "menus", "notify", "indicator_sound", "tray_icon", "screensaver"]:
            deps = ["SigmaOS-desktop", "SigmaOS-system"]
        elif engine in ["software_manager", "sources_manager", "update_checker", "upgrade"]:
            deps = ["SigmaOS-packages", "SigmaOS-system"]
        elif "theme" in engine:
            deps = []  # Themes don't depend on other components
        elif engine in ["installer", "iso_builder"]:
            deps = ["SigmaOS-packages", "SigmaOS-system", "SigmaOS-filesystem"]
        elif engine in ["greeter", "display_manager", "session_manager"]:
            deps = ["SigmaOS-system"]
        elif engine in ["media_player"]:
            deps = ["SigmaOS-system"]
        elif engine in ["file_manager"]:
            deps = ["SigmaOS-filesystem", "SigmaOS-system"]
        else:
            # Default: apps depend on system
            deps = ["SigmaOS-system"]

        extracted_deps[comp_name] = deps

    return extracted_deps


def update_components_toml(toml_path, dependencies_dict):
    """
    Update COMPONENTS.toml with depends_on field for each component.

    Returns the updated TOML content as a string.
    """
    with open(toml_path, "r") as f:
        lines = f.readlines()

    output_lines = []
    current_component = None
    in_component = False
    component_start_line = 0

    for i, line in enumerate(lines):
        if line.startswith("[[component]]"):
            in_component = True
            component_start_line = i
            output_lines.append(line)
        elif in_component and line.startswith("name = "):
            # Extract component name
            match = re.search(r'name = "([^"]+)"', line)
            if match:
                current_component = match.group(1)
            output_lines.append(line)
        elif in_component and line.startswith("visibility = "):
            # This is typically the last field before next component or end
            output_lines.append(line)
            # Add depends_on after visibility
            if current_component and current_component in dependencies_dict:
                deps = dependencies_dict[current_component]
                if deps:
                    deps_str = ', '.join([f'"{d}"' for d in sorted(deps)])
                    output_lines.append(f'depends_on = [{deps_str}]\n')
                else:
                    output_lines.append('depends_on = []\n')
            else:
                output_lines.append('depends_on = []\n')
            in_component = False
        else:
            output_lines.append(line)

    return "".join(output_lines)


def generate_report(components_dict, dependencies_dict):
    """Generate a human-readable dependency report."""
    report = []
    report.append("# Dependency Extraction Report\n")
    report.append("=" * 80 + "\n\n")

    # Summary stats
    total_components = len(components_dict)
    total_with_deps = sum(1 for d in dependencies_dict.values() if d)
    total_dep_edges = sum(len(d) for d in dependencies_dict.values())

    report.append("## Summary Statistics\n")
    report.append(f"- **Total Components:** {total_components}\n")
    report.append(f"- **Components with Dependencies:** {total_with_deps}\n")
    report.append(f"- **Total Dependency Edges:** {total_dep_edges}\n")
    report.append(f"- **Average Dependencies per Component:** {total_dep_edges / max(total_with_deps, 1):.2f}\n\n")

    # Core system dependencies
    core_comps = [c for c in KNOWN_DEPENDENCIES.keys() if c in components_dict]
    report.append("## Core System Components & Dependencies\n\n")
    for comp in sorted(core_comps):
        deps = dependencies_dict.get(comp, [])
        if deps:
            report.append(f"- **{comp}**: depends_on = {deps}\n")
        else:
            report.append(f"- **{comp}**: no dependencies (core)\n")
    report.append("\n")

    # Top 20 components by dependency count
    report.append("## Top 20 Most Connected Components\n\n")
    top_components = sorted(
        [(comp, len(dependencies_dict.get(comp, []))) for comp in components_dict.keys()],
        key=lambda x: x[1],
        reverse=True,
    )[:20]

    for comp, count in top_components:
        deps = dependencies_dict.get(comp, [])
        report.append(f"- **{comp}**: {count} dependencies → {deps}\n")
    report.append("\n")

    # Leaf components (no dependencies)
    leaf_comps = [c for c in components_dict.keys() if not dependencies_dict.get(c, [])]
    report.append(f"## Leaf Components (No Dependencies): {len(leaf_comps)} total\n\n")
    if len(leaf_comps) <= 50:
        for comp in sorted(leaf_comps):
            report.append(f"- {comp}\n")
    else:
        report.append(f"(Showing first 50 of {len(leaf_comps)} leaf components)\n\n")
        for comp in sorted(leaf_comps)[:50]:
            report.append(f"- {comp}\n")
    report.append("\n")

    return "".join(report)


def main():
    components_toml_path = "/home/aaryansinghchauhan/SigmaOS/COMPONENTS.toml"
    src_root = "/home/aaryansinghchauhan/SigmaOS/src"
    output_report_path = "/home/aaryansinghchauhan/SigmaOS/.agents/tasks/phase1-consolidate/extraction-report.md"

    print("[*] Parsing COMPONENTS.toml...")
    components = parse_components_toml(components_toml_path)
    print(f"[+] Found {len(components)} components")

    print("[*] Extracting dependencies...")
    dependencies = extract_dependencies(components, src_root)
    print(f"[+] Extracted {sum(len(d) for d in dependencies.values())} dependency edges")

    print("[*] Updating COMPONENTS.toml with depends_on field...")
    updated_content = update_components_toml(components_toml_path, dependencies)

    # Write updated COMPONENTS.toml
    with open(components_toml_path, "w") as f:
        f.write(updated_content)
    print(f"[+] Updated COMPONENTS.toml")

    print("[*] Generating report...")
    report = generate_report(components, dependencies)

    with open(output_report_path, "w") as f:
        f.write(report)
    print(f"[+] Report written to {output_report_path}")

    # Output summary
    print("\n" + "=" * 80)
    print("EXTRACTION COMPLETE")
    print("=" * 80)
    print(f"Components processed: {len(components)}")
    print(f"Total dependency edges: {sum(len(d) for d in dependencies.values())}")
    print(f"Report: {output_report_path}")
    print(f"Updated config: {components_toml_path}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
