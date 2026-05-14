import re

with open('zenith_desktop.css', 'r', encoding='utf-8') as f:
    content = f.read()

# Remove the malformed lines
new_content = re.sub(r'^\s*-webkit--webkit-backdrop-filter:.*?;backdrop-filter:.*?;$\n', '', content, flags=re.MULTILINE)

# Also handle cases where it's not on its own line (like in line 697)
new_content = re.sub(r'-webkit--webkit-backdrop-filter:.*?;backdrop-filter:.*?;', '', new_content)

if new_content != content:
    with open('zenith_desktop.css', 'w', encoding='utf-8') as f:
        f.write(new_content)
    print("Fixed malformed backdrop-filter lines in zenith_desktop.css")
else:
    print("No malformed lines found or already fixed.")
