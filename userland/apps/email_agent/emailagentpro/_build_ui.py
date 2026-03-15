"""
Auto-split from userland\apps\email_agent.py — EmailAgentPro._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext
import os
import time
import random



class EmailAgentPro:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=40, pady=25)
        head.pack(side='top', fill='x')
        tk.Label(head, text='SOVEREIGN EMAIL AGENT', font=('Inter', 24, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        self.stats = tk.Label(head, text='4 UNREAD | NEURAL TRIAGE: ACTIVE', font=('Inter', 8, 'bold'), fg=PAL['unread'], bg=PAL['bg'])
        self.stats.pack(side='right', pady=15)
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        self.side_fr = tk.Frame(body, bg=PAL['sidebar'], width=280, padx=20, pady=30)
        self.side_fr.pack(side='left', fill='y')
        self.side_fr.pack_propagate(False)
        tk.Label(self.side_fr, text='SOVEREIGN ACCOUNTS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w', pady=(0, 20))
        accounts = [('✉️ PRIMARY MESH', PAL['accent']), ('📦 COLD VAULT', PAL['dim']), ('⚙️ SYSTÈME', PAL['dim'])]
        for acc, clr in accounts:
            f = tk.Frame(self.side_fr, bg=PAL['sidebar'], pady=12, cursor='hand2')
            f.pack(fill='x')
            tk.Label(f, text=acc, font=('Inter', 11, 'bold'), fg=clr, bg=PAL['sidebar']).pack(side='left')
            if 'PRIMARY' in acc:
                tk.Label(f, text='●', fg=PAL['unread'], bg=PAL['sidebar']).pack(side='right')
        self.list_fr = tk.Frame(body, bg=PAL['bg'], width=450, padx=15, pady=20)
        self.list_fr.pack(side='left', fill='y', padx=10)
        self.list_fr.pack_propagate(False)
        tk.Label(self.list_fr, text='NEURAL INBOX', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        for mail in self.inbox:
            m_fr = tk.Frame(self.list_fr, bg=PAL['panel'], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL['border'])
            m_fr.pack(fill='x', pady=5)
            tk.Label(m_fr, text=mail['from'], font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(m_fr, text=mail['subject'], font=('Inter', 9, 'bold'), fg=PAL['accent'], bg=PAL['panel']).pack(anchor='w', pady=2)
            tk.Label(m_fr, text=mail['summary'], font=('Inter', 8), fg=PAL['dim'], bg=PAL['panel'], height=2, wraplength=380, justify='left').pack(anchor='w')
            tk.Label(m_fr, text=mail['time'], font=('Inter', 7), fg=PAL['dim'], bg=PAL['panel']).pack(side='right')
        self.read_fr = tk.Frame(body, bg=PAL['panel'], padx=30, pady=30)
        self.read_fr.pack(side='right', fill='both', expand=True)
        tk.Label(self.read_fr, text='AGENTIC COMPOSER (AI-TRIAGE)', font=('Inter', 10, 'bold'), fg=PAL['success'], bg=PAL['panel']).pack(anchor='w')
        self.read_txt = scrolledtext.ScrolledText(self.read_fr, bg='#000', fg=PAL['text'], font=('Inter', 12), borderwidth=0, padx=25, pady=25)
        self.read_txt.pack(fill='both', expand=True, pady=20)
        self.read_txt.insert('1.0', '[SELECT EMAIL TO INTERACT]')
        comp_fr = tk.Frame(self.read_fr, bg='#000', height=80, highlightthickness=1, highlightbackground=PAL['border'])
        comp_fr.pack(side='bottom', fill='x')
        self.comp_inp = tk.Entry(comp_fr, bg='#000', fg=PAL['text'], insertbackground='white', font=('Inter', 11), borderwidth=0)
        self.comp_inp.pack(side='left', fill='both', expand=True, padx=20)
        tk.Button(comp_fr, text='🪄 DRAFT WITH AI', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=25, pady=12, command=self._draft_ai).pack(side='right')
        self.status = tk.Label(self, text='INBOX SYNCHRONIZED | ENCRYPTION: RSA-4096 VALID | OFFLINE MODE', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
