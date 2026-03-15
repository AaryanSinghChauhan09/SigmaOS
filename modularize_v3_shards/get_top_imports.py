# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, sys, textwrap

def get_top_imports(tree: ast.Module, source: str) -> str:
    """Collect all import statements from the top of the module."""
    import_lines = []
    for node in tree.body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            import_lines.append(ast.unparse(node))
    return '\n'.join(import_lines) + ('\n\n' if import_lines else '')