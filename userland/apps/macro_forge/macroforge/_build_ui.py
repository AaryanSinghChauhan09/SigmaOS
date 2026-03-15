"""
Auto-split from userland\apps\macro_forge.py — MacroForge._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random



class MacroForge:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='MACRO FORGE APEX', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [('➕ NEW SEQUENCE', self._new_macro), ('▶️ EXECUTE ALL', self._run_all)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.lib_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=300, padx=15, pady=15)
        self.lib_fr.pack(side='left', fill='y', padx=(0, 20))
        self.lib_fr.pack_propagate(False)
        tk.Label(self.lib_fr, text='CAPABILITY NODES', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        nodes = [('⚡ TRIGGER: System Boot', PAL['success']), ('⚡ TRIGGER: Time = 08:00', PAL['success']), ("⚡ TRIGGER: Connect to 'Home_WiFi'", PAL['success']), ('⚙️ ACTION: Launch Omni-Lens', PAL['accent']), ('⚙️ ACTION: Engage Zenith Focus', PAL['accent']), ('⚙️ ACTION: Parse Clipboard via AI', PAL['accent']), ('⚙️ ACTION: Send via Nexus Share', PAL['accent']), ('🔀 LOGIC: If Battery < 20%', '#1E90FF')]
        for n, c in nodes:
            lbl = tk.Label(self.lib_fr, text=n, font=('Inter', 9, 'bold'), fg='white', bg=c, padx=10, pady=5, cursor='hand2')
            lbl.pack(fill='x', pady=5)
            lbl.bind('<Button-1>', lambda e, text=n: messagebox.showinfo('Node Added', f"Node '{text}' staged in Sequence."))
        self.seq_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.seq_fr.pack(side='left', fill='both', expand=True)
        tk.Label(self.seq_fr, text="ACTIVE SEQUENCE: 'Morning Sovereignty'", font=('Inter', 12, 'bold'), fg=PAL['text'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        self.tree = ttk.Treeview(self.seq_fr, columns=('Type', 'Node Function', 'Priority'), show='headings', style='Forge.Treeview')
        self.tree.heading('Type', text='VECTOR')
        self.tree.column('Type', width=100, anchor='center')
        self.tree.heading('Node Function', text='KERNEL HOOK')
        self.tree.column('Node Function', width=400)
        self.tree.heading('Priority', text='RING')
        self.tree.column('Priority', width=100, anchor='center')
        macros = [('TRIGGER', 'Time = 06:30 AM', 'Ring-3'), ('ACTION', 'Disable Notifications (Focus Engine)', 'Ring-2'), ('ACTION', 'Launch Omni-Lens API (Scan Email)', 'Ring-3'), ('LOGIC', 'Wait for 15 minutes', 'Ring-0'), ('ACTION', 'Terminate Background Apps (Energy Core)', 'Ring-1')]
        for m in macros:
            self.tree.insert('', 'end', values=m)
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', lambda e: messagebox.showinfo('Configure Node', 'Modifying Ring-Level hook parameters...'))
        self.status = tk.Label(self, text='FORGE ENGINE STANDBY | HARDWARE ACCELERATION ON', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
