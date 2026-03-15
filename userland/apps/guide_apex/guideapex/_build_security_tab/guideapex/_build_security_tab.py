# Generated method: GuideApex._build_security_tab
import tkinter as tk
from tkinter import ttk, scrolledtext
import time

class GuideApex:
    def _build_security_tab(self):
        tab = tk.Frame(self.nb, bg=PAL['bg'], padx=25, pady=25)
        self.nb.add(tab, text='  🛡️ Security Protocols  ')
        tk.Label(tab, text='ZERO-TRUST HIERARCHY', font=('Inter Bold', 12), fg='white', bg=PAL['bg']).pack(anchor='w', pady=(0, 20))
        protocols = [('Vanguard Crypto', 'ChaCha20-Poly1305 encryption on all VFS blocks.'), ('Hex-Scan Heuristics', 'Industry-standard behavioral analysis for threat detection.'), ('PID Isolation', 'Every app runs in a dedicated, memory-fenced partition.'), ('Kill-Switch', 'One-click OS lockdown via the Security Guardian.')]
        for name, desc in protocols:
            f = tk.Frame(tab, bg=PAL['card'], padx=15, pady=10, highlightthickness=1, highlightbackground='#333')
            f.pack(fill='x', pady=5)
            tk.Label(f, text=name, font=('Inter Bold', 10), fg=PAL['accent'], bg=PAL['card']).pack(side='left')
            tk.Label(f, text=f' — {desc}', font=('Inter', 9), fg=PAL['dim'], bg=PAL['card']).pack(side='left')