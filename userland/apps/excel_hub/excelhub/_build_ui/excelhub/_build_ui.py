# Generated method: ExcelHub._build_ui
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import time

class ExcelHub:
    def _build_ui(self):
        self.toolbar = tk.Frame(self, bg=PAL['bg'], height=60, padx=25)
        self.toolbar.pack(side='top', fill='x')
        tk.Label(self.toolbar, text='EXCEL PRO', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.toolbar, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tools = [('📁 OPEN', self.load), ('💾 SAVE', self.save), ('🚀 NEURAL-AUTO', self._run_ai), ('🧼 DEEP-CLEAN', self._run_clean)]
        for txt, cmd in tools:
            tk.Button(btn_fr, text=txt, font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.panes = ttk.PanedWindow(self.workspace, orient='horizontal')
        self.panes.pack(fill='both', expand=True)
        self.side_fr = tk.Frame(self.panes, bg=PAL['sidebar'], width=240, padx=20, pady=25)
        self.panes.add(self.side_fr, weight=1)
        self.side_fr.pack_propagate(False)
        tk.Label(self.side_fr, text='WORKBOOK NAVIGATOR', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w')
        sheets = [('📊 Master_Intel', 'success'), ('📈 Mesh_Revenue', 'text'), ('🔍 Forensic_Audit', 'dim')]
        for s, col in sheets:
            tk.Label(self.side_fr, text=f'• {s}', font=('Inter', 10), fg=PAL[col], bg=PAL['sidebar'], pady=10, cursor='hand2').pack(anchor='w')
        self.center_fr = tk.Frame(self.panes, bg=PAL['bg'], padx=20)
        self.panes.add(self.center_fr, weight=4)
        self.tabs = ttk.Notebook(self.center_fr, style='Excel.TNotebook')
        self.tabs.pack(fill='both', expand=True)
        self.grid_fr = tk.Frame(self.tabs, bg=PAL['bg'])
        self.tabs.add(self.grid_fr, text=' QUANTUM GRID ')
        self._build_grid(self.grid_fr)
        self.cons_fr = tk.Frame(self.tabs, bg=PAL['panel'])
        self.tabs.add(self.cons_fr, text=' AI LOGS ')
        self.log = scrolledtext.ScrolledText(self.cons_fr, bg=PAL['panel'], fg=PAL['success'], font=('JetBrains Mono', 10), borderwidth=0, padx=20, pady=20)
        self.log.pack(fill='both', expand=True)
        self.status = tk.Label(self, text='SOVEREIGN EXCEL [V3.0] | LEDGER: SYNCHRONIZED | RENDERING: GPU_OPTIMIZED', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')