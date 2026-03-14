"""
SigmaOS NCERT Master Lab v10.0 — The Ultimate Virtual Suite
Unified Virtual Lab for Physics, Chemistry, Biology & Math (1–12)
100% stdlib/tkinter | Exhaustive Interactive Experiment Hub
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback, json, time
from typing import Dict, Any, List, Optional

# Decouple via absolute path injection
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

try:
    from sigma_core.ui.fluid_design import PALETTE as FLUID_PAL, FluidTheme, ICONS # type: ignore
    from sigma_core.education.ncert_engine import NCERTEducationEngine # type: ignore
except ImportError:
    FLUID_PAL = None
    FluidTheme = None
    NCERTEducationEngine = None

# Adaptive Palette mapped to Fluid Design if available
PAL = {
    "bg": FLUID_PAL["background"] if FLUID_PAL else "#0B0D17",
    "panel": FLUID_PAL["surface"] if FLUID_PAL else "#11142A",
    "card": "#181B2E",
    "accent": FLUID_PAL["primary"] if FLUID_PAL else "#6C63FF",
    "ph": "#3B82F6", "ch": "#22C55E", "bi": "#EC4899", "ma": "#F59E0B",
    "text": FLUID_PAL["text_primary"] if FLUID_PAL else "#E8E8F0",
    "dim": FLUID_PAL["text_secondary"] if FLUID_PAL else "#8888A0",
    "border": FLUID_PAL["border"] if FLUID_PAL else "#252840",
}

class NCERTMasterLab(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.engine = NCERTEducationEngine(kernel) if NCERTEducationEngine else None
        self.title("SigmaOS • NCERT Virtual Lab v10.0")
        self.geometry("1400x900")
        self.configure(bg=PAL["bg"])
        
        self._mods = {}
        self._exp_map = {}
        self._user_identity = "SOVEREIGN_RESEARCHER"
        self._completed_count = 0
        
        # UI Proxies for static analysis
        self._tree: Any = None
        self._mid: Any = None
        self._out: Any = None
        self._tree_fr: Any = None
        self._out_fr: Any = None
        self._mid_msg: Any = None
        self._status_lbl: Any = None
        self.search_ent: Any = None

        self._build_ui()
        self._load_backends()

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg=PAL["panel"], height=65)
        hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr, text=f"{ICONS.get('ncert', '🔬')} LAB v10.0", fg=PAL["accent"], bg=PAL["panel"], font=("Segoe UI Bold",18)).pack(side="left", padx=25)
        
        self._status_lbl = tk.Label(hdr, text="[SYSTEM OPERATIONAL]", fg="#00D26A", bg=PAL["panel"], font=("Consolas",9))
        self._status_lbl.pack(side="right", padx=25)

        # Body
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=12, pady=12)
        
        # Left Panel (Browse Tree)
        self._tree_fr = tk.Frame(body, bg=PAL["panel"], width=320)
        self._tree_fr.pack(side="left", fill="y", padx=(0,12)); self._tree_fr.pack_propagate(False)
        
        self.search_ent = tk.Entry(self._tree_fr, bg=PAL["card"], fg="white", font=("Segoe UI", 9), relief="flat")
        self.search_ent.pack(fill="x", padx=10, pady=10)
        self.search_ent.insert(0, "Filter experiments...")
        
        self._tree = ttk.Treeview(self._tree_fr, show="tree", selectmode="browse")
        self._tree.pack(fill="both", expand=True)
        self._tree.bind("<<TreeviewSelect>>", self._on_select)
        
        # Middle Panel (Interaction Form)
        self._mid = tk.Frame(body, bg=PAL["bg"], width=450)
        self._mid.pack(side="left", fill="y", padx=(0,12)); self._mid.pack_propagate(False)
        self._mid_msg = tk.Label(self._mid, text="◄ SELECT AN EXPERIMENT", fg=PAL["dim"], bg=PAL["bg"], font=("Segoe UI Bold",12))
        self._mid_msg.pack(expand=True)
        
        # Right Panel (Scientific Console)
        self._out_fr = tk.Frame(body, bg=PAL["bg"])
        self._out_fr.pack(side="right", fill="both", expand=True)
        self._out = scrolledtext.ScrolledText(self._out_fr, bg="#070910", fg="#00D26A", font=("Cascadia Code",10), borderwidth=0, padx=20, pady=20)
        self._out.pack(fill="both", expand=True)
        self._out.tag_config("title", foreground=PAL["accent"], font=("Segoe UI Bold",15))
        self._out.tag_config("badge", foreground=PAL["ch"], font=("Segoe UI Bold",12))
        self._out.tag_config("err", foreground="#FF4B4B")

    def _load_backends(self):
        backend_info = [
            ("ncert_physics_lab", "PHYSICS_REGISTRY", PAL["ph"], f"{ICONS.get('ncert', '⚛')} Physics (6-12)"),
            ("ncert_chemistry_lab", "CHEMISTRY_REGISTRY", PAL["ch"], f"{ICONS.get('genai_lab', '🧪')} Chemistry (6-12)"), # Using genai_lab icon for chem
            ("ncert_biology_lab", "BIOLOGY_REGISTRY", PAL["bi"], f"{ICONS.get('ml_engine', '🧬')} Biology (6-12)"),
            ("ncert_maths_lab", "MATHS_REGISTRY", PAL["ma"], f"{ICONS.get('calculator', '📐')} Mathematics (1-12)"),
        ]
        for mod_name, reg_name, color, label in backend_info:
            try:
                mod = importlib.import_module(mod_name)
                registry = getattr(mod, reg_name)
                self._add_to_tree(label, registry, color)
            except: pass

    def _add_to_tree(self, label, registry, color):
        root = self._tree.insert("", "end", text=label, open=True)
        for cls_label, cls_obj in registry.items():
            cls_node = self._tree.insert(root, "end", text=cls_label)
            for exp_display, data in cls_obj.EXP_DATA.items():
                node = self._tree.insert(cls_node, "end", text=f"• {exp_display}")
                self._exp_map[node] = (cls_obj, exp_display, data, color)

    def _on_select(self, _):
        sel = self._tree.selection()
        if not sel or sel[0] not in self._exp_map: return
        self._build_form(*self._exp_map[sel[0]])

    def _build_form(self, cls, name, data, color):
        for w in self._mid.winfo_children(): w.destroy()
        method_name, fields = data
        tk.Label(self._mid, text=name, fg=color, bg=PAL["bg"], font=("Segoe UI Bold",16)).pack(pady=20)
        
        entries = {}
        for f_label, f_def in fields:
            row = tk.Frame(self._mid, bg=PAL["bg"])
            row.pack(fill="x", padx=25, pady=6)
            tk.Label(row, text=f_label, fg=PAL["text"], bg=PAL["bg"], width=20, anchor="w").pack(side="left")
            e = tk.Entry(row, bg=PAL["card"], fg="white", relief="flat")
            e.insert(0, str(f_def))
            e.pack(side="right", fill="x", expand=True)
            entries[f_label] = e
            
        def run_sim():
            try:
                args = [float(entries[l].get()) if "." in entries[l].get() else int(entries[l].get()) for l, _ in fields]
                res = getattr(cls, method_name)(*args)
                self._completed_count += 1
                if self.engine and hasattr(self.engine, "earn_xp"): 
                    self.engine.earn_xp(150) # type: ignore
                self._show_res(name, res)
            except Exception:
                self._out.insert("end", traceback.format_exc(), "err")

        tk.Button(self._mid, text=f"{ICONS.get('bootloader', '🚀')} EXECUTE SIMULATION", bg=color, fg="white", command=run_sim, pady=10).pack(fill="x", padx=25, pady=35)

    def _show_res(self, name, res):
        self._out.delete("1.0", "end")
        self._out.insert("end", f"▶ LAB LOG: {name.upper()}\n", "title")
        self._out.insert("end", f"OUTPUT: {res}\n\n")
        if self.engine and hasattr(self.engine, "xp"):
            xp = getattr(self.engine, "xp", 0)
            self._out.insert("end", f"◈ XP EARNED: +150 | TOTAL: {xp}\n", "badge")
            if self._status_lbl:
                self._status_lbl.config(text=f"[XP: {xp} | COMPLETED: {self._completed_count}]")

if __name__ == "__main__":
    NCERTMasterLab().mainloop()
