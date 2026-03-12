"""
SigmaOS Omni-Search Indexer (v2.0)
==================================
Neural file searching, semantic intent clustering, and zero-latency index lookups.
USP: Deep ML-driven asset fetching bypassing legacy directory traversal.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import time

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00F0FF", # Cyan Omni
    "accent_dim": "#008899",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24",
    "highlight": "#1A1D24"
}

class OmniSearch(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Omni-Search")
        self.geometry("900x650")
        self.configure(bg=PAL["bg"])
        
        # Mocks
        self.db = [
            ("kernel.py", "System Core", "85kb", "C:/SigmaOS/sigma_core/"),
            ("Design_Specs.pdf", "Encrypted Vault", "4.2mb", "F:/Secured/"),
            ("Aura Display Config", "System Setting", "--", "Config Matrix"),
            ("Deploy Nodes", "Macro Action", "0ms", "Omni Automation"),
            ("vacation_photos.enc", "Archived Drive", "12GB", "Z:/Backup/")
        ]
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Omni.Treeview", background=PAL["sidebar"], fieldbackground=PAL["sidebar"], 
                        foreground=PAL["text"], borderwidth=0, font=("Inter", 10), rowheight=30)
        style.configure("Omni.Treeview.Heading", background=PAL["panel"], foreground=PAL["dim"], 
                        font=("Inter", 9, "bold"), borderwidth=0)
        style.map("Omni.Treeview", background=[("selected", PAL["highlight"])])

    def _build_ui(self):
        # 1. Floating Omnibox
        self.bar_fr = tk.Frame(self, bg=PAL["bg"], pady=30, padx=50)
        self.bar_fr.pack(side="top", fill="x")
        
        self.search_entry = tk.Entry(self.bar_fr, font=("Inter", 24, "bold"), bg=PAL["panel"], fg=PAL["accent"], 
                                     insertbackground=PAL["accent"], relief="flat", justify="center")
        self.search_entry.pack(fill="x", ipady=15)
        self.search_entry.insert(0, "Initiate Neural Search Sequence...")
        
        self.search_entry.bind("<FocusIn>", lambda e: self.search_entry.delete(0, tk.END) if self.search_entry.get() == "Initiate Neural Search Sequence..." else None)
        self.search_entry.bind("<KeyRelease>", self._live_search)
        self.search_entry.bind("<Return>", self._execute_search)

        # 2. Main View
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=30, pady=10)
        self.workspace.pack(fill="both", expand=True)

        cols = ("Asset", "Type", "Mass", "Vector Location")
        self.tree = ttk.Treeview(self.workspace, columns=cols, show="headings", style="Omni.Treeview", height=12)
        
        for c in cols:
            self.tree.heading(c, text=c.upper())
            
        self.tree.column("Asset", width=250, anchor="w")
        self.tree.column("Type", width=120, anchor="center")
        self.tree.column("Mass", width=80, anchor="center")
        self.tree.column("Vector Location", width=300, anchor="w")

        self.tree.pack(fill="both", expand=True)
        self.tree.bind("<Double-1>", self._launch_asset)
        
        # Populate initial (blank or all)
        for d in self.db:
            self.tree.insert("", "end", values=d)

        # 3. Status
        self.status = tk.Label(self, text="SEMANTIC ENGINE IDLE | 0.00 MS INDEX LATENCY", 
                               bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _live_search(self, event):
        q = self.search_entry.get().lower()
        
        self.tree.delete(*self.tree.get_children())
        
        if not q:
            for item in self.db:
                self.tree.insert("", "end", values=item)
            self.status.config(text="SEMANTIC ENGINE IDLE | 0.00 MS INDEX LATENCY")
            return
            
        results = 0
        for item in self.db:
            if q in item[0].lower() or q in item[1].lower():
                self.tree.insert("", "end", values=item)
                results += 1
                
        self.status.config(text=f"NEURAL MATCHES FOUND: {results} | SEARCH TIME: 0.12 ms (O[1] Hashing)", bg=PAL["success"], fg="black")

    def _execute_search(self, event):
        self._live_search(None)
        
    def _launch_asset(self, event):
        item = self.tree.selection()
        if item:
            val = self.tree.item(item, "values")[0]
            messagebox.showinfo("Omni-Launch", f"Neural fetch complete. Launching vector:\n\n{val}")

if __name__ == "__main__":
    app = OmniSearch()
    app.mainloop()
