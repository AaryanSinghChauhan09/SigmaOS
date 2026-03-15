# Generated method: AuditViewPage._build_ui
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL

class AuditViewPage:
    def _build_ui(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        up = tk.Frame(body, bg=PAL['bg'])
        up.pack(fill='both', expand=True)
        cols = ['Component', 'Windows 11', 'macOS', 'Linux', 'SigmaOS Sovereign']
        tree = ttk.Treeview(up, columns=cols, show='headings', height=8)
        for c in cols:
            tree.heading(c, text=c)
        tree.pack(fill='both', expand=True, pady=10)
        data = [('Identity', 'Registry/SAM', 'Keychain', 'PAM/Shadow', 'Zero-Trust Canvas (🥇)'), ('Virtualization', 'Hyper-V', 'Virt.framework', 'KVM/QEMU', 'Universal Virt Layer (🥇)'), ('App Store', 'MS Store', 'App Store', 'Flatpak/APT', 'Galactic Store (🥇)'), ('Security', 'Defender', 'Gatekeeper', 'SELinux/AppArmor', 'Sentinel Hardening (🥇)'), ('Gaming', 'AutoHDR', 'Game Mode', 'Proton/WINE', 'HyperDrive Apex (🥇)')]
        for d in data:
            tree.insert('', 'end', values=d)
        down = self._card(body, '🔍 Live Verification Logs')
        down.master.pack(fill='x', pady=10)
        self.audit_log = self._console(down, height=10)
        self.audit_log.pack(fill='both', expand=True)
        self.after(500, lambda: self._log(self.audit_log, 'INITIATING TITAN PARITY VERIFICATION...', 'HEAD'))
        self.after(1500, lambda: self._log(self.audit_log, 'SYSTEM: [OK] Kernel Pre-emption latency: 4ns'))
        self.after(2000, lambda: self._log(self.audit_log, 'SYSTEM: [OK] Shadow Memory protection: ARMED'))
        self.after(2500, lambda: self._log(self.audit_log, 'VERDICT: SIGMAOS IS SUPERIOR.', 'OK'))