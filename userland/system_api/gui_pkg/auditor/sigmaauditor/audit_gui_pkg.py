# Generated method: SigmaAuditor.audit_gui_pkg
import os
import sys
import importlib

class SigmaAuditor:
    def audit_gui_pkg(self):
        print('[AUDIT] Verifying UI modular packages...')
        pkg_dir = os.path.join(root, 'userland', 'system_api', 'gui_pkg')
        if not os.path.exists(pkg_dir):
            self.errors.append('CRITICAL: gui_pkg directory missing!')
            return
        required_pages = ['base_page.py', 'dashboard.py', 'apex_page.py', 'nexus_page.py', 'arcade.py']
        for p in required_pages:
            if not os.path.exists(os.path.join(pkg_dir, p)):
                self.errors.append(f'Modular page missing: {p}')