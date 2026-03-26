import os


def fix_apex():
    p = 'c:\\Users\\SigmaSovereign\\Downloads\\SigmaOS\\sigma_apex_optimizer.py'
    if not os.path.exists(p):
        return
    with open(p, 'r', encoding='utf-8') as f:
        src = f.read()
    src = src.replace('return ".".join(parts[:-1])', 'return ".".join(list(parts)[:-1])  # type: ignore')
    old_assign = '        elif isinstance(node, ast.Assign):\n            if all(isinstance(t, ast.Name) and t.id.isupper() for t in node.targets):\n                constants.append(node)\n                for t in node.targets: sisters[t.id] = node'
    new_assign = "        elif isinstance(node, ast.Assign):\n            if all(isinstance(t, ast.Name) and getattr(t, 'id', '').isupper() for t in node.targets):\n                constants.append(node)\n                for t in node.targets:\n                    if isinstance(t, ast.Name):\n                        sisters[t.id] = node"
    src = src.replace(old_assign, new_assign)
    src = src.replace('lines = []', 'lines = []  # type: ignore')
    src = src.replace('bb or [ast.Pass()]', 'bb if bb else [ast.Pass()]  # type: ignore')
    src = src.replace('for t in c.targets: shims.append', 'for t in c.targets:\n                if isinstance(t, ast.Name):\n                    shims.append')
    with open(p, 'w', encoding='utf-8') as f:
        f.write(src)