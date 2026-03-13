"""
SigmaOS NCERT Master Lab v4.1 — Fully Dynamic Virtual Lab
Physics, Chemistry, Biology, Mathematics | Classes 1–12
100% stdlib/tkinter | Automatic simulation engine
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback

# ─── UI DEFAULTS ─────────────────────────────────────────────────────────────
PAL = {
    "bg":"#0B0D17", "panel":"#11142A", "card":"#181B2E", "accent":"#6C63FF",
    "ph":"#3B82F6", "ch":"#22C55E", "bi":"#EC4899", "ma":"#F59E0B",
    "text":"#E8E8F0", "dim":"#8888A0", "border":"#252840",
}

class NCERTMasterLab(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • NCERT Virtual Lab v4.1")
        self.geometry("1400x900")
        self.configure(bg=PAL["bg"])
        
        # Initialize attributes to satisfy linter
        self._mods = {}
        self._exp_map = {}
        self._tree = None
        self._mid = None
        self._out = None
        self._tree_fr = None
        self._out_fr = None
        self._mid_msg = None

        self._build_ui()
        self._load_backends()

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg=PAL["panel"], height=60)
        hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr, text="🔬 NCERT MASTER LAB", fg=PAL["accent"], bg=PAL["panel"], font=("Segoe UI Bold",16)).pack(side="left", padx=20)
        
        # Main Body
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=5, pady=5)
        
        # Left Panel: Tree
        self._tree_fr = tk.Frame(body, bg=PAL["panel"], width=300)
        self._tree_fr.pack(side="left", fill="y", padx=(0,5)); self._tree_fr.pack_propagate(False)
        
        self._tree = ttk.Treeview(self._tree_fr, show="tree", selectmode="browse")
        self._tree.pack(fill="both", expand=True)
        self._tree.bind("<<TreeviewSelect>>", self._on_select)
        
        # Middle Panel: Dynamic Form
        self._mid = tk.Frame(body, bg=PAL["bg"], width=400)
        self._mid.pack(side="left", fill="y", padx=(0,5)); self._mid.pack_propagate(False)
        self._mid_msg = tk.Label(self._mid, text="Select an experiment\nfrom the list", fg=PAL["dim"], bg=PAL["bg"], font=("Segoe UI",11))
        self._mid_msg.pack(expand=True)
        
        # Right Panel: Output Console
        self._out_fr = tk.Frame(body, bg=PAL["bg"])
        self._out_fr.pack(side="right", fill="both", expand=True)
        self._out = scrolledtext.ScrolledText(self._out_fr, bg="#070910", fg="#00D26A", font=("Cascadia Code",10), borderwidth=0, padx=15, pady=15)
        self._out.pack(fill="both", expand=True)
        self._out.tag_config("title", foreground=PAL["accent"], font=("Segoe UI Bold",14))
        self._out.tag_config("key", foreground=PAL["ma"])

    def _load_backends(self):
        backend_info = [
            ("ncert_physics_lab", "PHYSICS_REGISTRY", PAL["ph"], "⚛ Physics"),
            ("ncert_chemistry_lab", "CHEMISTRY_REGISTRY", PAL["ch"], "🧪 Chemistry"),
            ("ncert_biology_lab", "BIOLOGY_REGISTRY", PAL["bi"], "🧬 Biology"),
            ("ncert_maths_lab", "MATHS_REGISTRY", PAL["ma"], "📐 Mathematics"),
            ("ncert_primary_science", "SCIENCE_PRIMARY_REGISTRY", PAL["ch"], "🌱 Primary Science"),
            ("ncert_primary_maths", "PRIMARY_MATHS_REGISTRY", PAL["ma"], "➕ Primary Math"),
        ]
        
        sys.path.insert(0, os.path.dirname(__file__))
        
        for mod_name, reg_name, color, label in backend_info:
            try:
                mod = importlib.import_module(mod_name)
                importlib.reload(mod)
                registry = getattr(mod, reg_name)
                self._add_to_tree(label, registry, color)
            except Exception as e:
                print(f"Failed to load {mod_name}: {e}")

    def _add_to_tree(self, label, registry, color):
        if not self._tree: return
        root = self._tree.insert("", "end", text=label, open=True)
        self._tree.tag_configure(root, foreground=color, font=("Segoe UI Bold", 10))
        
        for cls_label, cls_obj in registry.items():
            cls_node = self._tree.insert(root, "end", text=cls_label)
            for exp_display, data in cls_obj.EXP_DATA.items():
                exp_node = self._tree.insert(cls_node, "end", text=f"• {exp_display}")
                self._exp_map[exp_node] = (cls_obj, exp_display, data, color)

    def _on_select(self, _):
        if not self._tree: return
        sel = self._tree.selection()
        if not sel or sel[0] not in self._exp_map: return
        self._build_form(*self._exp_map[sel[0]])

    def _build_form(self, cls, name, data, color):
        if not self._mid: return
        for w in self._mid.winfo_children(): w.destroy()
        
        method_name, fields = data
        tk.Label(self._mid, text=name, fg=color, bg=PAL["bg"], font=("Segoe UI Bold",14), pady=10).pack()
        tk.Label(self._mid, text=getattr(cls,"TITLE",""), fg=PAL["dim"], bg=PAL["bg"], font=("Segoe UI",9)).pack()
        tk.Frame(self._mid, bg=PAL["border"], height=1).pack(fill="x", pady=15)
        
        entries = {}
        for f_label, f_def in fields:
            row = tk.Frame(self._mid, bg=PAL["bg"])
            row.pack(fill="x", padx=15, pady=5)
            tk.Label(row, text=f_label, fg=PAL["text"], bg=PAL["bg"], width=18, anchor="w").pack(side="left")
            e = tk.Entry(row, bg=PAL["card"], fg="white", relief="flat", insertbackground="white")
            e.insert(0, str(f_def))
            e.pack(side="right", fill="x", expand=True)
            entries[f_label] = e
            
        def run_exp():
            try:
                # Type-aware argument parsing
                args = []
                for label, _ in fields:
                    val = entries[label].get()
                    try: 
                        if "." in val or "e" in val: args.append(float(val))
                        else: args.append(int(val))
                    except: args.append(val)
                
                res = getattr(cls, method_name)(*args)
                self._show_res(name, res)
            except Exception as e:
                if self._out:
                    self._out.delete("1.0", "end")
                    self._out.insert("end", traceback.format_exc(), "err")

        tk.Button(self._mid, text="RUN SIMULATION", bg=color, fg="white", font=("Segoe UI Bold",10), relief="flat", command=run_exp, pady=10).pack(fill="x", padx=15, pady=20)

    def _show_res(self, name, res):
        if not self._out: return
        self._out.delete("1.0", "end")
        self._out.insert("end", f"▶  {name} RESULTS\n\n", "title")
        if isinstance(res, dict):
            for k, v in res.items():
                self._out.insert("end", f"   {k}: ", "key")
                self._out.insert("end", f"{v}\n\n")
        else:
            self._out.insert("end", f"   Result: {res}\n")

if __name__ == "__main__":
    NCERTMasterLab().mainloop()
