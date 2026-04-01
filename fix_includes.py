import os

directory = r"C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
for root, dirs, files in os.walk(directory):
    for file in files:
        if file.endswith((".c", ".h", ".cpp", ".hpp")):
            filepath = os.path.join(root, file)
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                if '#include "SigmaOOP.hpp"' in content:
                    content = content.replace('#include "SigmaOOP.hpp"', '#include "SigmaC11.h"')
                    with open(filepath, 'w', encoding='utf-8') as f:
                        f.write(content)
                    print(f"Updated {filepath}")
            except Exception as e:
                pass
print("Done")
