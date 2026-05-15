import os
import re

wiki_dir = r'c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\wiki_repo'

def fix_markdown(content):
    # MD022: Headers should be surrounded by blank lines
    content = re.sub(r'([^\n])\n(#+ .*)', r'\1\n\n\2', content)
    content = re.sub(r'(#+ .*)\n([^\n])', r'\1\n\n\2', content)
    
    # MD012: Multiple consecutive blank lines
    content = re.sub(r'\n{3,}', '\n\n', content)
    
    return content

for root, dirs, files in os.walk(wiki_dir):
    for file in files:
        if file.endswith('.md'):
            path = os.path.join(root, file)
            try:
                with open(path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                new_content = fix_markdown(content)
                
                if new_content != content:
                    with open(path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"Fixed linting in {file}")
            except Exception as e:
                print(f"Error processing {file}: {e}")

print("Markdown linting fix complete.")
