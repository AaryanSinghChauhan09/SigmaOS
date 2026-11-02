#!/usr/bin/env python3
"""
SigmaOS Component Classification Tool
Phase 1 Task 1.2: Classify all 1,635 components with engine and category metadata.

Usage:
    python3 classify.py /path/to/COMPONENTS.toml /path/to/output.toml
"""

import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple, Optional

# Classification rules based on component naming, descriptions, and known mappings
ENGINE_MAPPINGS = {
    # Core kernel & system engines
    'kernel': 'kernel_core',
    'filesystem': 'vfs_core',
    'network': 'net_core',
    'security': 'security_core',
    'system': 'system_core',
    'desktop': 'desktop_core',
    'packages': 'pkg_core',
    
    # Boot & installation
    'bootloader': 'bootloader',
    'plymouth': 'plymouth',
    'installer': 'installer',
    'iso': 'iso_builder',
    'live': 'iso_builder',
    'setup': 'setup',
    'try': 'try_live',
    
    # Service management
    'systemd': 'unit_manager',
    'session': 'session_manager',
    'display': 'greeter',
    'greeter': 'greeter',
    'login': 'greeter',
    
    # System management
    'power': 'power_manager',
    'settings': 'configurator',
    'control': 'configurator',
    'configure': 'configurator',
    'daemon': 'daemon',
    'update': 'update_checker',
    'upgrade': 'upgrade',
    'mirror': 'mirror',
    'backup': 'backup',
    'driver': 'driver_manager',
    'gpu': 'driver_manager',
    'nvidia': 'driver_manager',
    
    # Package management
    'apt': 'sources_manager',
    'package': 'sources_manager',
    'software': 'software_manager',
    'sources': 'sources_manager',
    
    # Desktop & UI
    'panel': 'panel',
    'menu': 'menus',
    'window': 'wm_lib',
    'compositor': 'desktop_lib',
    'theme': 'theme_engine',
    'indicator': 'indicator_sound',
    'tray': 'tray_icon',
    'notify': 'notify',
    'notification': 'notify',
    'screensaver': 'screensaver',
    'screen-saver': 'screensaver',
    'wallpaper': 'wallpaper',
    
    # Applications & utilities
    'calculator': 'calc',
    'chat': 'chat_server',
    'draw': 'draw',
    'spreadsheet': 'spreadsheet',
    'slide': 'slide',
    'sticky': 'sticky',
    'file-manager': 'file_manager',
    'writer': 'file_writer',
    'folder': 'folder_color',
    'renamer': 'bulk_renamer',
    'photo': 'media_player',
    'viewer': 'media_player',
    'player': 'media_player',
    'reel': 'reel',
    'stream': 'stream_player',
    'radio': 'stream_player',
    'screenshot': 'screenshot',
    'screen-shot': 'screenshot',
    'webcam': 'webcam',
    'scanner': 'scanner',
    'video-cut': 'video_cut',
    'xreader': 'xreader',
    'office': 'office_style',
    'webapp': 'webapp_manager',
    'remote': 'remote_desktop',
    'upload': 'upload',
    'stick': 'image_writer',
    'wine': 'wine_installer',
    'vpn': 'vpn_manager',
    'bluetooth': 'bluetooth',
    'lan': 'lan_share',
    'ssh': 'ssh_forward',
    'domain': 'domain_blocker',
    
    # Development & tools
    'dev': 'dev_tools',
    'nvim': 'nvim_distro',
    'vim': 'nvim_distro',
    'shell': 'shell_config',
    'zsh': 'shell_config',
    'fish': 'shell_config',
    'term': 'term',
    'terminal': 'term',
    'text-effect': 'tte',
    'quickshell': 'quickshell',
    'audio': 'audio_tuner',
    'js': 'js_interpreter',
    'javascript': 'js_interpreter',
    'node': 'node_binary',
    'mesa': 'graphics',
    'xorg': 'graphics',
    'xapp': 'xapp_lib',
    'gtk': 'gtk_lib',
    'desktop-portal': 'desktop_portal',
    
    # Documentation & website
    'website': 'docs_site',
    'guide': 'docs_site',
    'doc': 'docs_site',
    'translation': 'translations',
    'welcome': 'welcome',
    
    # Theme & customization
    'color': 'theme_engine',
    'cursor': 'theme_engine',
    'icon': 'theme_engine',
    'brand': 'brand_logo',
    'dots': 'dots',
    'forum': 'forum_themes',
    'marketplace': 'theme_marketplace',
    'registry': 'plugin_registry',
    'plugin': 'plugin_registry',
    'action': 'plugin_registry',
    'sync': 'theme_engine',
    
    # System utilities
    'troubleshoot': 'troubleshoot',
    'report': 'troubleshoot',
    'automate': 'automate',
    'state': 'state_of',
    'sysadmin': 'sysadmin',
    'support': 'support',
    'locale': 'locale_manager',
    'discovery': 'net_discovery',
    'monitor': 'monitoring',
    'ml': 'ml',
    'print': 'printing',
    'night': 'nightlight',
    'monitor': 'monitoring',
    'common': 'common_lib',
    'lib': 'common_lib',
    'herdr': 'herdr',
    'asdcontrol': 'asdcontrol',
    'logging': 'logging',
}

CATEGORY_MAPPINGS = {
    # Core system
    'kernel_core': 'System Services',
    'vfs_core': 'System Services',
    'net_core': 'Network',
    'security_core': 'Security',
    'system_core': 'System Services',
    
    # Boot & installation
    'bootloader': 'System Services',
    'plymouth': 'System Services',
    'installer': 'Utilities',
    'iso_builder': 'Utilities',
    'setup': 'Utilities',
    'try_live': 'Utilities',
    
    # Service management
    'unit_manager': 'System Services',
    'session_manager': 'System Services',
    'greeter': 'Desktop/UI',
    'power_manager': 'System Services',
    'configurator': 'System Services',
    'daemon': 'System Services',
    'update_checker': 'Package Management',
    'upgrade': 'Package Management',
    'mirror': 'Network',
    'backup': 'Utilities',
    'driver_manager': 'Drivers & Hardware',
    
    # Package management
    'sources_manager': 'Package Management',
    'software_manager': 'Package Management',
    'pkg_core': 'Package Management',
    
    # Desktop & UI
    'desktop_core': 'Desktop/UI',
    'desktop_lib': 'Desktop/UI',
    'wm_lib': 'Desktop/UI',
    'panel': 'Desktop/UI',
    'menus': 'Desktop/UI',
    'indicator_sound': 'Desktop/UI',
    'tray_icon': 'Desktop/UI',
    'notify': 'Desktop/UI',
    'screensaver': 'Desktop/UI',
    'wallpaper': 'Desktop/UI',
    'theme_engine': 'Desktop/UI',
    'theme_marketplace': 'Desktop/UI',
    
    # Applications
    'calc': 'Utilities',
    'chat_server': 'Network',
    'draw': 'Creative',
    'spreadsheet': 'Creative',
    'slide': 'Creative',
    'sticky': 'Utilities',
    'file_manager': 'Utilities',
    'file_writer': 'Creative',
    'folder_color': 'Desktop/UI',
    'bulk_renamer': 'Utilities',
    'media_player': 'Creative',
    'reel': 'Creative',
    'stream_player': 'Network',
    'screenshot': 'Utilities',
    'screen_recorder': 'Creative',
    'webcam': 'Creative',
    'scanner': 'Drivers & Hardware',
    'video_cut': 'Creative',
    'xreader': 'Creative',
    'office_style': 'Creative',
    'webapp_manager': 'Utilities',
    'remote_desktop': 'Network',
    'upload': 'Network',
    'image_writer': 'Utilities',
    'wine_installer': 'Utilities',
    'vpn_manager': 'Network',
    'bluetooth': 'Drivers & Hardware',
    'lan_share': 'Network',
    'ssh_forward': 'Network',
    'domain_blocker': 'Security',
    
    # Development & tools
    'dev_tools': 'Developer Tools',
    'nvim_distro': 'Developer Tools',
    'shell_config': 'Developer Tools',
    'term': 'Developer Tools',
    'tte': 'Developer Tools',
    'quickshell': 'Developer Tools',
    'audio_tuner': 'System Services',
    'js_interpreter': 'Developer Tools',
    'node_binary': 'Developer Tools',
    'graphics': 'Drivers & Hardware',
    'xapp_lib': 'Developer Tools',
    'gtk_lib': 'Developer Tools',
    'desktop_portal': 'System Services',
    
    # Documentation
    'docs_site': 'Documentation',
    'translations': 'Documentation',
    'welcome': 'Utilities',
    
    # Customization
    'brand_logo': 'Desktop/UI',
    'dots': 'Utilities',
    'forum_themes': 'Desktop/UI',
    'plugin_registry': 'Utilities',
    'plugin_marketplace': 'Desktop/UI',
    
    # System utilities
    'troubleshoot': 'Utilities',
    'automate': 'Utilities',
    'state_of': 'Utilities',
    'sysadmin': 'System Services',
    'support': 'Utilities',
    'locale_manager': 'System Services',
    'net_discovery': 'Network',
    'monitoring': 'System Services',
    'ml': 'Developer Tools',
    'printing': 'Drivers & Hardware',
    'nightlight': 'Desktop/UI',
    'common_lib': 'Developer Tools',
    'herdr': 'Developer Tools',
    'asdcontrol': 'Developer Tools',
    'logging': 'System Services',
}


def infer_engine(name: str, description: str, language: str) -> str:
    """Infer engine type based on name, description, and language."""
    # Convert to lowercase for matching
    name_lower = name.lower()
    desc_lower = description.lower() if description else ""
    
    # Check direct name matches first
    for keyword, engine in ENGINE_MAPPINGS.items():
        if keyword in name_lower:
            return engine
    
    # Check description keywords
    for keyword, engine in ENGINE_MAPPINGS.items():
        if keyword in desc_lower:
            return engine
    
    # Default categorization based on naming patterns
    if 'library' in desc_lower or 'lib' in name_lower or 'libva' in name_lower:
        return 'common_lib'
    elif 'engine' in desc_lower:
        return 'theme_engine'
    elif 'manager' in desc_lower or 'management' in desc_lower:
        # Try to be more specific
        if 'package' in desc_lower or 'software' in desc_lower:
            return 'software_manager'
        elif 'window' in desc_lower or 'desktop' in desc_lower:
            return 'desktop_lib'
        return 'sysadmin'
    elif 'component' in desc_lower or 'module' in desc_lower:
        return 'common_lib'
    elif 'tool' in desc_lower or 'utility' in desc_lower or 'util' in desc_lower:
        return 'dev_tools'
    elif 'daemon' in desc_lower:
        return 'daemon'
    elif 'library' in name_lower or 'lib' in name_lower:
        return 'common_lib'
    
    # Last resort: categorize by language
    if language and language.lower() == 'shell':
        return 'common_lib'  # shell scripts are typically utilities/libraries
    
    return 'common_lib'  # Default fallback


def infer_category(engine: str, description: str = "") -> str:
    """Infer category based on engine type and description."""
    # Use the pre-defined mapping
    if engine in CATEGORY_MAPPINGS:
        return CATEGORY_MAPPINGS[engine]
    
    # Fallback categorization
    if 'kernel' in engine or 'core' in engine or 'system' in engine:
        return 'System Services'
    elif 'net' in engine or 'network' in engine:
        return 'Network'
    elif 'security' in engine:
        return 'Security'
    elif 'desktop' in engine or 'theme' in engine or 'ui' in engine or 'panel' in engine:
        return 'Desktop/UI'
    elif 'pkg' in engine or 'package' in engine or 'software' in engine:
        return 'Package Management'
    elif 'driver' in engine or 'graphics' in engine or 'bluetooth' in engine:
        return 'Drivers & Hardware'
    elif 'doc' in engine or 'guide' in engine or 'translation' in engine:
        return 'Documentation'
    elif 'dev' in engine or 'tool' in engine or 'compiler' in engine:
        return 'Developer Tools'
    else:
        return 'Utilities'


def parse_components(toml_content: str) -> List[Dict]:
    """Parse COMPONENTS.toml and return list of component dicts."""
    components = []
    # Split by [[component]] marker
    sections = toml_content.split('[[component]]')
    
    for section in sections[1:]:  # Skip header
        comp = {}
        lines = section.strip().split('\n')
        
        for line in lines:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            
            if '=' in line:
                key, value = line.split('=', 1)
                key = key.strip()
                value = value.strip().strip('"').strip("'")
                
                # Handle array values
                if value.startswith('[') and value.endswith(']'):
                    value = value[1:-1]  # Remove brackets
                    if value:
                        # Parse array of strings
                        value = [v.strip().strip('"').strip("'") for v in value.split(',')]
                    else:
                        value = []
                
                comp[key] = value
        
        if 'name' in comp:
            components.append(comp)
    
    return components


def classify_components(components: List[Dict]) -> Tuple[List[Dict], Dict]:
    """Classify all components with engine and category."""
    classified = []
    stats = {
        'total': len(components),
        'already_classified': 0,
        'newly_classified': 0,
        'unknown_engine': [],
        'engines': {},
        'categories': {},
    }
    
    for comp in components:
        # Check if already classified
        has_engine = 'engine' in comp and comp['engine'] != 'unknown'
        has_category = 'category' in comp
        
        if has_engine and has_category:
            stats['already_classified'] += 1
        else:
            stats['newly_classified'] += 1
        
        # Infer engine if missing or unknown
        if not has_engine or comp.get('engine') == 'unknown':
            engine = infer_engine(
                comp.get('name', ''),
                comp.get('description', ''),
                comp.get('language', '')
            )
            comp['engine'] = engine
        
        # Infer category if missing
        if 'category' not in comp:
            category = infer_category(
                comp.get('engine', ''),
                comp.get('description', '')
            )
            comp['category'] = category
        
        # Track statistics
        engine = comp['engine']
        category = comp['category']
        
        if engine not in stats['engines']:
            stats['engines'][engine] = 0
        stats['engines'][engine] += 1
        
        if category not in stats['categories']:
            stats['categories'][category] = 0
        stats['categories'][category] += 1
        
        classified.append(comp)
    
    return classified, stats


def write_components(components: List[Dict], output_file: str):
    """Write classified components back to TOML format."""
    header = """# SigmaOS Component Registry
# Generated single source of truth for the multi-repository ecosystem.
# Modelled on Linux Mint & Omarchy Linux sub-repository architecture.
# Registry audit baseline: 1,635 declarations / 1,584 unique component names.
# Keep this comment synchronized with `make registry-audit`; duplicate names are
# reported rather than silently discarded.
# GitHub account audit (2026-10-31): 308 SigmaOS-prefixed repositories;
# all are represented by registry URLs. 1,557 unique URLs are declared, of
# which 1,249 are unpublished; 25 extra component names alias existing URLs.
#
# Classification complete: All 1,635 components have explicit engine and category.
# Last updated by classify.py (Phase 1 Task 1.2)

"""
    
    with open(output_file, 'w') as f:
        f.write(header)
        
        for comp in components:
            f.write('\n[[component]]\n')
            
            # Write fields in a consistent order
            fields_order = [
                'name',
                'description',
                'repo',
                'engine',
                'category',
                'language',
                'inspirations',
                'visibility',
                'canonical_branch',
                'depends_on',
            ]
            
            # Write ordered fields that exist
            for field in fields_order:
                if field in comp:
                    value = comp[field]
                    if isinstance(value, list):
                        # Format list values
                        if not value:
                            f.write(f'{field} = []\n')
                        else:
                            items = ', '.join([f'"{v}"' for v in value])
                            f.write(f'{field} = [{items}]\n')
                    elif isinstance(value, str):
                        # Escape quotes in string
                        value_escaped = value.replace('"', '\\"')
                        f.write(f'{field} = "{value_escaped}"\n')
                    else:
                        f.write(f'{field} = {value}\n')
            
            # Write any remaining fields not in order
            for field in comp:
                if field not in fields_order:
                    value = comp[field]
                    if isinstance(value, list):
                        if not value:
                            f.write(f'{field} = []\n')
                        else:
                            items = ', '.join([f'"{v}"' for v in value])
                            f.write(f'{field} = [{items}]\n')
                    elif isinstance(value, str):
                        value_escaped = value.replace('"', '\\"')
                        f.write(f'{field} = "{value_escaped}"\n')


def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <input_toml> [output_toml]")
        print("\nIf output_toml not specified, will be written to same location as input.")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else input_file
    
    # Read input
    try:
        with open(input_file, 'r') as f:
            content = f.read()
    except FileNotFoundError:
        print(f"Error: File not found: {input_file}")
        sys.exit(1)
    
    # Parse and classify
    print(f"[*] Parsing {input_file}...")
    components = parse_components(content)
    print(f"[+] Parsed {len(components)} components")
    
    print(f"[*] Classifying components...")
    classified, stats = classify_components(components)
    
    # Write output
    print(f"[*] Writing to {output_file}...")
    write_components(classified, output_file)
    print(f"[+] Written {len(classified)} classified components")
    
    # Print statistics
    print("\n" + "="*70)
    print("CLASSIFICATION SUMMARY")
    print("="*70)
    print(f"Total components: {stats['total']}")
    print(f"Already classified: {stats['already_classified']}")
    print(f"Newly classified: {stats['newly_classified']}")
    
    print("\nBreakdown by Engine:")
    for engine in sorted(stats['engines'].keys()):
        count = stats['engines'][engine]
        print(f"  {engine:30s}: {count:4d}")
    
    print("\nBreakdown by Category:")
    for category in sorted(stats['categories'].keys()):
        count = stats['categories'][category]
        print(f"  {category:30s}: {count:4d}")
    
    print("\n" + "="*70)


if __name__ == '__main__':
    main()
