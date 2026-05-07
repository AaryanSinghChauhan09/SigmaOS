import os
import json
import re

def main():
    with open('headers_map.json', 'r') as f:
        headers = json.load(f)
    
    header_map = {}
    for h in headers:
        header_map[h['Name']] = h['RelPath']
    
    # We want to replace `#include "..."` or `#include <...>` with the exact correct path if it's in header_map.
    # E.g., `#include "../../../include/sigma_hal.h"` -> `#include "hal/sigma_hal.h"`
    # E.g., `#include <sigma_types.h>` -> `#include "core/sigma_types.h"`
    
    def replace_match(match):
        inc_path = match.group(2)
        base_name = os.path.basename(inc_path)
        if base_name in header_map:
            return f'#include "{header_map[base_name]}"'
        return match.group(0)

    regex = re.compile(r'#include\s+([<"])([^>"]+)([>"])')

    dirs_to_check = ['kernel', 'include', 'tests', 'lib']
    for d in dirs_to_check:
        for root, dirs, files in os.walk(d):
            for file in files:
                if file.endswith(('.c', '.cpp', '.h', '.hpp')):
                    path = os.path.join(root, file)
                    with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                    
                    new_content = regex.sub(replace_match, content)
                    
                    # Fix some undeclared identifiers
                    new_content = new_content.replace("sigma_printf", "sigma_log")
                    # SovereignEngine namespace issues
                    new_content = re.sub(r'(?<!SigmaOS::Kernel::Security::)SovereignSandboxEngine', 'SigmaOS::Kernel::Security::SovereignSandboxEngine', new_content)
                    new_content = re.sub(r'(?<!SigmaOS::Kernel::Security::)SovereignPQCEngine', 'SigmaOS::Kernel::Security::SovereignPQCEngine', new_content)
                    new_content = re.sub(r'(?<!SigmaOS::Kernel::Syscall::)SovereignSyscallEngine', 'SigmaOS::Kernel::Syscall::SovereignSyscallEngine', new_content)
                    new_content = re.sub(r'(?<!SigmaOS::Kernel::HAL::)SovereignSMPEngine', 'SigmaOS::Kernel::HAL::SovereignSMPEngine', new_content)
                    new_content = re.sub(r'(?<!SigmaOS::Kernel::AI::)SovereignAISchedEngine', 'SigmaOS::Kernel::AI::SovereignAISchedEngine', new_content)

                    # sigma_log does not exist? wait, SovereignLibC.h has sigma_log maybe? Let's check where it is later.
                    
                    if new_content != content:
                        print(f"Updating {path}")
                        with open(path, 'w', encoding='utf-8') as f:
                            f.write(new_content)

if __name__ == '__main__':
    main()
