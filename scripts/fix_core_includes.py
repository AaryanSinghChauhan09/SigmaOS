import os
import re

include_dir = "include"

for root, dirs, files in os.walk(include_dir):
    for file in files:
        if file.endswith((".h", ".hpp")):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Replace #include "./include/..." with #include "./..." 
            # if the file is in include/
            new_content = re.sub(r'#include\s+"\./include/', '#include "./', content)
            
            if content != new_content:
                with open(path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"Fixed: {path}")

# Now fix the ones that need to go UP
# (this is harder to automate generally, so I'll do it for the known ones)
