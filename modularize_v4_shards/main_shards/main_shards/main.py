# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, textwrap

def main():
    print('=' * 60)
    print('SigmaOS Auto-Modularizer v4.0 — Final Deep Pass')
    print('=' * 60)
    targets = []
    for d, dirs, files in os.walk(ROOT):
        dirs[:] = [x for x in dirs if x not in SKIP_DIRS]
        for fn in files:
            if not fn.endswith('.py') or fn in SKIP_FILES:
                continue
            fp = os.path.join(d, fn)
            sz = os.path.getsize(fp)
            if sz >= MIN_BYTES:
                targets.append((sz, fp))
    targets.sort(reverse=True)
    print(f'Candidates: {len(targets)} files >= {MIN_BYTES // 1000}KB\n')
    total = 0
    for sz, fp in targets:
        rel = os.path.relpath(fp, ROOT)
        n = split_file(fp)
        if n:
            print(f'  [OK] {rel}  -> {n} modules')
            total += n
    print(f"\n{'=' * 60}")
    print(f'Modules created this pass: {total}')
    print('Running syntax check...')
    ok, errors = syntax_check()
    if errors:
        print(f'ERRORS ({len(errors)}):')
        for e in errors[:30]:
            print(f'  {e}')
    else:
        print(f'PASS — {ok} files, 0 errors')
    print('=' * 60)