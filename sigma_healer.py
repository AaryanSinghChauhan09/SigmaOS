import os
import ast

ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'evidence_vault', 'SOVEREIGN_DISTRO_IMG', 'artifacts', '.gemini'}

def heal_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            source = f.read()
        ast.parse(source)
        return False # No error
    except IndentationError:
        # Check for empty class/func bodies
        lines = source.splitlines()
        new_lines = []
        for line in lines:
            new_lines.append(line)
            if line.strip().endswith(':') and not line.strip().startswith('#'):
                # We'll see if the next non-empty line is indented. 
                # For simplicity, we'll just add a '    pass' if it looks like a header with no body
                pass 
        
        # A better way is using AST on the original and ensuring 'pass' is added
        # But if we can't parse it, we must fix it as strings roughly.
        
        # ACTUALLY, let's just use a regex to find empty blocks
        # find lines ending in : and if the next line (ignoring comments) is not indented, add pass
        processed_lines = []
        lines = source.splitlines()
        for i in range(len(lines)):
            processed_lines.append(lines[i])
            if lines[i].strip().endswith(':'):
                if i + 1 >= len(lines) or not lines[i+1].startswith(' ') and not lines[i+1].startswith('\t'):
                    processed_lines.append('    pass')
        
        new_source = '\n'.join(processed_lines)
        try:
            ast.parse(new_source)
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_source)
            return True
        except:
            return False
    except SyntaxError:
        # Handle cases like "from . import x" in a file with no parent package
        # Actually, for shims, we can just wrap in try-except or fix the path
        return False
    except:
        return False

if __name__ == "__main__":
    print("Healing modularized files...")
    healed = 0
    for root, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for file in files:
            if file.endswith('.py'):
                if heal_file(os.path.join(root, file)):
                    healed += 1
    print(f"Healed {healed} files.")
