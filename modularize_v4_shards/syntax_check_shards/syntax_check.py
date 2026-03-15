# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, textwrap

def syntax_check():
    errors, ok = ([], 0)
    for d, dirs, files in os.walk(ROOT):
        dirs[:] = [x for x in dirs if x not in SKIP_DIRS]
        for fn in files:
            if not fn.endswith('.py'):
                continue
            fp = os.path.join(d, fn)
            raw = open(fp, 'rb').read().decode('utf-8', 'replace')
            try:
                ast.parse(raw)
                ok += 1
            except SyntaxError as e:
                errors.append(f'{os.path.relpath(fp, ROOT)}: {e}')
    return (ok, errors)