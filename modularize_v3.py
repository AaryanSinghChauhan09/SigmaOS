"""
SigmaOS Auto-Modularizer v3.0
=============================
AST-driven: reads every .py file in the repo, finds classes/functions,
and writes each into its own dedicated file.  Skips already-tiny files,
evidence_vault, SOVEREIGN_DISTRO_IMG, and the modularizer itself.
"""
import ast, os, sys, textwrap

ROOT   = os.path.abspath(os.path.join(os.path.dirname(__file__)))
SKIP_DIRS  = {".git", "__pycache__", "node_modules", "evidence_vault",
              "SOVEREIGN_DISTRO_IMG", ".pytest_cache"}
SKIP_FILES = {"modularize_all.py", "modularize_v3.py", "setup.py", "conftest.py"}
MIN_BYTES  = 6_000   # only split files larger than this

# ── helpers ──────────────────────────────────────────────────────────────────

def safe_write(path: str, content: str, overwrite: bool = True):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if not overwrite and os.path.exists(path):
        return f"  [SKIP-EXISTS] {path}"
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    return f"  [WROTE] {path}"


def make_init(pkg_dir: str, exports: list[str]):
    lines = ['"""Auto-generated package __init__.py"""\n']
    for name in exports:
        lines.append(f"from .{name} import *  # noqa: F401, F403\n")
    return safe_write(os.path.join(pkg_dir, "__init__.py"), "".join(lines))


def collect_imports(tree: ast.Module) -> list[str]:
    """Return all top-level import lines from a module as source strings."""
    lines = ast.unparse(node) + "\n"
    return []          # placeholder – we rebuild with ast.unparse below


def source_of(node: ast.AST) -> str:
    return ast.unparse(node)


def dedent_src(src: str) -> str:
    return textwrap.dedent(src)


def node_header_comment(filename: str, name: str) -> str:
    return f'"""\nAuto-split from {filename} — {name}\n"""\n\n'


def get_top_imports(tree: ast.Module, source: str) -> str:
    """Collect all import statements from the top of the module."""
    import_lines = []
    for node in tree.body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            import_lines.append(ast.unparse(node))
    return "\n".join(import_lines) + ("\n\n" if import_lines else "")


def get_top_constants(tree: ast.Module) -> str:
    """Collect module-level assignments (constants / type aliases)."""
    parts = []
    for node in tree.body:
        if isinstance(node, (ast.Assign, ast.AnnAssign, ast.AugAssign)):
            parts.append(ast.unparse(node))
        elif isinstance(node, ast.ClassDef):
            # collect simple enum/dataclass-only classes as constants
            if all(isinstance(n, (ast.Assign, ast.AnnAssign, ast.Pass, ast.Expr))
                   for n in node.body):
                parts.append(ast.unparse(node))
    return "\n".join(parts) + ("\n\n" if parts else "")


# ── core splitter ─────────────────────────────────────────────────────────────

def split_file(filepath: str) -> int:
    """Split one .py file into a package of per-function files.
    Returns the number of new module files created."""
    with open(filepath, encoding="utf-8", errors="replace") as f:
        source = f.read()

    if len(source.encode()) < MIN_BYTES:
        return 0

    try:
        tree = ast.parse(source, filename=filepath)
    except SyntaxError as e:
        print(f"  [SYNTAX-ERR] {filepath}: {e}")
        return 0

    rel     = os.path.relpath(filepath, ROOT)
    dirname = os.path.dirname(filepath)
    stem    = os.path.splitext(os.path.basename(filepath))[0]
    pkg_dir = os.path.join(dirname, stem)

    # Gather imports + constants once
    hdr_imports   = get_top_imports(tree, source)
    hdr_constants = get_top_constants(tree)

    created  = []   # file stems written
    shim_exports: list[str] = []

    for node in tree.body:
        # ── top-level function ──────────────────────────────────────────────
        if isinstance(node, ast.FunctionDef) or isinstance(node, ast.AsyncFunctionDef):
            fname   = node.name
            fn_src  = ast.unparse(node)
            content = (
                node_header_comment(rel, fname)
                + hdr_imports
                + "\n\n"
                + fn_src
                + "\n"
            )
            out = os.path.join(pkg_dir, f"{fname}.py")
            result = safe_write(out, content, overwrite=True)
            print(result)
            created.append(fname)
            shim_exports.append(fname)

        # ── class (split each method into its own file) ─────────────────────
        elif isinstance(node, ast.ClassDef):
            cls_name     = node.name
            cls_pkg_dir  = os.path.join(pkg_dir, cls_name.lower())
            class_exports: list[str] = []

            # class-level header: imports + __init__ stays in class_pkg/init_.py
            class_imports_src = hdr_imports

            for item in node.body:
                if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    method_name = item.name
                    method_src  = ast.unparse(item)

                    # Rebuild as standalone method inside the class shell
                    content = (
                        node_header_comment(rel, f"{cls_name}.{method_name}")
                        + class_imports_src
                        + f"\n\nclass {cls_name}:\n"
                        + textwrap.indent(method_src, "    ")
                        + "\n"
                    )
                    out    = os.path.join(cls_pkg_dir, f"{method_name}.py")
                    result = safe_write(out, content, overwrite=True)
                    print(result)
                    class_exports.append(method_name)

                elif isinstance(item, (ast.Assign, ast.AnnAssign)):
                    # Class-level attributes go into _attrs.py
                    attrs_path = os.path.join(cls_pkg_dir, "_attrs.py")
                    attr_src   = ast.unparse(item)
                    os.makedirs(cls_pkg_dir, exist_ok=True)
                    with open(attrs_path, "a", encoding="utf-8") as af:
                        af.write(attr_src + "\n")

            if class_exports:
                result = make_init(cls_pkg_dir, class_exports)
                print(result)
                created.append(cls_name.lower())
                shim_exports.append(cls_name)

    if not created:
        return 0

    # Write package __init__
    result = make_init(pkg_dir, [s.lower() if s[0].isupper() else s for s in shim_exports])
    print(result)

    # Write backward-compat shim (overwrite the original .py)
    shim = (
        f'"""\n{stem}.py — backward-compat shim.\n'
        f'Real implementation lives in {stem}/ package.\n"""\n\n'
        + "\n".join(f"from {stem}.{s} import *  # noqa" for s in shim_exports)
        + "\n\n__all__ = " + repr(shim_exports) + "\n"
    )
    result = safe_write(filepath, shim, overwrite=True)
    print(result)

    return len(created)


# ── walk the repo ─────────────────────────────────────────────────────────────

def main():
    print("=" * 60)
    print("SigmaOS Auto-Modularizer v3.0  (AST-driven, per-function)")
    print("=" * 60)

    targets = []
    for dirpath, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for fn in files:
            if not fn.endswith(".py"):
                continue
            if fn in SKIP_FILES:
                continue
            fp   = os.path.join(dirpath, fn)
            size = os.path.getsize(fp)
            if size >= MIN_BYTES:
                targets.append((size, fp))

    targets.sort(reverse=True)
    print(f"Found {len(targets)} files >= {MIN_BYTES//1000}KB to modularize.\n")

    total = 0
    for size, fp in targets:
        rel = os.path.relpath(fp, ROOT)
        print(f"\n>> {rel}  ({size:,} bytes)")
        n = split_file(fp)
        print(f"   -> {n} module(s) created")
        total += n

    # Syntax-check everything
    print("\n" + "=" * 60)
    print("Syntax verification pass ...")
    errors = []
    ok = 0
    for dirpath, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for fn in files:
            if not fn.endswith(".py"):
                continue
            fp = os.path.join(dirpath, fn)
            try:
                with open(fp, encoding="utf-8", errors="replace") as f:
                    ast.parse(f.read(), filename=fp)
                ok += 1
            except SyntaxError as e:
                errors.append(f"  SYNTAX_ERR {fp}: {e}")

    if errors:
        print(f"ERRORS ({len(errors)}):")
        for e in errors:
            print(e)
    else:
        print(f"All {ok} files passed syntax check — 0 errors.")

    print("=" * 60)
    print(f"Done. {total} new module files created across all packages.")
    print("=" * 60)


if __name__ == "__main__":
    main()
