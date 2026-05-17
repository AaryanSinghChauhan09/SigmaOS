import os
import re

# Fix sigma_vr_studio.cpp
f_vr = 'tools/sigma_vr_studio.cpp'
if os.path.exists(f_vr):
    with open(f_vr, 'r', encoding='utf-8') as f:
        content = f.read()
    if content.startswith('m '):
        content = content[2:]
    with open(f_vr, 'w', encoding='utf-8') as f:
        f.write(content)

# Fix double class attributes in zenith.html
f_html = 'zenith.html'
if os.path.exists(f_html):
    with open(f_html, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Merge class="something" class="auto-style-123" into class="something auto-style-123"
    content = re.sub(r'class="([^"]+)"\s+class="([^"]+)"', r'class="\1 \2"', content)
    
    with open(f_html, 'w', encoding='utf-8') as f:
        f.write(content)

print("Files fixed locally.")
