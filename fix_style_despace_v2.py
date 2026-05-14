def despace(text):
    # Try both starting at 0 and starting at 1
    d0 = text[::2]
    d1 = text[1::2]
    
    # Return the one that looks more like code
    score0 = sum(1 for c in d0 if c in '{}:;./*')
    score1 = sum(1 for c in d1 if c in '{}:;./*')
    
    return d0 if score0 >= score1 else d1

with open('style.css', 'r', encoding='utf-8') as f:
    lines = f.readlines()

new_lines = []
for i, line in enumerate(lines):
    if i + 1 >= 260:
        stripped = line.strip()
        if len(stripped) > 4 and ' ' in stripped:
            despaced = despace(line)
            if any(c in despaced for c in '{}:;./*'):
                new_lines.append(despaced.rstrip() + '\n')
                continue
    new_lines.append(line)

with open('style.css', 'w', encoding='utf-8') as f:
    f.writelines(new_lines)
print("Improved despacing of style.css")
