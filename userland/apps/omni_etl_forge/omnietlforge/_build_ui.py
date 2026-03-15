"""
Auto-split from userland\apps\omni_etl_forge.py — OmniETLForge._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading



class OmniETLForge:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='OMNI-ETL PIPELINE FORGE', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [('🧠 AUTO-SCHEMA', self._auto_schema), ('❄️ ZERO-COPY CLONE', self._zero_copy), ('▶️ COMPILE & RUN DAG', self._run_dag)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.conn_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=230, padx=15, pady=15)
        self.conn_fr.pack(side='left', fill='y', padx=(0, 20))
        self.conn_fr.pack_propagate(False)
        tk.Label(self.conn_fr, text='SOURCE CONNECTORS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        connectors = ['PostgreSQL Core', 'MongoDB Matrix', 'Salesforce API', 'AWS S3 Bucket', 'Stripe Webhooks']
        for c in connectors:
            lbl = tk.Label(self.conn_fr, text=f'📥 {c}', font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['sidebar'], padx=10, pady=8, cursor='hand2')
            lbl.pack(fill='x', pady=5)
            lbl.bind('<Button-1>', lambda e, n=c: self._add_node(n, 'IN'))
        tk.Label(self.conn_fr, text='TARGET SINKS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(20, 10))
        sinks = ['Sovereign Lakehouse', 'Quantum Data Mart', 'Real-Time Redis']
        for s in sinks:
            lbl = tk.Label(self.conn_fr, text=f'📤 {s}', font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['sidebar'], padx=10, pady=8, cursor='hand2')
            lbl.pack(fill='x', pady=5)
            lbl.bind('<Button-1>', lambda e, n=s: self._add_node(n, 'OUT'))
        self.dag_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.dag_fr.pack(side='left', fill='both', expand=True)
        tk.Label(self.dag_fr, text='VISUAL ORCHESTRATION (DAG)', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w')
        self.canvas = tk.Canvas(self.dag_fr, bg=PAL['sidebar'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, pady=10)
        self.canvas.create_text(300, 250, text='CLICK CONNECTORS TO BUILD PIPELINE', fill=PAL['dim'], font=('Inter', 12, 'bold'))
        self.dbt_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=280, padx=15, pady=15)
        self.dbt_fr.pack(side='right', fill='y', padx=(20, 0))
        self.dbt_fr.pack_propagate(False)
        tk.Label(self.dbt_fr, text='TRANSFORMATIONS (dbt)', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        self.sql_text = tk.Text(self.dbt_fr, bg=PAL['bg'], fg=PAL['accent'], font=('Consolas', 9), relief='flat')
        self.sql_text.pack(fill='both', expand=True, pady=5)
        self.sql_text.insert(tk.END, "SELECT\n  id,\n  neural_hash(email) as usr,\n  revenue * 1.05 as proj_rev\nFROM\n  {{ ref('raw_stripe') }}\nWHERE\n  status = 'active'")
        tk.Button(self.dbt_fr, text='MATERIALIZE VIEW', font=('Inter', 8, 'bold'), bg=PAL['accent'], fg='black', relief='flat', pady=8, command=self._materialize).pack(fill='x', pady=(10, 0))
        self.status = tk.Label(self, text='OMNI-ETL FORGE IDLE | ZERO-COPY LAKEHOUSE MOUNTED', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
