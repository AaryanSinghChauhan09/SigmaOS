# Generated method: SearchPage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class SearchPage:
    def build(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        search_card = self.gui._card(body, '🔍 Universal Intelligence Query')
        search_card.master.pack(fill='x', pady=(0, 10))
        row = tk.Frame(search_card, bg=PAL['card'])
        row.pack(fill='x', pady=10)
        self.q_ent = ttk.Entry(row, font=FONT_MED)
        self.q_ent.pack(side='left', fill='x', expand=True, padx=(0, 10))
        self.q_ent.insert(0, 'What is the core philosophy of SigmaOS?')
        self.q_ent.bind('<Return>', lambda e: self._do_search())
        ttk.Button(row, text='SEMANTIC SEARCH', command=self._do_search).pack(side='right')
        self.res_fr = self.gui._card(body, '📄 Sourced Sovereign Intelligence')
        self.res_fr.master.pack(fill='both', expand=True)
        self.res_scroll = tk.Frame(self.res_fr, bg=PAL['card'])
        self.res_scroll.pack(fill='both', expand=True)
        ctrl_fr = tk.Frame(body, bg=PAL['bg'])
        ctrl_fr.pack(fill='x', pady=10)
        self.stats_lbl = tk.Label(ctrl_fr, text='Indexed Documents: 142 | Latency: 45ms', font=FONT_SMALL, fg=PAL['dim'], bg=PAL['bg'])
        self.stats_lbl.pack(side='left')
        ttk.Button(ctrl_fr, text='Force Re-indexing', command=self._reindex).pack(side='right')