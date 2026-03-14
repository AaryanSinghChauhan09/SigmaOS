import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_MED, FONT_SMALL

class BrowserPage(SigmaPage):
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "Sovereign Browser Pro", "Absorption of Chrome/Arc/Safari — Zero-Trust Rendering")
        self.browser = self.kernel.registry.get("browser")
        self._build_ui()

    def _build_ui(self):
        # Pro Address Bar with Identity Shield
        nav = tk.Frame(self, bg=PAL["bg2"], height=50)
        nav.pack(fill="x")
        nav.pack_propagate(False)
        
        self.status_lbl = tk.Label(nav, text="🛡️", font=FONT_MED, fg=PAL["green"], bg=PAL["bg2"])
        self.status_lbl.pack(side="left", padx=10)
        
        self.url_e = tk.Entry(nav, bg=PAL["bg3"], fg="white", font=FONT_MED, bd=0, insertbackground="white")
        self.url_e.pack(side="left", fill="x", expand=True, padx=5, pady=10)
        
        initial_url = self.browser.tabs[0]["url"] if self.browser and self.browser.tabs else "https://sigma.search"
        self.url_e.insert(0, initial_url)
        
        # Web Canvas
        view = tk.Frame(self, bg="white")
        view.pack(fill="both", expand=True)
        self.content_lbl = tk.Label(view, text="SOVEREIGN SEARCH", font=("Inter Bold", 24), fg=PAL["bg"], bg="white", wraplength=800)
        self.content_lbl.pack(pady=50)

        def _go(e=None):
            url = self.url_e.get()
            if self.browser:
                self.browser.navigate(self.browser.tabs[0]["id"], url)
                self.status_lbl.config(text="🛰️", fg=PAL["teal"])
                self.after(500, lambda: self.content_lbl.config(text=self.browser.tabs[0]["content"]))
                self.after(1000, lambda: self.status_lbl.config(text="🛡️", fg=PAL["green"]))

        self.url_e.bind("<Return>", _go)
        ttk.Button(nav, text="GO", command=_go).pack(side="right", padx=10)
        
        tk.Label(nav, text="⚡ AI Lens", font=FONT_SMALL, fg=PAL["accent"], bg=PAL["bg2"]).pack(side="right", padx=5)
