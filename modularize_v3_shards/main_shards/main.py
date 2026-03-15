# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, sys, textwrap

def main():
    print('=' * 60)
    print('SigmaOS Auto-Modularizer v3.0  (AST-driven, per-function)')
    print('=' * 60)
    targets = []
    for dirpath, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for fn in files:
            if not fn.endswith('.py'):
                continue
            if fn in SKIP_FILES:
                continue
            fp = os.path.join(dirpath, fn)
            size = os.path.getsize(fp)
            if size >= MIN_BYTES:
                targets.append((size, fp))
    targets.sort(reverse=True)
    print(f'Found {len(targets)} files >= {MIN_BYTES // 1000}KB to modularize.\n')
    total = 0
    for size, fp in targets:
        rel = os.path.relpath(fp, ROOT)
        print(f'\n>> {rel}  ({size:,} bytes)')
        n = split_file(fp)
        print(f'   -> {n} module(s) created')
        total += n
    print('\n' + '=' * 60)
    print('Syntax verification pass ...')
    errors = []
    ok = 0
    for dirpath, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for fn in files:
            if not fn.endswith('.py'):
                continue
            fp = os.path.join(dirpath, fn)
            try:
                with open(fp, encoding='utf-8', errors='replace') as f:
                    ast.parse(f.read(), filename=fp)
                ok += 1
            except SyntaxError as e:
                errors.append(f'  SYNTAX_ERR {fp}: {e}')
    if errors:
        print(f'ERRORS ({len(errors)}):')
        for e in errors:
            print(e)
    else:
        print(f'All {ok} files passed syntax check — 0 errors.')
    print('=' * 60)
    print(f'Done. {total} new module files created across all packages.')
    print('=' * 60)