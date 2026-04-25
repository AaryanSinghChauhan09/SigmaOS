import os

core_h_path = "sigmaos/core/src/sigma_core.h"
src_dir = "sigmaos/core/src"

with open(core_h_path, "r") as f:
    lines = f.readlines()

functions = []
for line in lines:
    line = line.strip()
    if line.startswith("void ") or line.startswith("void* ") or line.startswith("int "):
        if "(" in line and ");" in line:
            func_decl = line.replace(";", "")
            func_name = func_decl.split("(")[0].split(" ")[-1].replace("*", "")
            functions.append((func_name, func_decl))

for func_name, func_decl in functions:
    file_path = os.path.join(src_dir, f"atomic_{func_name}.cpp")
    if not os.path.exists(file_path):
        content = f"""#include "sigma_core.h"
#include "sigma_libc.h"

extern "C" {{

{func_decl} {{
    sigma_kprint("[SigmaAtomic] Executing {func_name}...\\n");
    // TODO: Implement native silicon logic
"""
        if func_decl.startswith("void* "):
            content += "    return 0;\n"
        elif func_decl.startswith("int "):
            content += "    return 0;\n"
        
        content += "}\n\n}\n"
        
        with open(file_path, "w") as f:
            f.write(content)

print(f"Generated atomic modules for {len(functions)} functions.")
