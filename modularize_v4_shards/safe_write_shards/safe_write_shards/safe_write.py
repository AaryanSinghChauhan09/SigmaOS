# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, textwrap

def safe_write(path: str, content: str):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)
    print(f'  [WROTE] {path}')