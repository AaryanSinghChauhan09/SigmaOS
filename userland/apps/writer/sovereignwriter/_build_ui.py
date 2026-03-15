"""
Auto-split from userland\apps\writer.py — SovereignWriter._build_ui
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import os



class SovereignWriter:
    def _build_ui(self):
        self.toolbar = tk.Frame(self, bg=PAL['bg'], height=60, padx=20)
        self.toolbar.pack(side='top', fill='x')
        tk.Label(self.toolbar, text=f"{ICONS.get('writer', '🖋️')} WRITER PRO", font=('Inter', 12, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.toolbar, bg=PAL['bg'])
        btn_fr.pack(side='left', padx=30)
        tool_btns = [('B', 'bold'), ('I', 'italic'), ('U', 'underline'), (f"{ICONS.get('intelligence', '✨')} AI", 'ai')]
        for txt, tag in tool_btns:
            bg = PAL['sidebar'] if tag != 'ai' else PAL['accent']
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=bg, fg='white', relief='flat', padx=15, pady=8).pack(side='left', padx=2)
        tk.Button(self.toolbar, text=f"{ICONS.get('snapshots', '💾')} SAVE", font=('Inter', 8, 'bold'), bg=PAL['success'], fg='white', relief='flat', padx=20, pady=8, command=self.save).pack(side='right', padx=5)
        tk.Button(self.toolbar, text=f"{ICONS.get('minimalist', '🧘')} ZEN MODE", font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=20, pady=8, command=self.toggle_zen).pack(side='right', padx=5)
        self.main_fr = tk.Frame(self, bg=PAL['bg'])
        self.main_fr.pack(fill='both', expand=True)
        self.sidebar = tk.Frame(self.main_fr, bg=PAL['sidebar'], width=220, padx=15, pady=20)
        self.sidebar.pack(side='left', fill='y')
        self.sidebar.pack_propagate(False)
        tk.Label(self.sidebar, text=f"{ICONS.get('search', '📑')} OUTLINE", font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w')
        sections = ['Introduction', 'Strategic Overview', 'Quantum Mesh Specs', 'Conclusion']
        for s in sections:
            tk.Label(self.sidebar, text=f'• {s}', font=('Inter', 10), fg=PAL['text'], bg=PAL['sidebar'], pady=8, cursor='hand2').pack(anchor='w')
        self.ai_side = tk.Frame(self.main_fr, bg=PAL['sidebar'], width=250, padx=15, pady=20)
        self.ai_side.pack(side='right', fill='y')
        self.ai_side.pack_propagate(False)
        tk.Label(self.ai_side, text=f"{ICONS.get('intelligence', '🧠')} AI ASSISTANT", font=('Inter', 8, 'bold'), fg=PAL['accent'], bg=PAL['sidebar']).pack(anchor='w')
        self.ai_box = tk.Text(self.ai_side, bg='#000', fg=PAL['success'], font=('Consolas', 9), height=15, borderwidth=0, padx=10, pady=10)
        self.ai_box.pack(fill='x', pady=15)
        self.ai_box.insert('1.0', "[AGENT] Ready to assist.\n\nSuggested Next Sentence:\n'The scalability of the Aether Mesh ensures 100% reliability.'")
        self.editor_fr = tk.Frame(self.main_fr, bg=PAL['bg'], padx=40, pady=20)
        self.editor_fr.pack(fill='both', expand=True)
        self.editor = scrolledtext.ScrolledText(self.editor_fr, font=('Inter', 13), padx=80, pady=80, bg='#FFFFFF', fg='#111', insertbackground='black', borderwidth=0, undo=True, highlightthickness=1, highlightbackground=PAL['border'])
        self.editor.pack(fill='both', expand=True)
        self.editor.insert('1.0', 'Welcome to Sovereign Writer Apex Pro.\n\nStart your mission-critical documentation here.')
        self.status = tk.Label(self, text='WORDS: 12 | ENCRYPTION: SHA-256 | LATENCY: 0ms', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')
