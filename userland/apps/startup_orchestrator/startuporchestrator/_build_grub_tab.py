"""
Auto-split from userland\apps\startup_orchestrator.py — StartupOrchestrator._build_grub_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random



class StartupOrchestrator:
    def _build_grub_tab(self):
        tk.Label(self.tab_grub, text='SOVEREIGN BOOT LOADER (GRUB2 Usurp)', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        grub_fr = tk.Frame(self.tab_grub, bg=PAL['panel'], padx=20, pady=20)
        grub_fr.pack(fill='both', expand=True)
        settings = [('Default Boot Entry:', ['SigmaOS Sovereign (Kernel 6.x)', 'SigmaOS Recovery (Safe Mode)', 'Windows 11 (Legacy)']), ('Boot Timeout (sec):', ['0', '3', '5', '10']), ('Boot Resolution:', ['1920x1080', '2560x1440', '3840x2160']), ('Kernel Parameters:', None)]
        for label, opts in settings:
            row = tk.Frame(grub_fr, bg=PAL['panel'], pady=8)
            row.pack(fill='x')
            tk.Label(row, text=label, font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel'], width=25, anchor='w').pack(side='left')
            if opts:
                var = tk.StringVar(value=opts[0])
                om = ttk.Combobox(row, values=opts, textvariable=var, font=('Inter', 10), width=35)
                om.pack(side='left', padx=10)
            else:
                e = tk.Entry(row, font=('Consolas', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat', width=45)
                e.insert(0, 'quiet splash loglevel=3 mitigations=off numa_balancing=enable')
                e.pack(side='left', padx=10)
        tk.Button(grub_fr, text='UPDATE BOOT LOADER (grub-mkconfig)', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='black', relief='flat', pady=10, command=lambda: messagebox.showinfo('Boot Loader', 'Sovereign boot config regenerated.\nEFI entry updated to /boot/efi/EFI/sovereign/')).pack(fill='x', pady=(20, 0))
