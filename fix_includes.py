import json
import os
import re

problems = json.loads('''[{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\agents\\\\orchestration\\\\CommandInterpreter.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":25,"endLine":25},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\agents\\\\orchestration\\\\SovereignContainerManager.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":18,"endLine":18},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\agents\\\\policy\\\\GovernanceRules.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":28,"endLine":28},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\agents\\\\quota\\\\QuotaManager.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":27,"endLine":27},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\drivers\\\\linux_distros\\\\hardware\\\\SovereignARM64.cpp","message":"Use of undeclared identifier 'sigma_log_info' (fix available)","severity":"error","startLine":13,"endLine":13},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\memory\\\\memory_manager.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":16,"endLine":16},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\network\\\\SovereignNetStack.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":18,"endLine":18},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\profiles\\\\SovereignProfileManager.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":40,"endLine":40},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\security\\\\audit\\\\SovereignAudit.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":18,"endLine":18},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\SovereignDiag.cpp","message":"Use of undeclared identifier 'sigma_log' (fix available)","severity":"error","startLine":24,"endLine":24},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\SovereignMonitor.cpp","message":"Use of undeclared identifier 'sigma_log' (fix available)","severity":"error","startLine":21,"endLine":21},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\SovereignPQC.cpp","message":"Use of undeclared identifier 'sigma_log' (fix available)","severity":"error","startLine":23,"endLine":23},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\SovereignQuantumHooks.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":18,"endLine":18},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\SovereignScheduler.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":24,"endLine":24},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\kernel\\\\core\\\\SovereignSnap.cpp","message":"Use of undeclared identifier 'sigma_log' (fix available)","severity":"error","startLine":14,"endLine":14},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\recovery\\\\EmergencyLatticeSync.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":18,"endLine":18},{"path":"c:\\\\Users\\\\Aaryan\\\\.gemini\\\\antigravity\\\\scratch\\\\SigmaOS-Repo\\\\recovery\\\\ForensicEngine.cpp","message":"Use of undeclared identifier 'sigma_log' (fixes available)","severity":"error","startLine":18,"endLine":18}]''')

from pathlib import Path

def get_rel_path(filepath, target='include/sigma_log.h'):
    p = Path(filepath)
    repo_root = next(p for p in p.parents if p.name == 'SigmaOS-Repo')
    return os.path.relpath(repo_root / target, p.parent).replace('\\\\', '/')

for prob in problems:
    path = prob['path']
    if os.path.exists(path):
        with open(path, 'r') as f:
            content = f.read()
            
        if 'sigma_log.h' not in content:
            rel = get_rel_path(path)
            content = f'#include "{rel}"\n' + content
            with open(path, 'w') as f:
                f.write(content)
            print(f'Fixed {path}')
