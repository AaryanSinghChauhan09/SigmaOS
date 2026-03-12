import tkinter as tk
from tkinter import scrolledtext, messagebox
from .styles import PAL, FONT_MONO, FONT_SMALL, FONT_BOLD

class UIMixin:
    """Helper methods for SigmaOS GUI components."""
    
    def _card(self, parent, title="", padx=16, pady=12, glass=False) -> tk.Frame:
        bg_col = PAL["bg2"] if glass else PAL["card"]
        bord = PAL["accent"] if glass else PAL["border"]
        
        outer = tk.Frame(parent, bg=bord, padx=1, pady=1)
        container = tk.Frame(outer, bg=bg_col, padx=padx, pady=pady)
        container.pack(fill="both", expand=True)
        container.master = outer 
        
        if title:
            hdr = tk.Frame(container, bg=bg_col)
            hdr.pack(fill="x", pady=(0, 10))
            tk.Label(hdr, text=title.upper(), font=("Inter Bold", 8),
                     fg=PAL["dim"] if not glass else PAL["cyan"], bg=bg_col).pack(side="left")
            tk.Frame(container, bg=PAL["bg3"], height=1).pack(fill="x", pady=(0, 15))
            
        return container

    def _console(self, parent, height=8) -> scrolledtext.ScrolledText:
        st = scrolledtext.ScrolledText(
            parent, bg="#0A0A14", fg=PAL["green"], insertbackground=PAL["cyan"],
            font=FONT_MONO, height=height, relief="flat",
            selectbackground=PAL["accent"])
        st.tag_configure("OK",    foreground=PAL["green"])
        st.tag_configure("WARN",  foreground=PAL["gold"])
        st.tag_configure("ERR",   foreground=PAL["red"])
        st.tag_configure("INFO",  foreground=PAL["cyan"])
        st.tag_configure("HEAD",  foreground=PAL["accent2"], font=("Consolas",10,"bold"))
        return st

    def _log(self, console: scrolledtext.ScrolledText, text: str, tag="OK"):
        def _inner():
            if not console.winfo_exists(): return
            console.configure(state="normal")
            console.insert("end", text + "\n", tag)
            console.see("end")
            console.configure(state="disabled")
        self.after(0, _inner)

    def _notify(self, title: str, msg: str, level: str = "INFO"):
        """System notification proxy."""
        # This will be overridden or linked back to SigmaGUI._notify
        if hasattr(self, 'master') and hasattr(self.master, '_notify'):
            self.master._notify(title, msg, level)
        else:
            print(f"[{level}] {title}: {msg}")

    def _build_page_header(self, parent, title, subtitle):
        """Standardized Page Header."""
        header = tk.Frame(parent, bg=PAL["bg"], pady=30, padx=20)
        header.pack(fill="x")
        
        tk.Label(header, text=title.upper(), font=("Inter", 24, "bold"), 
                 fg=PAL["text"], bg=PAL["bg"]).pack(anchor="w")
        tk.Label(header, text=subtitle, font=("Inter", 10), 
                 fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w", pady=(5, 0))
        tk.Frame(parent, bg=PAL["border"], height=1).pack(fill="x", padx=20)
