import tkinter as tk
from tkinter import ttk
import random
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ShoppingWizardPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Shopping Wizard", "Omnichannel Price Tracking & Coupon Parity Engine")
        self._tracked_items = [
            {"name": "RTX 5090 FE", "price": "$1599", "target": "$1499", "status": "Steady", "store": "BestBuy"},
            {"name": "MacBook Pro M4 Max", "price": "$3499", "target": "$3100", "status": "Dropping", "store": "Amazon"},
            {"name": "Herman Miller Embody", "price": "$1895", "target": "$1500", "status": "Steady", "store": "DesignWithinReach"},
        ]
        self._build_ui()

    def _build_ui(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # Search & Add Bar
        search_fr = self._card(body, "Track New Product")
        search_fr.master.pack(fill="x", pady=(0, 10))
        
        entry_fr = tk.Frame(search_fr, bg=PAL["card"])
        entry_fr.pack(fill="x")
        
        self._prod_url = tk.StringVar(value="https://store.example.com/product/...")
        ent = ttk.Entry(entry_fr, textvariable=self._prod_url, font=("Inter", 10))
        ent.pack(side="left", fill="x", expand=True, padx=(0, 10))
        
        ttk.Button(entry_fr, text="🔍 Analyze & Track", command=self._add_track).pack(side="right")

        # Main Workspace
        panes = tk.Frame(body, bg=PAL["bg"])
        panes.pack(fill="both", expand=True)

        # Left: Tracked Items & Price Comparison
        left_fr = tk.Frame(panes, bg=PAL["bg"])
        left_fr.pack(side="left", fill="both", expand=True, padx=(0, 10))
        
        track_card = self._card(left_fr, "Active Price Monitor")
        track_card.master.pack(fill="both", expand=True)
        
        self.tree = ttk.Treeview(track_card, columns=("price", "target", "store", "status"), show="headings", height=8)
        self.tree.heading("price", text="Current Price")
        self.tree.heading("target", text="Alert Threshold")
        self.tree.heading("store", text="Best Store")
        self.tree.heading("status", text="Trend")
        self.tree.pack(fill="both", expand=True)
        
        for item in self._tracked_items:
            self.tree.insert("", "end", text=item["name"], values=(item["price"], item["target"], item["store"], item["status"]))

        # Right: Coupon Parity & Deals
        right_fr = tk.Frame(panes, bg=PAL["bg"], width=300)
        right_fr.pack(side="right", fill="both")
        right_fr.pack_propagate(False)

        coupon_card = self._card(right_fr, "⚡ Coupon Parity")
        coupon_card.master.pack(fill="both", expand=True)
        
        coupons = [
            ("AMZ_SAVE20", "20% Off Electronics", "VERIFIED"),
            ("BB_GAMER", "$50 Off GPU", "EXPIRED"),
            ("FREESHIP", "Free Shipping", "VERIFIED"),
        ]
        for code, desc, stat in coupons:
            c = tk.Frame(coupon_card, bg=PAL["bg2"], pady=5, padx=5)
            c.pack(fill="x", pady=2)
            tk.Label(c, text=code, font=FONT_BOLD, fg=PAL["cyan"], bg=PAL["bg2"]).pack(anchor="w")
            tk.Label(c, text=desc, font=FONT_SMALL, fg=PAL["text"], bg=PAL["bg2"]).pack(anchor="w")
            col = PAL["green"] if stat=="VERIFIED" else PAL["red"]
            tk.Label(c, text=stat, font=("Inter", 7, "bold"), fg=col, bg=PAL["bg2"]).pack(anchor="e")

        ttk.Button(right_fr, text="🍯 Auto-Apply Best Coupon (Honey)", 
                   command=lambda: self.gui._log_voice("Honey: Testing 42 codes... Found VERIFIED 'AMZ_SAVE20'. Saving $319.")).pack(fill="x", pady=10)

        # Price History Mock (CamelCamelCamel USP)
        hist_card = self._card(right_fr, "📈 Price History (90d)")
        hist_card.master.pack(fill="both", expand=True, pady=(10, 0))
        tk.Label(hist_card, text="LOWER THAN 98% OF YEAR", font=FONT_BOLD, fg=PAL["green"], bg=PAL["card"]).pack(pady=5)
        
        canvas = tk.Canvas(hist_card, height=60, bg=PAL["bg2"], highlightthickness=0)
        canvas.pack(fill="x")
        points = [10, 50, 20, 40, 30, 10, 50, 0]
        for i in range(len(points)-1):
            canvas.create_line(i*30, 50-points[i], (i+1)*30, 50-points[i+1], fill=PAL["cyan"], width=2)

    def _add_track(self):
        url = self._prod_url.get()
        self.gui._log_voice(f"Sovereign Crawler dispatched to: {url}")
        self.after(1000, lambda: self.gui._log_voice("Analysis complete. Found parity at 4 stores. Tracking activated."))
