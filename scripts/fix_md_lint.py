import os
import re

def fix_markdown(content):
    # MD009: No trailing spaces
    content = re.sub(r'[ \t]+$', '', content, flags=re.MULTILINE)
    
    # MD022: Blanks around headings
    # Ensure newline before #
    content = re.sub(r'([^\n])\n(#+ )', r'\1\n\n\2', content)
    # Ensure newline after #
    content = re.sub(r'(#+ [^\n]+)\n([^\n])', r'\1\n\n\2', content)
    
    # MD032: Blanks around lists
    # Ensure newline before list marker (-, *, + or 1.)
    content = re.sub(r'([^\n])\n([-*+] |[0-9]+\. )', r'\1\n\n\2', content)
    # Ensure newline after list
    # (This is harder to do safely with regex, but let's try a basic one)
    # content = re.sub(r'(\n[-*+] [^\n]+)\n([^\n])', r'\1\n\n\2', content)
    
    # MD036: No emphasis as heading
    # Replace lines that are just **Text** or *Text* with ### Text
    content = re.sub(r'^\s*(\*\*|__)([^*_]+)\1\s*$', r'### \2', content, flags=re.MULTILINE)
    
    # MD004: Unordered list style (Force -)
    content = re.sub(r'^[ \t]*\* ', r'- ', content, flags=re.MULTILINE)
    
    return content

for root, dirs, files in os.walk("."):
    for file in files:
        if file.endswith(".md"):
            path = os.path.join(root, file)
            try:
                with open(path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                new_content = fix_markdown(content)
                
                if content != new_content:
                    with open(path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"Linted: {path}")
            except Exception as e:
                print(f"Error linting {path}: {e}")
