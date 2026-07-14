import sys

with open("DEFEATING_LINUX_DISTROS_BLUEPRINT.md", "r", encoding="utf-8") as f:
    lines = f.readlines()

new_lines = []
in_conflict = False
take_this = False

for line in lines:
    if line.startswith("<<<<<<< HEAD"):
        in_conflict = True
        take_this = False
    elif line.startswith("======="):
        take_this = True
    elif line.startswith(">>>>>>> origin/jules-5658883166131122080-74df2d50"):
        in_conflict = False
        take_this = False
    else:
        if not in_conflict:
            new_lines.append(line)
        elif take_this:
            new_lines.append(line)

with open("DEFEATING_LINUX_DISTROS_BLUEPRINT.md", "w", encoding="utf-8") as f:
    f.writelines(new_lines)
