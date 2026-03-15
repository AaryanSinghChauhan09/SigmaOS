"""
Auto-split from userland\system_api\antigravity_core.py — AntigravityLayer.spawn_micro_vm
"""

import os
import hashlib
import time



class AntigravityLayer:
    def spawn_micro_vm(self, agent_name: str, allowed_dirs: list):
        """The Zero-Trust Agent Sandbox (Firecracker-style simulation)"""
        vm_id = f'mvm-{hashlib.md5(agent_name.encode()).hexdigest()[:6]}'
        self._active_micro_vms[vm_id] = {'agent': agent_name, 'fs_bounds': allowed_dirs, 'status': 'RUNNING'}
        return vm_id
