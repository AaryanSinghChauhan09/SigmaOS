import os

root = r"c:\Users\Aaryan\Downloads\SigmaOS"
replacements = {
    "from sigma_core.system.interfaces": "from sigma_core.system.interfaces",
    "from sigma_core.hal.kernel_hal": "from sigma_core.hal.kernel_hal"
}

for dirpath, dirnames, filenames in os.walk(root):
    for f in filenames:
        if f.endswith(".py"):
            fp = os.path.join(dirpath, f)
            try:
                with open(fp, "r", encoding="utf-8") as file:
                    content = file.read()
                
                new_content = content
                for old, new in replacements.items():
                    new_content = new_content.replace(old, new)
                
                if new_content != content:
                    with open(fp, "w", encoding="utf-8") as file:
                        file.write(new_content)
                    print(f"Fixed: {fp}")
            except Exception as e:
                print(f"Error on {fp}: {e}")
