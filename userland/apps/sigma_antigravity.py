"""
SigmaOS × Antigravity AI Hub (v3.0) — NATIVE OS INTEGRATION
============================================================
Absorbs & extends the full Antigravity AI Orchestrator ecosystem into SigmaOS.
USP: Pure logical dashboard using only standard library and Antigravity Shards.
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import threading, webbrowser, urllib.parse, json, os, time, sys
from typing import Dict, Any, List, Optional

# Absolute path injection for zero-friction module discovery
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

try:
    from sigma_core.ai.antigravity_manifest import PLATFORMS, QUOTA_DEFAULTS # type: ignore
    from sigma_core.ai.antigravity_engine import AntigravityEngine # type: ignore
    from sigma_core.ui.fluid_design import PALETTE as PAL # type: ignore
except ImportError:
    PLATFORMS = [{"name": "ChatGPT", "url": "https://chatgpt.com", "color": "#10A37F", "tier": 1, "icon": "🤖"}]
    QUOTA_DEFAULTS = {"ChatGPT": {"limit": 40, "used": 0, "unit": "msgs", "pro": True}}
    class AntigravityEngine:
        def __init__(self, kernel=None):
            self.platforms = PLATFORMS
            self.quotas = QUOTA_DEFAULTS
            self.history = []
        def dispatch_prompt(self, p, s): return {"status": "MOCK", "time": "00:00", "prompt": p, "platforms": s}
    PAL = {"bg": "#07070A", "panel": "#0F1018", "card": "#13141C", "header": "#0C0D15", "accent": "#3D9EFF", "text": "#E5E5EA", "dim": "#8E8E93", "border": "#2C2C3C", "green": "#32D74B", "red": "#FF453A", "orange": "#FF9F0A", "gold": "#FFD60A"}

class SigmaAntigravity(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.engine = AntigravityEngine(kernel)
        self.title("SigmaOS × Antigravity AI Hub v3.0 Apex")
        self.geometry("1400x900")
        self.configure(bg=PAL["bg"])
        
        # UI Proxies for static analysis
        self.nb: ttk.Notebook = None # type: ignore
        self.prompt_txt: tk.Text = None # type: ignore
        self.dispatch_btn: tk.Button = None # type: ignore
        self.log: scrolledtext.ScrolledText = None # type: ignore
        self.quota_fr: tk.Frame = None # type: ignore
        self.hist_tree: ttk.Treeview = None # type: ignore
        
        self._sel_platforms: dict[str, tk.BooleanVar] = {}
        self._server_online = False
        
        self._setup_styles()
        self._build_ui()
        self._poll_status()

    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use("clam")
        s.configure("Treeview", background=PAL["card"], foreground=PAL["text"], fieldbackground=PAL["card"])
        s.configure("TNotebook", background=PAL["bg"])
        s.configure("TNotebook.Tab", background=PAL["panel"], foreground=PAL["dim"], padding=[15, 5])
        s.map("TNotebook.Tab", background=[("selected", PAL["card"])], foreground=[("selected", "white")])

    def _build_ui(self):
        head = tk.Frame(self, bg=PAL["header"], height=60)
        head.pack(fill="x")
        tk.Label(head, text="⚡ ANTIGRAVITY", font=("Inter Bold", 16), fg=PAL["accent"], bg=PAL["header"]).pack(side="left", padx=20)
        
        self.nb = ttk.Notebook(self)
        self.nb.pack(fill="both", expand=True)
        
        # Tab 1: Distributor
        dist_fr = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(dist_fr, text="  🚀 Distributor  ")
        body = tk.Frame(dist_fr, bg=PAL["bg"], padx=20, pady=20)
        body.pack(fill="both", expand=True)
        
        left = tk.Frame(body, bg=PAL["panel"], width=250)
        left.pack(side="left", fill="y", padx=(0, 20))
        left.pack_propagate(False)
        
        tk.Label(left, text="AI FLEET", font=("Inter", 9, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(pady=10)
        for plat in self.engine.platforms:
            var = tk.BooleanVar(value=plat.get("tier", 2) == 1)
            self._sel_platforms[plat["name"]] = var
            row = tk.Frame(left, bg=PAL["panel"])
            row.pack(fill="x", padx=10)
            tk.Checkbutton(row, variable=var, bg=PAL["panel"]).pack(side="left")
            tk.Label(row, text=f"{plat.get('icon', '🤖')} {plat['name']}", fg=plat.get("color", "white"), bg=PAL["panel"]).pack(side="left")

        right = tk.Frame(body, bg=PAL["bg"])
        right.pack(side="left", fill="both", expand=True)
        self.prompt_txt = tk.Text(right, bg=PAL["card"], fg=PAL["text"], font=("Inter", 11), height=10, padx=15, pady=15)
        self.prompt_txt.pack(fill="x", pady=10)
        
        self.dispatch_btn = tk.Button(right, text="⚡ DISPATCH TO FLEET", bg=PAL["accent"], fg="white", font=("Inter Bold", 12), command=self._dispatch)
        self.dispatch_btn.pack(fill="x")
        
        self.log = scrolledtext.ScrolledText(right, bg="#050508", fg=PAL["green"], font=("Consolas", 9), height=12)
        self.log.pack(fill="both", expand=True, pady=(20, 0))

        # Tab 2: Quotas
        self.quota_fr = tk.Frame(self.nb, bg=PAL["bg"], padx=20, pady=20)
        self.nb.add(self.quota_fr, text="  📊 Quotas  ")
        self._refresh_quota_ui()

        # Tab 3: History
        hist_fr = tk.Frame(self.nb, bg=PAL["bg"], padx=20, pady=20)
        self.nb.add(hist_fr, text="  📋 History  ")
        cols = ("Time", "Platforms", "Prompt Preview")
        self.hist_tree = ttk.Treeview(hist_fr, columns=cols, show="headings")
        for col in cols: self.hist_tree.heading(col, text=col)
        self.hist_tree.pack(fill="both", expand=True)

    def _refresh_quota_ui(self):
        for w in self.quota_fr.winfo_children(): w.destroy()
        for name, data in self.engine.quotas.items():
            card = tk.Frame(self.quota_fr, bg=PAL["card"], padx=15, pady=10, highlightthickness=1, highlightbackground=PAL["border"])
            card.pack(fill="x", pady=5)
            tk.Label(card, text=name, font=("Inter Bold", 10), fg=PAL["accent"], bg=PAL["card"]).pack(side="left")
            tk.Label(card, text=f"{data['used']} / {data['limit']} {data['unit']}", font=("Inter", 9), fg=PAL["text"], bg=PAL["card"]).pack(side="right")

    def _dispatch(self):
        prompt = self.prompt_txt.get("1.0", "end").strip()
        if not prompt: return
        selected = [name for name, var in self._sel_platforms.items() if var.get()]
        if not selected: return
        res = self.engine.dispatch_prompt(prompt, selected)
        self.log.insert("end", f"[{res['time']}] DISPATCHED: {len(selected)} nodes active.\n")
        self._update_history()

    def _update_history(self):
        self.hist_tree.delete(*self.hist_tree.get_children())
        for rec in reversed(self.engine.history):
            self.hist_tree.insert("", "end", values=(rec["time"], ", ".join(rec["platforms"]), rec["prompt"][:100]))

    def _poll_status(self):
        self._update_history()
        self.after(5000, self._poll_status)

if __name__ == "__main__":
    SigmaAntigravity().mainloop()
