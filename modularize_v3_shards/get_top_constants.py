# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, sys, textwrap

def get_top_constants(tree: ast.Module) -> str:
    """Collect module-level assignments (constants / type aliases)."""
    parts = []
    for node in tree.body:
        if isinstance(node, (ast.Assign, ast.AnnAssign, ast.AugAssign)):
            parts.append(ast.unparse(node))
        elif isinstance(node, ast.ClassDef):
            if all((isinstance(n, (ast.Assign, ast.AnnAssign, ast.Pass, ast.Expr)) for n in node.body)):
                parts.append(ast.unparse(node))
    return '\n'.join(parts) + ('\n\n' if parts else '')