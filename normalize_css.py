import re

file_path = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS-Repo\ui\themes\zenith_desktop.css"

with open(file_path, "r") as f:
    content = f.read()

# Remove redundant backdrop-filter lines
# Keep one -webkit-backdrop-filter and one backdrop-filter per block
def clean_block(match):
    block = match.group(1)
    # Extract all backdrop-filter lines
    filters = re.findall(r".*backdrop-filter:.*", block)
    if not filters:
        return match.group(0)
    
    # Keep only the first occurrence of each type
    webkit_kept = False
    standard_kept = False
    new_lines = []
    for line in block.splitlines():
        if "-webkit-backdrop-filter" in line:
            if not webkit_kept:
                new_lines.append("    -webkit-backdrop-filter: blur(var(--glass-blur));")
                webkit_kept = True
        elif "backdrop-filter" in line:
            if not standard_kept:
                new_lines.append("    backdrop-filter: blur(var(--glass-blur));")
                standard_kept = True
        else:
            new_lines.append(line)
    
    return "{" + "\n".join(new_lines) + "}"

# Simplified regex to match CSS blocks
new_content = re.sub(r"\{([^{}]*backdrop-filter[^{}]*)\}", clean_block, content)

with open(file_path, "w") as f:
    f.write(new_content)

print("CSS normalization complete.")
