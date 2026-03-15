# Generated method: LinuxParityPage._build_ui
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_BOLD, FONT_SMALL

class LinuxParityPage:
    def _build_ui(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        l_fr = tk.Frame(body, bg=PAL['bg2'], width=450)
        l_fr.pack(side='left', fill='both', padx=5)
        l_fr.pack_propagate(False)
        gap_c = self._card(l_fr, '📊 Distro Gap Analysis')
        gap_c.master.pack(fill='x', pady=5)
        ttk.Button(gap_c, text='Audit: Sigma vs Kali', command=lambda: self._log_linux_gap('Kali')).pack(side='left', padx=5)
        ttk.Button(gap_c, text='Audit: Sigma vs Arch', command=lambda: self._log_linux_gap('Arch')).pack(side='left', padx=5)
        ttk.Button(gap_c, text='🛡️ RHEL Security Scan', command=self._run_sec_audit).pack(side='left', padx=5)
        pm_c = self._card(l_fr, '📦 Multi-Stack Package Manager')
        pm_c.master.pack(fill='x', pady=5)
        tk.Label(pm_c, text='Translates: apt, dnf, pacman, snap, flatpak', font=('Segoe UI', 8), bg=PAL['card'], fg=PAL['dim']).pack()
        ttk.Button(pm_c, text='Sync Repos', command=lambda: self._log_linux(self.kernel.linux_parity.pm.sync_repos())).pack(side='left', padx=5)
        ttk.Button(pm_c, text='Upgrade System', command=lambda: self._log_linux(self.kernel.linux_parity.pm.upgrade_system())).pack(side='left', padx=5)
        dp_c = self._card(l_fr, '🎭 Persona Mode (Distro Mimicry)')
        dp_c.master.pack(fill='x', pady=5)
        tk.Label(dp_c, text='Apply Distro-specific optimizations instantly.', font=('Segoe UI', 8), bg=PAL['card'], fg=PAL['dim']).pack()
        for d in ['Ubuntu', 'Kali', 'Arch', 'Fedora', 'Pop!', 'Zorin', 'Gentoo', 'Alpine']:
            ttk.Button(dp_c, text=d, command=lambda x=d: self._apply_distro_tuning(x)).pack(side='left', padx=2)
        r_fr = tk.Frame(body, bg=PAL['bg'])
        r_fr.pack(side='left', fill='both', expand=True, padx=5)
        console_c = self._card(r_fr, '📟 Linux Parity Ops & Log')
        console_c.master.pack(fill='both', expand=True)
        self._linux_log = self._console(console_c, height=25)
        self._linux_log.pack(fill='both', expand=True)