# Generated method: ExcelValidator._build_ui
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import random

class ExcelValidator:
    def _build_ui(self):
        main = tk.Frame(self, bg=PAL['bg'], padx=40, pady=40)
        main.pack(fill='both', expand=True)
        head = tk.Frame(main, bg=PAL['bg'])
        head.pack(fill='x', pady=(0, 30))
        tk.Label(head, text='EXCEL', font=('Inter', 24, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        tk.Label(head, text='VALIDATOR PRO', font=('Inter', 24, 'bold'), fg='white', bg=PAL['bg']).pack(side='left', padx=5)
        self.load_fr = tk.Frame(main, bg=PAL['card'], height=120, highlightthickness=1, highlightbackground=PAL['border'])
        self.load_fr.pack(fill='x', pady=(0, 30))
        self.load_fr.pack_propagate(False)
        self.file_lbl = tk.Label(self.load_fr, text='DRAG & DROP DATASET (XLSX, CSV, PARQUET)', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['card'])
        self.file_lbl.pack(expand=True)
        self.load_fr.bind('<Button-1>', lambda e: self._select_file())
        schema_fr = tk.Frame(main, bg=PAL['bg'])
        schema_fr.pack(fill='both', expand=True)
        self.rules_fr = tk.Frame(schema_fr, bg=PAL['card'], width=300, padx=20, pady=20)
        self.rules_fr.pack(side='left', fill='y', padx=(0, 20))
        self.rules_fr.pack_propagate(False)
        tk.Label(self.rules_fr, text='COMPLIANCE RULES', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['card']).pack(anchor='w', pady=(0, 15))
        rules = [('ISO-20547 Schema', True), ('Strict Type Check', True), ('Empty Cell Purge', False), ('Unique Key Audit', True), ('Outlier Detection', False), ('PQC Encryption', True)]
        self.rule_vars = {}
        for r, d in rules:
            v = tk.BooleanVar(value=d)
            self.rule_vars[r] = v
            tk.Checkbutton(self.rules_fr, text=r, variable=v, bg=PAL['card'], fg=PAL['text'], selectcolor='#000', font=('Inter', 9)).pack(anchor='w', pady=5)
        self.report_fr = tk.Frame(schema_fr, bg=PAL['card'], padx=25, pady=25)
        self.report_fr.pack(side='right', fill='both', expand=True)
        tk.Label(self.report_fr, text='VALIDATION REPORT', font=('Inter', 8, 'bold'), fg=PAL['accent'], bg=PAL['card']).pack(anchor='w')
        self.report_txt = tk.Text(self.report_fr, bg='#000', fg=PAL['dim'], font=('JetBrains Mono', 9), borderwidth=0, padx=15, pady=15)
        self.report_txt.pack(fill='both', expand=True, pady=(15, 0))
        foot = tk.Frame(main, bg=PAL['bg'], pady=30)
        foot.pack(side='bottom', fill='x')
        tk.Button(foot, text='🛡️ TRIGGER VALIDATION', font=('Inter', 11, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=40, pady=15, command=self._validate).pack(side='right')
        self.status = tk.Label(self, text='ENGINE: READY | BUFFER: COLD', bg=PAL['card'], fg=PAL['dim'], font=('Inter', 8, 'bold'), pady=8)
        self.status.pack(side='bottom', fill='x')