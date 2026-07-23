import os

src_dir = r"C:\Users\Aaryan\.gemini\antigravity-ide\scratch\SigmaOS\src"
entries = os.listdir(src_dir)
mods = []
for entry in entries:
    full_path = os.path.join(src_dir, entry)
    if os.path.isdir(full_path):
        # check if it has mod.rs or any .rs files
        rs_files = [f for f in os.listdir(full_path) if f.endswith(".rs")]
        if rs_files:
            mods.append(entry.replace("-", "_"))

mods.sort()
print("Discovered modules:", len(mods))
print(mods)
