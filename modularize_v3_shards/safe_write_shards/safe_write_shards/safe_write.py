# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, sys, textwrap

def safe_write(path: str, content: str, overwrite: bool=True):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if not overwrite and os.path.exists(path):
        return f'  [SKIP-EXISTS] {path}'
    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)
    return f'  [WROTE] {path}'