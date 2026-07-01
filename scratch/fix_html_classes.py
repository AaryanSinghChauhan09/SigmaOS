import re
import os

def fix_html_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Regex to find duplicate class attributes and merge them
    # Example: class="a b" ... class="c d" -> class="a b c d"
    pattern = re.compile(r'class="([^"]*)"([^>]*)\s+class="([^"]*)"')
    
    new_content = content
    while True:
        match = pattern.search(new_content)
        if not match:
            break
        
        c1 = match.group(1).strip()
        middle = match.group(2)
        c2 = match.group(3).strip()
        
        merged_class = f'class="{c1} {c2}"'
        # Replace only this occurrence
        start, end = match.span()
        new_content = new_content[:start] + merged_class + middle + new_content[end:]

    if new_content != content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Fixed {filepath}")
    else:
        print(f"No changes needed for {filepath}")

files_to_fix = [
    r'c:\Users\Aaryan\.\gemini\antigravity\scratch\SigmaOS\zenith.html',
    r'c:\Users\Aaryan\.\gemini\antigravity\scratch\SigmaOS\installer.html',
    r'c:\Users\Aaryan\.\gemini\antigravity\scratch\SigmaOS\roadmap.html',
    r'c:\Users\Aaryan\.\gemini\antigravity\scratch\SigmaOS\index.html'
]

for f in files_to_fix:
    if os.path.exists(f):
        fix_html_file(f)
