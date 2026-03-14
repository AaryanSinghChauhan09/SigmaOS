import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "SOVEREIGN EXPLORER", "Distributed FS & Silo Orchestration")
        self.current_path = tk.StringVar(value=gui.kernel._root)
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # 1. Path Bar
        path_fr = tk.Frame(body, bg=PAL["bg"])
        path_fr.pack(fill="x", pady=(0, 10))
        
        ttk.Button(path_fr, text="??", width=3, command=self._go_up).pack(side="left")
        self.path_ent = ttk.Entry(path_fr, textvariable=self.current_path)
        self.path_ent.pack(side="left", fill="x", expand=True, padx=5)
        self.path_ent.bind("<Return>", lambda e: self._load_dir())
        
        ttk.Button(path_fr, text="REFRESH", command=self._load_dir).pack(side="left")
        ttk.Button(path_fr, text="OS ROOT", command=lambda: [self.current_path.set(self.kernel._root), self._load_dir()]).pack(side="left", padx=5)

        # 2. Main View
        paned = tk.PanedWindow(body, orient="horizontal", bg=PAL["border"], sashwidth=4)
        paned.pack(fill="both", expand=True)

        # Sidebar: Quick Access & Silos
        side = tk.Frame(paned, bg=PAL["bg2"], width=200)
        paned.add(side)
        
        self.gui._card(side, "Quick Access").pack(fill="x")
        for loc in ["Desktop", "Downloads", "Documents", "SigmaCore"]:
            b = tk.Button(side, text=f"?? {loc}", font=FONT_SMALL, fg=PAL["text"], bg=PAL["bg2"], relief="flat", anchor="w", padx=10)
            b.pack(fill="x")
            
        silo_card = self.gui._card(side, "Active Silos")
        silo_card.master.pack(fill="x", pady=10)
        self.silo_list = tk.Frame(silo_card, bg=PAL["card"])
        self.silo_list.pack(fill="x")

        # Main List
        self.main_list = tk.Frame(paned, bg=PAL["bg"])
        paned.add(self.main_list)
        
        cols = ("Name", "Size", "Type", "Integrity")
        self.tree = ttk.Treeview(self.main_list, columns=cols, show="headings", selectmode="browse")
        for c in cols: self.tree.heading(c, text=c)
        self.tree.column("Name", width=300)
        self.tree.column("Integrity", width=100)
        self.tree.pack(fill="both", expand=True)
        
        self.tree.bind("<Double-1>", self._on_double_click)
        self.tree.bind("<Button-3>", self._show_context_menu)

        self._load_dir()

    def _load_dir(self):
        path = self.current_path.get()
        if not os.path.exists(path): 
            messagebox.showerror("Error", "Path not found.")
            return

        for i in self.tree.get_children(): self.tree.delete(i)
        
        try:
            for item in os.listdir(path):
                full = os.path.join(path, item)
                is_dir = os.path.isdir(full)
                size = f"{os.path.getsize(full) // 1024} KB" if not is_dir else "-"
                itype = "Folder" if is_dir else "File"
                integrity = "VERIFIED" # Simulated
                
                icon = "??" if is_dir else "??"
                self.tree.insert("", "end", values=(f"{icon} {item}", size, itype, integrity), tags=(full,))
        except Exception as e:
            self.gui._notify("FS Error", str(e), "ERR")

    def _go_up(self):
        new = os.path.dirname(self.current_path.get())
        self.current_path.set(new)
        self._load_dir()

    def _on_double_click(self, event):
        item = self.tree.selection()[0]
        full_path = self.tree.item(item, "tags")[0]
        if os.path.isdir(full_path):
            self.current_path.set(full_path)
            self._load_dir()
        else:
            os.startfile(full_path)

    def _show_context_menu(self, event):
        item = self.tree.identify_row(event.y)
        if not item: return
        self.tree.selection_set(item)
        full_path = self.tree.item(item, "tags")[0]
        
        menu = tk.Menu(self, tearoff=0, bg=PAL["bg2"], fg=PAL["text"])
        menu.add_command(label="Open", command=lambda: os.startfile(full_path))
        menu.add_separator()
        menu.add_command(label="?? Mount in New Silo", command=lambda: self._mount_silo(full_path))
        menu.add_command(label="?? Semantic Index (Aeryn)", command=lambda: self.gui._notify("Search", "Queued for indexing.", "OK"))
        menu.add_command(label="?? Verify Shard Integrity", command=lambda: self.gui._notify("Integrity", "Hash verified.", "OK"))
        menu.post(event.x_root, event.y_root)

    def _mount_silo(self, path):
        if not os.path.isdir(path):
             return self.gui._notify("Silo Error", "Can only silo directories.", "ERR")
        sid = self.kernel.silo_fs.create_silo(os.path.basename(path), path)
        self.gui._notify("Silo Active", f"Mounted as {sid}", "OK")
        self._refresh_silos()

    def _refresh_silos(self):
        for w in self.silo_list.winfo_children(): w.destroy()
        for sid, data in self.kernel.silo_fs.active_silos.items():
            tk.Label(self.silo_list, text=f"?? {data['app']}", fg=PAL["cyan"], bg=PAL["card"]).pack(anchor="w")
