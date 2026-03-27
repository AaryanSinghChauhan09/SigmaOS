import os
import re

files = [
    'C:/Users/SOVEREIGN_USER/.gemini/antigravity/scratch/SigmaOS/userland/apps/sigma_bharat_legal_suite.html',
    'C:/Users/SOVEREIGN_USER/.gemini/antigravity/scratch/SigmaOS/userland/apps/sigma_bharat_procedural_matrix.html',
    'C:/Users/SOVEREIGN_USER/.gemini/antigravity/scratch/SigmaOS/userland/apps/sigma_bharat_compliance_assistant.html'
]

for f_path in files:
    if not os.path.exists(f_path): continue
    with open(f_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 1. user-select fix
    if '{ box-sizing: border-box; margin: 0; padding: 0; }' in content:
        content = content.replace('{ box-sizing: border-box; margin: 0; padding: 0; }', '{ box-sizing: border-box; margin: 0; padding: 0; -webkit-user-select: none; user-select: none; }')
    else:
        content = content.replace('user-select: none;', '-webkit-user-select: none; user-select: none;')

    # 2. backdrop-filter fix
    if 'backdrop-filter: blur' in content and '-webkit-backdrop-filter' not in content:
        content = re.sub(r'backdrop-filter:\s*blur\((.*?)\);', r'-webkit-backdrop-filter: blur(\1);\n            backdrop-filter: blur(\1);', content)

    # 3. Extract inline styles
    style_idx = 1
    extracted_styles = []

    def replacer(match):
        global style_idx
        full_tag = match.group(0)
        
        # Don't touch <style> tags or </style>
        if full_tag.startswith('<script>') or full_tag.startswith('<style>') or full_tag.startswith('</style>'):
            return full_tag

        style_match = re.search(r'style=\"([^\"]+)\"|style=\'([^\']+)\'', full_tag)
        if not style_match: return full_tag
        style_content = style_match.group(1) or style_match.group(2)
        
        cname = f'sigma-style-{style_idx}'
        extracted_styles.append(f'.{cname} {{ {style_content} }}')
        
        # Remove style attribute
        full_tag = re.sub(r'\s*style=\"[^\"]+\"|\s*style=\'[^\']+\'', '', full_tag)
        
        # Add inside class
        if 'class=' in full_tag:
            full_tag = re.sub(r'class=\"([^\"]+)\"', f'class=\"\\1 {cname}\"', full_tag)
            full_tag = re.sub(r'class=\'([^\']+)\'', f'class=\'\\1 {cname}\'', full_tag)
        else:
            full_tag = full_tag.rstrip('>') + f' class=\"{cname}\">'
            
        style_idx += 1
        return full_tag

    # Use re to find all HTML opening and closing tags, but only replace inside tags
    content = re.sub(r'<[^>]+>', replacer, content)

    if extracted_styles:
        styles_str = '\n        ' + '\n        '.join(extracted_styles) + '\n    </style>'
        content = content.replace('</style>', styles_str)

    with open(f_path, 'w', encoding='utf-8') as f:
        f.write(content)
        
print("Successfully refactored all HTML lints via OOP Python extraction.")
