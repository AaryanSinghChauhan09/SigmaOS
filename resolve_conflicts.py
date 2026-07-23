import os

def resolve_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    out = []
    in_conflict = False
    for line in lines:
        if line.startswith('<<<<<<<'):
            in_conflict = True
        elif line.startswith('======='):
            pass
        elif line.startswith('>>>>>>>'):
            in_conflict = False
        else:
            out.append(line)
            
    with open(filepath, 'w', encoding='utf-8') as f:
        f.writelines(out)
    print(f"Resolved {filepath}")

unmerged = [
    "src/device/manager.rs",
    "src/driver/device.rs",
    "src/driver/framework.rs",
    "src/drivers/main.rs",
    "src/drivers/mod.rs",
    "src/filesystem/vfs.rs",
    "src/kernel/main.rs",
    "src/lib.rs",
    "src/shell/repl.rs",
    "src/userspace/main.rs"
]

for f in unmerged:
    try:
        resolve_file(f)
    except Exception as e:
        print(f"Error on {f}: {e}")
