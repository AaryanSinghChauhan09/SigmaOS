import os

src_dir = r"C:\Users\Aaryan\.gemini\antigravity-ide\scratch\SigmaOS\src"

created_count = 0
for root, dirs, files in os.walk(src_dir):
    # relative path
    rel_path = os.path.relpath(root, src_dir)
    if rel_path == ".":
        continue
    
    # check if directory has .rs files (other than mod.rs)
    rs_files = [f for f in files if f.endswith(".rs") and f != "mod.rs"]
    if rs_files and "mod.rs" not in files:
        mod_file_path = os.path.join(root, "mod.rs")
        mod_contents = []
        for f in sorted(rs_files):
            mod_name = f[:-3]
            mod_contents.append(f"pub mod {mod_name};")
        
        with open(mod_file_path, "w", encoding="utf-8") as out:
            out.write("\n".join(mod_contents) + "\n")
        print(f"Created {mod_file_path} exposing: {rs_files}")
        created_count += 1

print(f"Total mod.rs created: {created_count}")
