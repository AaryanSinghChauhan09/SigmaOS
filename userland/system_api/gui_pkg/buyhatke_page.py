import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "SigmaBuyHatke", "Sovereign Market Intelligence")
        self.build()

    def build(self):
        tab_bar = tk.Frame(self, bg=PAL["bg2"], height=40)
        tab_bar.pack(fill="x", pady=(0,10))
        tab_bar.pack_propagate(False)
        
        tabs = [
            ("Tracker", "📉"), ("Forecast","🔮"), ("Logistics","🚚"),
            ("Coupons", "🎟️"), ("Compare", "⚖️"), ("CRM", "💼")
        ]
        
        self.container = tk.Frame(self, bg=PAL["bg"])
        self.container.pack(fill="both", expand=True)
        self.sub_pages = {}
        
        for name, icon in tabs:
            tk.Button(tab_bar, text=f"{icon} {name}", font=FONT_SMALL, fg=PAL["text"],
                      bg=PAL["bg2"], bd=0, activebackground=PAL["bg"], 
                      command=lambda n=name.lower(): self._show_sub(n)).pack(side="left", padx=10, fill="y")

        self._show_sub("tracker")

    def _show_sub(self, name):
        for s in self.sub_pages.values(): s.pack_forget()
        if name not in self.sub_pages:
            p = tk.Frame(self.container, bg=PAL["bg"])
            self.sub_pages[name] = p
            getattr(self, f"_build_{name}")(p)
        self.sub_pages[name].pack(fill="both", expand=True)

    def _build_tracker(self, parent):
        l_fr = tk.Frame(parent, bg=PAL["bg2"], width=300)
        l_fr.pack(side="left", fill="both", padx=5)
        tk.Label(l_fr, text="Product Price Intel", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg2"]).pack(pady=10)
        ent = ttk.Entry(l_fr); ent.pack(fill="x", padx=10); ent.insert(0, "iPhone 15")
        ttk.Button(l_fr, text="Analyze Trend").pack(pady=10)

    def _build_forecast(self, parent):
        tk.Label(parent, text="Quantum Price Forecasting", font=FONT_MED, fg=PAL["cyan"], bg=PAL["bg"]).pack(pady=10)
        log = self.gui._console(parent, height=15)
        log.pack(fill="both", expand=True, padx=20, pady=10)

    def _build_logistics(self, parent):
        tk.Label(parent, text="EDI Shipment Tracking", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg"]).pack(pady=10)

    def _build_coupons(self, parent):
        tk.Label(parent, text="Auto-Coupon Discovery", font=FONT_MED, fg=PAL["teal"], bg=PAL["bg"]).pack(pady=10)

    def _build_compare(self, parent):
        tk.Label(parent, text="Market Comparison Engine", font=FONT_MED, fg=PAL["gold"], bg=PAL["bg"]).pack(pady=10)

    def _build_crm(self, parent):
        tk.Label(parent, text="Lead Pipeline Management", font=FONT_MED, fg=PAL["cyan"], bg=PAL["bg"]).pack(pady=10)
