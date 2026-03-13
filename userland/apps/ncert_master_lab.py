"""
SigmaOS NCERT Master Lab v8.0 — The Interactive series
Unified Virtual Lab for Physics, Chemistry, Biology & Math (1–12)
100% stdlib/tkinter | Fully dynamic simulation hub
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback

# ─── PREMIUM THEME ───────────────────────────────────────────────────────────
PAL = {
    "bg":"#0B0D17", "panel":"#11142A", "card":"#181B2E", "accent":"#6C63FF",
    "ph":"#3B82F6", "ch":"#22C55E", "bi":"#EC4899", "ma":"#F59E0B",
    "text":"#E8E8F0", "dim":"#8888A0", "border":"#252840",
}

class NCERTMasterLab(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • NCERT Virtual Lab v8.0")
        self.geometry("1400x900")
        self.configure(bg=PAL["bg"])
        
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
        hdr = tk.Frame(self, bg=PAL["panel"], height=65)
        hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr, text="🔬 NCERT INTERACTIVE LAB v8.0", fg=PAL["accent"], bg=PAL["panel"], font=("Segoe UI Bold",18)).pack(side="left", padx=25)
        
        tk.Label(hdr, text="[INTERACTIVE MODE READY]", fg="#00D26A", bg=PAL["panel"], font=("Consolas",9)).pack(side="right", padx=25)

        # Body
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=8, pady=8)
        
        # Left Panel (Browse Tree)
        self._tree_fr = tk.Frame(body, bg=PAL["panel"], width=320)
        self._tree_fr.pack(side="left", fill="y", padx=(0,8)); self._tree_fr.pack_propagate(False)
        
        self._tree = ttk.Treeview(self._tree_fr, show="tree", selectmode="browse")
        self._tree.pack(fill="both", expand=True)
        self._tree.bind("<<TreeviewSelect>>", self._on_select)
        
        style = ttk.Style()
        style.configure("Treeview", background=PAL["panel"], foreground=PAL["text"], fieldbackground=PAL["panel"], borderwidth=0, font=("Segoe UI", 10))
        style.map("Treeview", background=[('selected', PAL["accent"])])
        
        # Middle Panel (Form)
        self._mid = tk.Frame(body, bg=PAL["bg"], width=420)
        self._mid.pack(side="left", fill="y", padx=(0,8)); self._mid.pack_propagate(False)
        self._mid_msg = tk.Label(self._mid, text="◄ SELECT AN EXPERIMENT", fg=PAL["dim"], bg=PAL["bg"], font=("Segoe UI Bold",12))
        self._mid_msg.pack(expand=True)
        
        # Right Panel (Console)
        self._out_fr = tk.Frame(body, bg=PAL["bg"])
        self._out_fr.pack(side="right", fill="both", expand=True)
        self._out = scrolledtext.ScrolledText(self._out_fr, bg="#070910", fg="#00D26A", font=("Cascadia Code",10), borderwidth=0, padx=20, pady=20)
        self._out.pack(fill="both", expand=True)
        self._out.tag_config("title", foreground=PAL["accent"], font=("Segoe UI Bold",15))
        self._out.tag_config("key", foreground=PAL["ma"])
        self._out.tag_config("err", foreground="#FF4B4B")

    def _load_backends(self):
        backend_info = [
            ("ncert_physics_lab", "PHYSICS_REGISTRY", PAL["ph"], "⚛ Physics (6-12)"),
            ("ncert_chemistry_lab", "CHEMISTRY_REGISTRY", PAL["ch"], "🧪 Chemistry (6-12)"),
            ("ncert_biology_lab", "BIOLOGY_REGISTRY", PAL["bi"], "🧬 Biology (6-12)"),
            ("ncert_maths_lab", "MATHS_REGISTRY", PAL["ma"], "📐 Mathematics (1-12)"),
            ("ncert_primary_science", "SCIENCE_PRIMARY_REGISTRY", PAL["ch"], "🌱 Primary Science (1-5)"),
            ("ncert_primary_maths", "PRIMARY_MATHS_REGISTRY", PAL["ma"], "➕ Primary Math (1-5)"),
        ]
        
        curr = os.path.dirname(os.path.abspath(__file__))
        if curr not in sys.path: sys.path.insert(0, curr)
        
        for mod_name, reg_name, color, label in backend_info:
            try:
                mod = importlib.import_module(mod_name)
                importlib.reload(mod)
                registry = getattr(mod, reg_name)
                self._add_to_tree(label, registry, color)
            except Exception:
                print(f"Failed to load {mod_name}")

    def _add_to_tree(self, label, registry, color):
        if not self._tree: return
        root = self._tree.insert("", "end", text=label, open=True)
        self._tree.tag_configure(root, foreground=color, font=("Segoe UI Bold", 11))
        
        for cls_label, cls_obj in registry.items():
            cls_node = self._tree.insert(root, "end", text=cls_label)
            sorted_exps = sorted(cls_obj.EXP_DATA.items())
            for exp_display, data in sorted_exps:
                node = self._tree.insert(cls_node, "end", text=f"• {exp_display}")
                self._exp_map[node] = (cls_obj, exp_display, data, color)

    def _on_select(self, _):
        if not self._tree: return
        sel = self._tree.selection()
        if not sel or sel[0] not in self._exp_map: return
        self._build_form(*self._exp_map[sel[0]])

    def _build_form(self, cls, name, data, color):
        if not self._mid: return
        for w in self._mid.winfo_children(): w.destroy()
        
        method_name, fields = data
        title_fr = tk.Frame(self._mid, bg=PAL["bg"])
        title_fr.pack(fill="x", pady=(20, 10))
        tk.Label(title_fr, text=name, fg=color, bg=PAL["bg"], font=("Segoe UI Bold",16)).pack()
        tk.Label(title_fr, text=getattr(cls,"TITLE",""), fg=PAL["dim"], bg=PAL["bg"], font=("Segoe UI Semibold",9)).pack()
        
        sep = tk.Frame(self._mid, bg=PAL["border"], height=1)
        sep.pack(fill="x", pady=15, padx=20)
        
        entries = {}
        for f_label, f_def in fields:
            row = tk.Frame(self._mid, bg=PAL["bg"])
            row.pack(fill="x", padx=25, pady=6)
            tk.Label(row, text=f_label, fg=PAL["text"], bg=PAL["bg"], width=20, anchor="w", font=("Segoe UI",10)).pack(side="left")
            e = tk.Entry(row, bg=PAL["card"], fg="white", relief="flat", insertbackground="white", font=("Consolas",10))
            e.insert(0, str(f_def))
            e.pack(side="right", fill="x", expand=True)
            entries[f_label] = e
            
        def run_sim():
            try:
                args = []
                for label, _ in fields:
                    v = entries[label].get()
                    try:
                        if "." in v or "e" in v: args.append(float(v))
                        else: args.append(int(v))
                    except: args.append(v)
                
                res = getattr(cls, method_name)(*args)
                self._show_res(name, res)
            except Exception:
                if self._out:
                    self._out.delete("1.0", "end")
                    self._out.insert("end", traceback.format_exc(), "err")

        btn = tk.Button(self._mid, text="RUN EXPERIMENT ⚗️", bg=color, fg="white", font=("Segoe UI Bold",11), relief="flat", command=run_sim, pady=12, cursor="hand2")
        btn.pack(fill="x", padx=25, pady=30)

    def _show_res(self, name, res):
        if not self._out: return
        self._out.delete("1.0", "end")
        self._out.insert("end", f"▶ INTERACTIVE LOG: {name.upper()}\n", "title")
        self._out.insert("end", "─" * 40 + "\n\n")
        
        if isinstance(res, dict):
            for k, v in res.items():
                self._out.insert("end", f" • {k}: ", "key")
                self._out.insert("end", f"{v}\n\n")
        else:
            self._out.insert("end", f" OUTPUT: {res}\n")

if __name__ == "__main__":
    NCERTMasterLab().mainloop()
