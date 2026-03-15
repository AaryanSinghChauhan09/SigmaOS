# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
from sigma_core.security.resilience_guard import resilient_module
import os
import ast

@resilient_module
def heal_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            source = f.read()
        if not source.strip():
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write('pass\n')
            return True
        lines = source.splitlines()
        new_lines = []
        changed = False
        for i in range(len(lines)):
            line = lines[i]
            new_lines.append(line)
            stripped = line.strip()
            if stripped.endswith(':') and (stripped.startswith('class ') or stripped.startswith('def ') or stripped.startswith('async def ')):
                has_body = False
                for j in range(i + 1, len(lines)):
                    if lines[j].strip():
                        if lines[j].startswith(' ') or lines[j].startswith('\t'):
                            has_body = True
                        break
                if not has_body:
                    new_lines.append('    pass')
                    changed = True
        if changed:
            fixed_source = '\n'.join(new_lines)
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(fixed_source)
            return True
        return False
    except Exception as e:
        return False