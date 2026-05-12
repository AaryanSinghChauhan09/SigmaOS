import os
import re

core_dir = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS-Repo\kernel\core"

for filename in os.listdir(core_dir):
    if filename.endswith(".cpp"):
        filepath = os.path.join(core_dir, filename)
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()
        
        # Replace Lattice.h with sigma_hal.h
        content = content.replace('#include "Lattice.h"', '#include "sigma_hal.h"\n#include "sigma_types.h"')
        content = content.replace('#include <Lattice.h>', '#include "sigma_hal.h"\n#include "sigma_types.h"')
        
        # Ensure sigma_hal.h is included
        if '#include "sigma_hal.h"' not in content:
            content = '#include "sigma_hal.h"\n' + content
            
        # Ensure sigma_types.h is included
        if '#include "sigma_types.h"' not in content:
            content = '#include "sigma_types.h"\n' + content
            
        # Fix SovereignOrchestratorEngine typo
        content = content.replace("SovereignOrchestraEngine", "SovereignOrchestratorEngine")
        
        with open(filepath, "w", encoding="utf-8") as f:
            f.write(content)

print("Automated C++ fixes applied.")
