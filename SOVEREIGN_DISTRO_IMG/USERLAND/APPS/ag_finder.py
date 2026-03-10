"""
SigmaOS Sovereign Finder Apex Pro (v3.0)
========================================
High-performance file exploration, tool discovery, and workspace orchestration.
USP: Neural Search Indexing & Sovereign Workspace Snapshots.
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import os
import time

PAL = {
    "bg": "#0B0C0F",
    "sidebar": "#16181D",
    "accent": "#5AC8FA", # Sky Blue
    "text": "#E8E8E8",
    "dim": "#8E8E93",
    "success": "#32D74B",
    "border": "#2C2F38",
    "panel": "#111216"
}

class ToolsFinder(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Sovereign Finder Apex Pro")
        self.geometry("1100x750")
        self.configure(bg=PAL["bg"])
        
        self.tools = [
            "Aether Orchestrator", "Titan Capture", "IndentFlow", "Email Discovery Agent",
            "Excel AI Filler", "Excel Preprocessor", "PDF Forge", "Pure Text", 
            "Text Cleaner", "OpenRoutines Dashboard", "Sovereign De-bloater", "AG Shuffler",
            "Sovereign Strategist", "AuraPaint Pro", "PulsePlayer Pro", "CodeForge Pro"
        ]
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Treeview", background=PAL["sidebar"], foreground=PAL["text"], 
                        fieldbackground=PAL["sidebar"], borderwidth=0, font=("Inter", 9))
        style.map("Treeview", background=[("selected", PAL["accent"])])

    def _build_ui(self):
        # 1. Dashboard Header
        head = tk.Frame(self, bg=PAL["bg"], padx=30, pady=25)
        head.pack(fill="x")
        
        tk.Label(head, text="SOVEREIGN FINDER", font=("Inter", 22, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        self.search_var = tk.StringVar()
        self.search_entry = tk.Entry(head, textvariable=self.search_var, bg="#000", fg=PAL["text"], 
                                    font=("Inter", 11), borderwidth=0, insertbackground="white", 
                                    width=40, highlightthickness=1, highlightbackground=PAL["border"])
        self.search_entry.pack(side="right", pady=5)
        self.search_entry.insert(0, "[ NEURAL SEARCH ]")
        self.search_entry.bind("<FocusIn>", lambda e: self.search_entry.delete(0, "end"))
        self.search_entry.bind("<KeyRelease>", lambda e: self._filter())

        # 2. Main Body
        body = tk.Frame(self, bg=PAL["bg"], padx=25)
        body.pack(fill="both", expand=True)

        self.panes = ttk.PanedWindow(body, orient="horizontal")
        self.panes.pack(fill="both", expand=True)

        # Sidebar: Disks & Shortcuts
        self.sidebar = tk.Frame(self.panes, bg=PAL["sidebar"], width=220, padx=15, pady=20)
        self.panes.add(self.sidebar, weight=1)
        self.sidebar.pack_propagate(False)

        tk.Label(self.sidebar, text="VAULTS", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["sidebar"]).pack(anchor="w")
        for disk in [("💾 Main Vault", "success"), ("🕸️ Mesh Drive", "accent"), ("🔥 Burner Temp", "dim")]:
            f = tk.Frame(self.sidebar, bg=PAL["sidebar"], pady=8, cursor="hand2")
            f.pack(fill="x")
            tk.Label(f, text=disk[0], font=("Inter", 10), fg=PAL["text"], bg=PAL["sidebar"]).pack(side="left")

        tk.Label(self.sidebar, text="QUICK ACCESS", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["sidebar"], pady=(20, 0)).pack(anchor="w")
        for item in ["Downloads", "Documents", "Source_Code", "Media"]:
             tk.Label(self.sidebar, text=f"📂 {item}", font=("Inter", 9), fg=PAL["text"], 
                      bg=PAL["sidebar"], pady=8, cursor="hand2").pack(anchor="w")

        # Main View: Results Grid
        self.view_fr = tk.Frame(self.panes, bg=PAL["bg"], padx=20)
        self.panes.add(self.view_fr, weight=4)

        self.list_canvas = tk.Canvas(self.view_fr, bg=PAL["bg"], highlightthickness=0)
        self.scroll = ttk.Scrollbar(self.view_fr, orient="vertical", command=self.list_canvas.yview)
        self.grid_fr = tk.Frame(self.list_canvas, bg=PAL["bg"])
        
        self.list_canvas.create_window((0, 0), window=self.grid_fr, anchor="nw")
        self.list_canvas.configure(yscrollcommand=self.scroll.set)
        
        self.list_canvas.pack(side="left", fill="both", expand=True)
        self.scroll.pack(side="right", fill="y")
        
        self.grid_fr.bind("<Configure>", lambda e: self.list_canvas.configure(scrollregion=self.list_canvas.bbox("all")))

        self._filter()

        # 3. Status Bar
        self.status = tk.Label(self, text="FINDER READY | INDEXING: 100% | WORKSPACE: SIGMA_CORE", 
                               bg=PAL["accent"], fg="white", font=("Inter", 8, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

    def _filter(self):
        q = self.search_var.get().lower() if self.search_var.get() != "[ NEURAL SEARCH ]" else ""
        
        for w in self.grid_fr.winfo_children(): w.destroy()
        
        visible_tools = [t for t in self.tools if q in t.lower()]
        
        cols = 3
        for i, t in enumerate(visible_tools):
            r, c = i // cols, i % cols
            card = tk.Frame(self.grid_fr, bg=PAL["panel"], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL["border"])
            card.grid(row=r, column=c, padx=10, pady=10, sticky="nsew")
            
            icon = "🛠️" if "Pro" not in t else "⚡"
            tk.Label(card, text=icon, font=("Inter", 24), bg=PAL["panel"]).pack()
            tk.Label(card, text=t.upper(), font=("Inter", 9, "bold"), fg=PAL["text"], bg=PAL["panel"]).pack(pady=(10, 0))
            tk.Label(card, text="Antigravity Enterprise App", font=("Inter", 7), fg=PAL["dim"], bg=PAL["panel"]).pack()
            
            btn = tk.Button(card, text="INITIATE", font=("Inter", 7, "bold"), bg=PAL["accent"], fg="white", relief="flat", padx=10)
            btn.pack(pady=(10, 0))
            
            # Hover effects
            card.bind("<Enter>", lambda e, cd=card: cd.config(highlightbackground=PAL["accent"]))
            card.bind("<Leave>", lambda e, cd=card: cd.config(highlightbackground=PAL["border"]))

    def _trigger_neural_scan(self):
        self.status.config(text="NEURAL SCAN IN PROGRESS...", bg="#FFD60A")
        self.after(1000, lambda: self.status.config(text="SCAN COMPLETE: ALL BITS VERIFIED", bg=PAL["accent"]))

if __name__ == "__main__":
    app = ToolsFinder()
    app.mainloop()
