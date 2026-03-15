"""Fix unicode-escape errors in auto-split docstring headers."""
import ast, os

ROOT = r'.'
SKIP = {'.git', '__pycache__', 'node_modules', 'evidence_vault', 'SOVEREIGN_DISTRO_IMG'}

fixed = 0
unfixable = []

for dirpath, dirs, files in os.walk(ROOT):
    dirs[:] = [x for x in dirs if x not in SKIP]
    for fn in files:
        if not fn.endswith('.py'):
            continue
        fp = os.path.join(dirpath, fn)
        try:
            with open(fp, encoding='utf-8', errors='replace') as f:
                src = f.read()
            ast.parse(src)   # already valid — skip
        except SyntaxError as e:
            if 'unicodeescape' not in str(e) and 'unicode error' not in str(e):
                continue     # different kind of error — skip

            lines = src.splitlines(keepends=True)
            new_lines = []
            in_header_docstring = True
            docstring_closed = False

            for i, line in enumerate(lines):
                if in_header_docstring and not docstring_closed:
                    # Replace the docstring header with a safe ASCII comment
                    if line.strip().startswith('"""'):
                        # Close docstring detection
                        count = line.count('"""')
                        if count >= 2:
                            # single-line docstring — replace entire line
                            new_lines.append('# auto-split module\n')
                            in_header_docstring = False
                            docstring_closed = True
                        else:
                            # opening triple-quote — replace until closing
                            new_lines.append('# auto-split module\n')
                            docstring_closed = False
                        continue
                    elif not docstring_closed:
                        # inside opening docstring block — skip lines until closing """
                        if '"""' in line:
                            docstring_closed = True
                            in_header_docstring = False
                        continue
                new_lines.append(line)

            new_src = ''.join(new_lines)
            try:
                ast.parse(new_src)
                with open(fp, 'w', encoding='utf-8') as f:
                    f.write(new_src)
                fixed += 1
            except SyntaxError as e2:
                unfixable.append(f'{fp}: {e2}')

if unfixable:
    print(f'UNFIXABLE ({len(unfixable)}):')
    for u in unfixable:
        print(' ', u)
else:
    print(f'All unicode-escape errors fixed. {fixed} files healed.')
