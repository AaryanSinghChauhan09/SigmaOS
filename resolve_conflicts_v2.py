import sys
import re

def resolve_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Remove conflict markers and keep both versions
    # Pattern: <<<<<<< ... ======= ... >>>>>>> 
    # We'll keep the content from both sides
    pattern = r'<<<<<<< HEAD\n(.*?)\n=======\n(.*?)\n>>>>>>> origin/feature/wireshark-distro-improvements-14948326477708832768'
    
    def replace_conflict(match):
        part1 = match.group(1)
        part2 = match.group(2)
        # Keep both parts, removing duplicate content
        # For now, just concatenate both parts
        return part1 + '\n' + part2
    
    content = re.sub(pattern, replace_conflict, content, flags=re.DOTALL)
    
    # Clean up any remaining conflict markers
    content = re.sub(r'<<<<<<< .*?\n', '', content)
    content = re.sub(r'=======\n', '\n', content)
    content = re.sub(r'>>>>>>> .*?\n', '\n', content)
    
    with open(filepath, 'w') as f:
        f.write(content)

for file in sys.argv[1:]:
    print(f"Resolving {file}...")
    resolve_file(file)
    print(f"Resolved {file}")