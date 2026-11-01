#!/usr/bin/env python3
"""
Generate COMPONENTS.md from COMPONENTS.toml

This script converts component metadata from TOML format into a Markdown matrix
suitable for documentation. It creates a formatted table with component information
including name, description, owner, status, and dependencies.
"""

import sys
import toml
from pathlib import Path
from typing import Dict, List, Any
from datetime import datetime


def load_components_toml(toml_path: str) -> Dict[str, Any]:
    """Load components from TOML file."""
    try:
        with open(toml_path, 'r') as f:
            return toml.load(f)
    except FileNotFoundError:
        print(f"Error: {toml_path} not found", file=sys.stderr)
        sys.exit(1)
    except toml.TomlDecodeError as e:
        print(f"Error parsing TOML: {e}", file=sys.stderr)
        sys.exit(1)


def generate_components_md(components_data: Dict[str, Any]) -> str:
    """Generate Markdown from components data."""
    md_lines = []
    
    # Header
    md_lines.append("# Component Matrix")
    md_lines.append("")
    md_lines.append(f"*Auto-generated on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*")
    md_lines.append("")
    md_lines.append("Central registry of SigmaOS components with their metadata and dependencies.")
    md_lines.append("")
    
    # Extract components
    components = components_data.get('components', {})
    if not components:
        md_lines.append("No components found in registry.")
        return '\n'.join(md_lines)
    
    # Table header
    md_lines.append("| Component | Category | Owner | Status | Description |")
    md_lines.append("|-----------|----------|-------|--------|-------------|")
    
    # Sort components by name
    for comp_name in sorted(components.keys()):
        comp = components[comp_name]
        
        name = comp.get('name', comp_name)
        category = comp.get('category', 'uncategorized')
        owner = comp.get('owner', 'TBD')
        status = comp.get('status', 'unknown')
        description = comp.get('description', '')
        
        # Escape pipes in description
        description = description.replace('|', '\\|')
        
        md_lines.append(f"| {name} | {category} | {owner} | {status} | {description} |")
    
    md_lines.append("")
    
    # Dependencies section
    md_lines.append("## Dependencies")
    md_lines.append("")
    
    has_dependencies = False
    for comp_name in sorted(components.keys()):
        comp = components[comp_name]
        deps = comp.get('dependencies', [])
        
        if deps:
            has_dependencies = True
            md_lines.append(f"### {comp_name}")
            md_lines.append("")
            for dep in deps:
                md_lines.append(f"- {dep}")
            md_lines.append("")
    
    if not has_dependencies:
        md_lines.append("No documented dependencies.")
        md_lines.append("")
    
    # Status legend
    md_lines.append("## Status Legend")
    md_lines.append("")
    md_lines.append("- **stable**: Production-ready, API stable")
    md_lines.append("- **beta**: Feature complete, may have bugs")
    md_lines.append("- **alpha**: Early development, API may change")
    md_lines.append("- **experimental**: Research/proof-of-concept")
    md_lines.append("- **deprecated**: Superseded, plan migration")
    md_lines.append("")
    
    return '\n'.join(md_lines)


def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print("Usage: generate-components-md.py <input.toml> [output.md]", file=sys.stderr)
        sys.exit(1)
    
    input_toml = sys.argv[1]
    output_md = sys.argv[2] if len(sys.argv) > 2 else 'COMPONENTS.md'
    
    # Load and generate
    components_data = load_components_toml(input_toml)
    markdown = generate_components_md(components_data)
    
    # Write output
    with open(output_md, 'w') as f:
        f.write(markdown)
    
    print(f"Generated {output_md} from {input_toml}")


if __name__ == '__main__':
    main()
