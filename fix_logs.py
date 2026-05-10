import os
import glob

cpp_files = glob.glob('**/*.cpp', recursive=True)
for f in cpp_files:
    try:
        with open(f, 'r', encoding='utf-8') as file:
            content = file.read()
        
        if 'sigma_log' in content and 'sigma_log.h' not in content:
            # find the first #include and add before it
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.startswith('#include'):
                    lines.insert(i, '#include "sigma_log.h"')
                    break
            else:
                lines.insert(0, '#include "sigma_log.h"')
            
            with open(f, 'w', encoding='utf-8') as file:
                file.write('\n'.join(lines))
            print(f'Fixed {f}')
    except Exception as e:
        pass
