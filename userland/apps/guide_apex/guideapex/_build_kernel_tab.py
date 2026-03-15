"""
Auto-split from userland\apps\guide_apex.py — GuideApex._build_kernel_tab
"""

import tkinter as tk
from tkinter import ttk, scrolledtext
import time



class GuideApex:
    def _build_kernel_tab(self):
        tab = tk.Frame(self.nb, bg=PAL['bg'], padx=25, pady=25)
        self.nb.add(tab, text='  ⚙️ Kernel Architecture  ')
        canvas = tk.Canvas(tab, bg='#050508', highlightthickness=1, highlightbackground=PAL['accent'])
        canvas.pack(side='left', fill='both', expand=True, padx=(0, 20))
        canvas.create_rectangle(50, 50, 450, 100, fill=PAL['accent'], outline='white', width=2)
        canvas.create_text(250, 75, text='USER SPACE (Sovereign Apps)', fill='white', font=('Inter Bold', 10))
        canvas.create_rectangle(50, 120, 450, 170, fill='#1A1B23', outline='white')
        canvas.create_text(250, 145, text='AI ORCHESTRATION LAYER', fill=PAL['text'], font=('Inter', 10))
        canvas.create_rectangle(50, 190, 450, 240, fill='#12131A', outline=PAL['gold'])
        canvas.create_text(250, 215, text='SIGMA KERNEL (Python/C/Rust)', fill=PAL['gold'], font=('Inter Bold', 10))
        canvas.create_rectangle(50, 260, 450, 310, fill='#08080C', outline='white')
        canvas.create_text(250, 285, text='HARDWARE ABSTRACTION (HAL)', fill=PAL['dim'], font=('Inter', 10))
        info = tk.Frame(tab, bg=PAL['bg'], width=300)
        info.pack(side='right', fill='y')
        info.pack_propagate(False)
        tk.Label(info, text='KERNEL SPECS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w')
        tk.Label(info, text='• Hybrid Core: Py/C/Rust\n• Zero-Trust: Mandatory\n• FS: Virtual Sandbox\n• Sync: P2P Mesh', font=('Inter', 10), fg=PAL['text'], bg=PAL['bg'], justify='left', pady=15).pack(anchor='w')
