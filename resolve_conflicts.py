import sys

def resolve_file(filepath):
    with open(filepath, 'r') as f:
        lines = f.readlines()
    
    out = []
    in_conflict = False
    part1 = []
    part2 = []
    current_part = 1
    
    for line in lines:
        if line.startswith('<<<<<<<'):
            in_conflict = True
            part1 = []
            part2 = []
            current_part = 1
        elif line.startswith('======='):
            current_part = 2
        elif line.startswith('>>>>>>>'):
            in_conflict = False
            # Append both parts
            out.extend(part1)
            out.extend(part2)
        else:
            if in_conflict:
                if current_part == 1:
                    part1.append(line)
                else:
                    part2.append(line)
            else:
                out.append(line)
                
    with open(filepath, 'w') as f:
        f.writelines(out)

for file in sys.argv[1:]:
    resolve_file(file)
