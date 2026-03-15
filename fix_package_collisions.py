import os

ROOT = "."
SKIP_DIRS = {".git", "__pycache__", "node_modules", "evidence_vault", "SOVEREIGN_DISTRO_IMG", "artifacts"}

def final_fix_package_structure():
    print("Fixing Package Structure (Converting shims to __init__)...")
    
    # We look for all .py files that have a directory with the same name (minus .py)
    for root, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for file in files:
            if file.endswith(".py") and file != "__init__.py":
                stem = file[:-3]
                dir_path = os.path.join(root, stem)
                
                if os.path.isdir(dir_path):
                    shim_file = os.path.join(root, file)
                    init_file = os.path.join(dir_path, "__init__.py")
                    
                    print(f"  Consolidating {shim_file} into {init_file}")
                    
                    try:
                        with open(shim_file, 'r', encoding='utf-8') as f:
                            shim_content = f.read()
                        
                        # Append shim content to __init__.py if it exists, or create it
                        existing_init = ""
                        if os.path.exists(init_file):
                            with open(init_file, 'r', encoding='utf-8') as f:
                                existing_init = f.read()
                        
                        # Merge logic: if shim just has imports, prepend them
                        with open(init_file, 'w', encoding='utf-8') as f:
                            # Prepend the shim content to handle the exports
                            f.write(shim_content + "\n" + existing_init)
                        
                        # Delete the shim file to avoid collision
                        os.remove(shim_file)
                        print(f"    Succeeded.")
                    except Exception as e:
                        print(f"    Failed: {e}")

if __name__ == "__main__":
    final_fix_package_structure()
