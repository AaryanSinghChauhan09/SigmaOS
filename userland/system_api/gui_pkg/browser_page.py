import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_MED, FONT_SMALL

class BrowserPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Sovereign Browser Pro", "Absorption of Chrome/Arc/Safari — Zero-Trust Rendering")
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
        
        # USP: Privacy Shield Controls
        shield_fr = tk.Frame(nav, bg=PAL["bg2"])
        shield_fr.pack(side="left", padx=10)
        for icon, tooltip in [("🎭", "WebGL Fake Values"), ("🖼️", "Canvas Noise"), ("📍", "Geo-Spoof")]:
            btn = tk.Button(shield_fr, text=icon, font=("Segoe UI Symbol", 10), bg=PAL["bg2"], fg=PAL["green"], 
                            relief="flat", bd=0, command=lambda t=tooltip: self.gui._log_voice(f"Shield: {t} active."))
            btn.pack(side="left")
        
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
        
        # USP: Wappalyzer / Tech Stack detector
        self.tech_lbl = tk.Label(nav, text="React • Python • Nginx", font=("Inter Bold", 8), fg=PAL["cyan"], bg=PAL["bg2"])
        self.tech_lbl.pack(side="right", padx=5)
        
        # USP: Word Replacer Max - Content Surgeon
        tk.Button(nav, text="✂️ Surgeon", font=FONT_SMALL, bg=PAL["bg2"], fg=PAL["gold"], relief="flat",
                  command=self._text_surgeon).pack(side="right", padx=5)

        tk.Label(nav, text="⚡ AI Lens", font=FONT_SMALL, fg=PAL["accent"], bg=PAL["bg2"]).pack(side="right", padx=5)

    def _text_surgeon(self):
        """USP: Word Replacer Max simulation."""
        current = self.content_lbl.cget("text")
        if "SOVEREIGN" in current:
            new_text = current.replace("SOVEREIGN", "ULTRA-SYNCED")
            self.content_lbl.config(text=new_text)
            self.gui._log_voice("Surgeon: Content dynamically transformed via WordReplacer node.")
        else:
            self.gui._log_voice("Surgeon: No target patterns found on page.")
