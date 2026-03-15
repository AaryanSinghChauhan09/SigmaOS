# Generated method: SearchPage._do_search
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class SearchPage:
    def _do_search(self):
        query = self.q_ent.get()
        if not query:
            return
        for w in self.res_scroll.winfo_children():
            w.destroy()
        results = self.kernel.aeryn_search.semantic_query(query)
        for res in results:
            item = tk.Frame(self.res_scroll, bg=PAL['bg3'], pady=8, padx=12)
            item.pack(fill='x', pady=2)
            header = tk.Frame(item, bg=PAL['bg3'])
            header.pack(fill='x')
            tk.Label(header, text=res['path'], font=FONT_BOLD, fg=PAL['cyan'], bg=PAL['bg3']).pack(side='left')
            tk.Label(header, text=f"{res['relevance'] * 100:.1f}% Relevance", font=FONT_SMALL, fg=PAL['teal'], bg=PAL['bg3']).pack(side='right')
            tk.Label(item, text=res['snippet'], font=FONT_SMALL, fg=PAL['dim'], bg=PAL['bg3'], wraplength=800, justify='left').pack(anchor='w', pady=(4, 0))
        self.gui._notify('Search Complete', f'Found {len(results)} relevant nodes.', 'OK')