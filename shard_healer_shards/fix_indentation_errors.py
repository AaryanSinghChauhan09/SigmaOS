# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import os

def fix_indentation_errors(root_dir):
    """
    Scans for empty class/def blocks and adds 'pass' to prevent IndentationError.
    """
    for root, dirs, files in os.walk(root_dir):
        if '.git' in root:
            continue
        for file in files:
            if file.endswith('.py'):
                path = os.path.join(root, file)
                try:
                    with open(path, 'r', encoding='utf-8', errors='replace') as f:
                        lines = f.readlines()
                    modified = False
                    new_lines = []
                    for i, line in enumerate(lines):
                        new_lines.append(line)
                        stripped = line.strip()
                        if stripped.endswith(':') and (stripped.startswith('def ') or stripped.startswith('class ') or stripped.startswith('async def ')):
                            has_body = False
                            if i + 1 < len(lines):
                                next_line = lines[i + 1].strip()
                                if not next_line:
                                    for j in range(i + 2, min(i + 5, len(lines))):
                                        if lines[j].strip():
                                            if lines[j].startswith((' ', '\t')):
                                                has_body = True
                                            break
                                elif lines[i + 1].startswith((' ', '\t')):
                                    has_body = True
                            if not has_body:
                                new_lines.append('    pass\n')
                                modified = True
                    if modified:
                        with open(path, 'w', encoding='utf-8') as f:
                            f.writelines(new_lines)
                except:
                    pass