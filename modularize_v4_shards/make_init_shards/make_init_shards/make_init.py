# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, textwrap

def make_init(pkg_dir: str, exports: list):
    lines = ['"""Auto-generated package __init__.py"""\n']
    for name in exports:
        lines.append(f'from .{name} import *  # noqa: F401, F403\n')
    safe_write(os.path.join(pkg_dir, '__init__.py'), ''.join(lines))