import os
import re

def fix_markdown(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    out_lines = []
    
    # Pre-computation
    for i, line in enumerate(lines):
        # Fix MD009 (Trailing spaces)
        line = line.rstrip() + '\n'
        
        # FIx MD030 (List marker space - multiple spaces after asterisk or dash)
        # matches "-   " or "*   " at the start of line with spaces before it
        line = re.sub(r'^(\s*[-*]) {2,}', r'\1 ', line)

        out_lines.append(line)
        
    final_lines = []
    for i, line in enumerate(out_lines):
        # Fix MD022 (Headings surrounded by blank lines)
        if re.match(r'^\#{1,6}\s', line):
            # Ensure blank line before
            if i > 0 and out_lines[i-1].strip() != '' and not re.match(r'^\s*$', final_lines[-1]):
                final_lines.append('\n')
            final_lines.append(line)
            # Ensure blank line after wait we inject next loop if needed, but actually we can just insert it here
            if (i+1 < len(out_lines)) and out_lines[i+1].strip() != '':
                 final_lines.append('\n')
            continue
            
        # Fix MD032 (Lists surrounded by blank lines)
        is_list = bool(re.match(r'^\s*[-*]\s', line))
        was_list = i > 0 and bool(re.match(r'^\s*[-*]\s', out_lines[i-1]))
        if is_list and not was_list:
            if final_lines and final_lines[-1].strip() != '':
                final_lines.append('\n')

        # Fix MD060 (Table pipe style space parsing)
        if "|" in line and "-|-" in line.replace(" ", ""):
            line = line.replace("|-", "| -").replace("-|", "- |")
            line = re.sub(r'\|\s*\-\s*\|', '| --- |', line)
            
        final_lines.append(line)

    with open(filepath, 'w', encoding='utf-8') as f:
        f.writelines(final_lines)

    print(f"Fixed {filepath}")

for root, dirs, files in os.walk('.'):
    for fn in files:
        if fn.endswith('.md'):
            fix_markdown(os.path.join(root, fn))
