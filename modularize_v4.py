"""
SigmaOS Auto-Modularizer v4.0 — Final Pass
============================================
Lowers threshold to 3 KB so remaining medium-sized files are captured.
Skips files that are already a single function/class (nothing to split),
and also skips the modularizer scripts themselves.
"""
import ast, os, textwrap

ROOT   = os.path.abspath(os.path.join(os.path.dirname(__file__)))
SKIP_DIRS  = {".git", "__pycache__", "node_modules", "evidence_vault",
              "SOVEREIGN_DISTRO_IMG", ".pytest_cache"}
SKIP_FILES = {
    "modularize_all.py", "modularize_v3.py", "modularize_v4.py",
    "fix_unicode_headers.py", "fix_emdash_headers.py",
    "setup.py", "conftest.py",
}
MIN_BYTES = 3_000   # 3 KB threshold


def safe_write(path: str, content: str):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"  [WROTE] {path}")


def make_init(pkg_dir: str, exports: list):
    lines = ['"""Auto-generated package __init__.py"""\n']
    for name in exports:
        lines.append(f"from .{name} import *  # noqa: F401, F403\n")
    safe_write(os.path.join(pkg_dir, "__init__.py"), "".join(lines))


def get_imports(tree: ast.Module) -> str:
    parts = []
    for node in tree.body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            parts.append(ast.unparse(node))
    return "\n".join(parts) + ("\n\n" if parts else "")


def split_file(filepath: str) -> int:
    with open(filepath, "rb") as f:
        raw = f.read().decode("utf-8", "replace")

    if len(raw.encode()) < MIN_BYTES:
        return 0

    try:
        tree = ast.parse(raw, filename=filepath)
    except SyntaxError as e:
        print(f"  [SYNTAX-ERR] {filepath}: {e}")
        return 0

    rel     = os.path.relpath(filepath, ROOT)
    dirname = os.path.dirname(filepath)
    stem    = os.path.splitext(os.path.basename(filepath))[0]

    # Skip if already a shim (contains only import * lines)
    non_import = [n for n in tree.body
                  if not isinstance(n, (ast.Import, ast.ImportFrom,
                                        ast.Expr, ast.Assign, ast.AnnAssign))]
    func_nodes   = [n for n in non_import
                    if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
    class_nodes  = [n for n in non_import if isinstance(n, ast.ClassDef)]

    # Nothing to split if ≤1 top-level callable and no class
    if len(func_nodes) + len(class_nodes) <= 1:
        return 0

    pkg_dir = os.path.join(dirname, stem)
    hdr = get_imports(tree)
    created: list[str] = []
    shim_exports: list[str] = []

    for node in tree.body:
        # ── top-level function ──────────────────────────────────────────────
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            content = (
                f'# auto-split: {rel} — {name}\n'
                + hdr + "\n\n"
                + ast.unparse(node) + "\n"
            )
            safe_write(os.path.join(pkg_dir, f"{name}.py"), content)
            created.append(name)
            shim_exports.append(name)

        # ── class ───────────────────────────────────────────────────────────
        elif isinstance(node, ast.ClassDef):
            cls_name    = node.name
            cls_pkg     = os.path.join(pkg_dir, cls_name.lower())
            cls_exports: list[str] = []

            for item in node.body:
                if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    mname   = item.name
                    content = (
                        f'# auto-split: {rel} — {cls_name}.{mname}\n'
                        + hdr
                        + f"\n\nclass {cls_name}:\n"
                        + textwrap.indent(ast.unparse(item), "    ")
                        + "\n"
                    )
                    safe_write(os.path.join(cls_pkg, f"{mname}.py"), content)
                    cls_exports.append(mname)
                elif isinstance(item, (ast.Assign, ast.AnnAssign)):
                    attrs_path = os.path.join(cls_pkg, "_attrs.py")
                    os.makedirs(cls_pkg, exist_ok=True)
                    with open(attrs_path, "a", encoding="utf-8") as af:
                        af.write(ast.unparse(item) + "\n")

            if cls_exports:
                make_init(cls_pkg, cls_exports)
                created.append(cls_name.lower())
                shim_exports.append(cls_name)

    if not created:
        return 0

    make_init(pkg_dir, [s.lower() if s[0].isupper() else s for s in shim_exports])

    # Backward-compat shim
    shim = (
        f'# {stem}.py — backward-compat shim\n'
        + "\n".join(f"from {stem}.{s} import *  # noqa" for s in shim_exports)
        + "\n\n__all__ = " + repr(shim_exports) + "\n"
    )
    safe_write(filepath, shim)
    return len(created)


def syntax_check():
    errors, ok = [], 0
    for d, dirs, files in os.walk(ROOT):
        dirs[:] = [x for x in dirs if x not in SKIP_DIRS]
        for fn in files:
            if not fn.endswith(".py"):
                continue
            fp = os.path.join(d, fn)
            raw = open(fp, "rb").read().decode("utf-8", "replace")
            try:
                ast.parse(raw)
                ok += 1
            except SyntaxError as e:
                errors.append(f"{os.path.relpath(fp, ROOT)}: {e}")
    return ok, errors


def main():
    print("=" * 60)
    print("SigmaOS Auto-Modularizer v4.0 — Final Deep Pass")
    print("=" * 60)

    targets = []
    for d, dirs, files in os.walk(ROOT):
        dirs[:] = [x for x in dirs if x not in SKIP_DIRS]
        for fn in files:
            if not fn.endswith(".py") or fn in SKIP_FILES:
                continue
            fp = os.path.join(d, fn)
            sz = os.path.getsize(fp)
            if sz >= MIN_BYTES:
                targets.append((sz, fp))

    targets.sort(reverse=True)
    print(f"Candidates: {len(targets)} files >= {MIN_BYTES//1000}KB\n")

    total = 0
    for sz, fp in targets:
        rel = os.path.relpath(fp, ROOT)
        n = split_file(fp)
        if n:
            print(f"  [OK] {rel}  -> {n} modules")
            total += n

    print(f"\n{'='*60}")
    print(f"Modules created this pass: {total}")
    print("Running syntax check...")
    ok, errors = syntax_check()
    if errors:
        print(f"ERRORS ({len(errors)}):")
        for e in errors[:30]:
            print(f"  {e}")
    else:
        print(f"PASS — {ok} files, 0 errors")
    print("=" * 60)


if __name__ == "__main__":
    main()
