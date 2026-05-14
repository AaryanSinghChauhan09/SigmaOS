def despace(text):
    return text[::2]

with open('style.css', 'r', encoding='utf-8') as f:
    lines = f.readlines()

new_lines = []
for i, line in enumerate(lines):
    if i + 1 >= 259:
        # Check if it's really spaced out
        # A simple check: if more than 50% of chars are spaces and they alternate
        stripped = line.strip()
        if len(stripped) > 4:
            # Try to despace
            despaced = despace(line)
            # Check if despaced looks like CSS (contains { or : or ; or .)
            if any(c in despaced for c in '{}:;.'):
                new_lines.append(despaced)
                continue
    new_lines.append(line)

with open('style.css', 'w', encoding='utf-8') as f:
    f.writelines(new_lines)
print("Despaced style.css starting from line 259")
