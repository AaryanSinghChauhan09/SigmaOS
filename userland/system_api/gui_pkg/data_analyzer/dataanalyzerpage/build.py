# Generated method: DataAnalyzerPage.build
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import csv
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class DataAnalyzerPage:
    def build(self):
        self.controller._build_page_header(self, 'DATA ANALYZER', 'Zero-Latency CSV/Data Forensics')
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        tools_fr = tk.Frame(body, bg=PAL['bg'])
        tools_fr.pack(fill='x', pady=5)
        self.file_lbl = tk.Label(tools_fr, text='No file loaded.', font=FONT_MED, bg=PAL['bg'], fg=PAL['dim'])
        self.file_lbl.pack(side='left', padx=10)

        def _load_csv():
            path = filedialog.askopenfilename(title='Select CSV', filetypes=[('CSV Files', '*.csv'), ('All', '*.*')])
            if path:
                self.file_lbl.config(text=f"Loaded: {path.split('/')[-1]}")
                self._parse_csv(path)
        ttk.Button(tools_fr, text='📂 Load Dataset', command=_load_csv).pack(side='right', padx=5)
        self.stats_fr = tk.Frame(body, bg=PAL['bg'])
        self.stats_fr.pack(fill='x', pady=5)
        self.stat_rows = tk.Label(self.stats_fr, text='Rows: 0', font=FONT_BOLD, fg=PAL['cyan'], bg=PAL['bg'])
        self.stat_rows.pack(side='left', padx=10)
        self.stat_cols = tk.Label(self.stats_fr, text='Cols: 0', font=FONT_BOLD, fg=PAL['teal'], bg=PAL['bg'])
        self.stat_cols.pack(side='left', padx=10)
        table_fr = self.controller._card(body, 'Data Preview')
        table_fr.master.pack(fill='both', expand=True, pady=10)
        self.tree = ttk.Treeview(table_fr, show='headings', height=15)
        y_scroll = ttk.Scrollbar(table_fr, orient='vertical', command=self.tree.yview)
        x_scroll = ttk.Scrollbar(table_fr, orient='horizontal', command=self.tree.xview)
        self.tree.configure(yscrollcommand=y_scroll.set, xscrollcommand=x_scroll.set)
        self.tree.pack(side='left', fill='both', expand=True)
        y_scroll.pack(side='right', fill='y')
        x_scroll.pack(side='bottom', fill='x')