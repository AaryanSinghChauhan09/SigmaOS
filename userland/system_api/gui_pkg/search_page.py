import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class SearchPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "AERYN SEMANTIC SEARCH", "Local-First Vector Intelligence Retrieval")
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # 1. Search Bar
        search_card = self.gui._card(body, "🔍 Universal Intelligence Query")
        search_card.master.pack(fill="x", pady=(0, 10))
        
        row = tk.Frame(search_card, bg=PAL["card"])
        row.pack(fill="x", pady=10)
        
        self.q_ent = ttk.Entry(row, font=FONT_MED)
        self.q_ent.pack(side="left", fill="x", expand=True, padx=(0, 10))
        self.q_ent.insert(0, "What is the core philosophy of SigmaOS?")
        self.q_ent.bind("<Return>", lambda e: self._do_search())
        
        ttk.Button(row, text="SEMANTIC SEARCH", command=self._do_search).pack(side="right")

        # 2. Results
        self.res_fr = self.gui._card(body, "📄 Sourced Sovereign Intelligence")
        self.res_fr.master.pack(fill="both", expand=True)
        
        self.res_scroll = tk.Frame(self.res_fr, bg=PAL["card"])
        self.res_scroll.pack(fill="both", expand=True)

        # 3. Stats & indexing
        ctrl_fr = tk.Frame(body, bg=PAL["bg"])
        ctrl_fr.pack(fill="x", pady=10)
        
        self.stats_lbl = tk.Label(ctrl_fr, text="Indexed Documents: 142 | Latency: 45ms", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["bg"])
        self.stats_lbl.pack(side="left")
        
        ttk.Button(ctrl_fr, text="Force Re-indexing", command=self._reindex).pack(side="right")

    def _do_search(self):
        query = self.q_ent.get()
        if not query: return
        
        # Clear old results
        for w in self.res_scroll.winfo_children(): w.destroy()
        
        results = self.kernel.aeryn_search.semantic_query(query)
        for res in results:
            item = tk.Frame(self.res_scroll, bg=PAL["bg3"], pady=8, padx=12)
            item.pack(fill="x", pady=2)
            
            header = tk.Frame(item, bg=PAL["bg3"])
            header.pack(fill="x")
            tk.Label(header, text=res["path"], font=FONT_BOLD, fg=PAL["cyan"], bg=PAL["bg3"]).pack(side="left")
            tk.Label(header, text=f"{res['relevance']*100:.1f}% Relevance", font=FONT_SMALL, fg=PAL["teal"], bg=PAL["bg3"]).pack(side="right")
            
            tk.Label(item, text=res["snippet"], font=FONT_SMALL, fg=PAL["dim"], bg=PAL["bg3"], wraplength=800, justify="left").pack(anchor="w", pady=(4,0))
        
        self.gui._notify("Search Complete", f"Found {len(results)} relevant nodes.", "OK")

    def _reindex(self):
        msg = self.kernel.aeryn_search.reindex_system()
        self.gui._notify("Re-indexing", msg, "OK")
        self.stats_lbl.config(text=self.kernel.aeryn_search.health_check())
