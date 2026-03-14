"""
SigmaOS NCERT Omni-Simulator & Universal Calculator v1.0
========================================================
The Ultimate Academic Suite for Physics, Chemistry, Biology & Math (1–12)
100% Native Python | Standard Library | Sovereign Design
"""
import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext
import math, json, os, sys

PAL = {
    "bg": "#050608",
    "panel": "#0E1018",
    "card": "#161B2A",
    "accent": "#6C63FF",
    "phys": "#3B82F6",
    "chem": "#10B981",
    "bio": "#EC4899",
    "math": "#F59E0B",
    "text": "#E2E8F0",
    "dim": "#94A3B8",
    "border": "#252B43"
}

CONSTANTS = {
    "G (Gravitational)": "6.674e-11 Nm²/kg²",
    "g (Acceleration)": "9.81 m/s²",
    "c (Light Speed)": "2.998e8 m/s",
    "h (Planck)": "6.626e-34 Js",
    "Na (Avogadro)": "6.022e23 mol⁻¹",
    "R (Gas Const)": "8.314 J/mol·K",
    "e (Charge)": "1.602e-19 C",
    "me (Electron Mass)": "9.109e-31 kg",
    "Sigma (Stefan)": "5.67e-8 W/m²K⁴",
    "pi (Math)": "3.14159..."
}

class NCERTOmniSimulator(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • NCERT OMNI-SIMULATOR PRO")
        self.geometry("1400x900")
        self.configure(bg=PAL["bg"])
        
        self.search_var = tk.StringVar()
        self.main_area = None
        self._build_ui()

    def _build_ui(self):
        # 1. Dashboard Header
        head = tk.Frame(self, bg=PAL["panel"], height=80)
        head.pack(fill="x"); head.pack_propagate(False)
        
        tk.Label(head, text="⚛ NCERT OMNI-SIMULATOR", font=("Segoe UI", 24, "bold"), fg=PAL["accent"], bg=PAL["panel"]).pack(side="left", padx=30)
        
        self.search_var = tk.StringVar()
        search_ent = tk.Entry(head, textvariable=self.search_var, bg=PAL["bg"], fg="white", 
                              font=("Segoe UI", 11), relief="flat", width=40, insertbackground="white")
        search_ent.pack(side="right", padx=30, pady=25)
        search_ent.insert(0, "Search NCERT Concepts (e.g. Optics, Titration)...")
        search_ent.bind("<FocusIn>", lambda e: search_ent.delete(0, tk.END))

        # 2. Main Layout (Sidebar + Grid)
        content = tk.Frame(self, bg=PAL["bg"])
        content.pack(fill="both", expand=True, padx=20, pady=20)
        
        # Sidebar: Subject Hubs
        side = tk.Frame(content, bg=PAL["panel"], width=280)
        side.pack(side="left", fill="y", padx=(0, 20))
        side.pack_propagate(False)
        
        tk.Label(side, text="SUBJECT VAULT", font=("Segoe UI Bold", 10), fg=PAL["dim"], bg=PAL["panel"]).pack(pady=(20, 10), anchor="w", padx=20)
        
        subjects = [
            ("PHYSICS LAB", PAL["phys"], self._show_phys),
            ("CHEMISTRY SUITE", PAL["chem"], self._show_chem),
            ("BIOLOGY MAPS", PAL["bio"], self._show_bio),
            ("MATHEMATICA", PAL["math"], self._show_math),
            ("PRIMARY HUB", PAL["accent"], self._show_primary)
        ]
        
        for name, color, cmd in subjects:
            btn = tk.Button(side, text=name, font=("Segoe UI Bold", 11), bg=PAL["card"], 
                            fg=color, relief="flat", anchor="w", padx=20, pady=15, 
                            cursor="hand2", command=cmd)
            btn.pack(fill="x", pady=5, padx=10)

        # Bottom Sidebar: Quick Stats
        tk.Label(side, text="SCIENTIFIC CONSTANTS", font=("Segoe UI Bold", 10), fg=PAL["dim"], bg=PAL["panel"]).pack(pady=(30, 10), anchor="w", padx=20)
        const_box = scrolledtext.ScrolledText(side, bg=PAL["bg"], fg=PAL["text"], font=("Consolas", 9), height=15, borderwidth=0)
        const_box.pack(fill="both", padx=10)
        for k, v in CONSTANTS.items():
            const_box.insert(tk.END, f"{k}:\n{v}\n\n")
        const_box.config(state=tk.DISABLED)

        # 3. Dynamic Display Area
        self.main_area = tk.Frame(content, bg=PAL["bg"])
        self.main_area.pack(side="right", fill="both", expand=True)
        
        self._show_welcome()

    def _show_welcome(self):
        self._clear_area()
        welcome = tk.Frame(self.main_area, bg=PAL["bg"])
        welcome.pack(expand=True)
        
        tk.Label(welcome, text="CHOOSE A RESEARCH DOMAIN", font=("Segoe UI", 20, "bold"), fg=PAL["text"], bg=PAL["bg"]).pack()
        tk.Label(welcome, text="Unified Simulator accessing 500+ NCERT data points", font=("Segoe UI", 11), fg=PAL["dim"], bg=PAL["bg"]).pack(pady=10)
        
        grid = tk.Frame(welcome, bg=PAL["bg"])
        grid.pack(pady=40)
        
        cards = [
            ("Periodic Table", "Explore elements and atomic data", PAL["chem"], "ncert_periodic_table"),
            ("Logic Lab", "Interactive Gate Simulations", PAL["accent"], "ncert_logic_circuit"),
            ("Optics Bench", "Mirror & Lens Formula Bench", PAL["phys"], "ncert_ray_optics"),
            ("Titration", "Acid-Base colorimetry lab", PAL["chem"], "ncert_titration_sim"),
            ("Physio Master", "Cardiac & Neural telemetry", PAL["bio"], "ncert_physio_hub")
        ]
        
        for name, desc, color, mod in cards:
            c = tk.Frame(grid, bg=PAL["card"], width=200, height=200, padx=20, pady=20)
            c.pack(side="left", padx=15)
            c.pack_propagate(False)
            tk.Label(c, text=name, font=("Segoe UI Bold", 14), fg=color, bg=PAL["card"]).pack(pady=10)
            tk.Label(c, text=desc, font=("Segoe UI", 9), fg=PAL["dim"], bg=PAL["card"], wraplength=160).pack()
            tk.Button(c, text="LAUNCH", font=("Segoe UI Bold", 8), bg=color, fg="black", relief="flat", command=lambda m=mod: self._launch_sublab(m)).pack(side="bottom", pady=5)

    def _clear_area(self):
        for w in self.main_area.winfo_children(): w.destroy()

    def _show_phys(self): self._launch_sublab("ncert_physics_lab")
    def _show_chem(self): self._launch_sublab("ncert_chemistry_lab")
    def _show_bio(self): self._launch_sublab("ncert_biology_lab")
    def _show_math(self): self._launch_sublab("ncert_maths_lab")
    def _show_primary(self): self._launch_sublab("ncert_primary_maths")

    def _launch_sublab(self, mod_name):
        # Open the specific lab or hub
        try:
            import importlib, subprocess
            # Try to launch as subprocess to keep main UI responsive
            subprocess.Popen([sys.executable, f"userland/apps/{mod_name}.py"])
            # messagebox.showinfo("Omni-Sync", f"Initializing {mod_name} Sub-Atomic Lab...")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to link module: {e}")

if __name__ == "__main__":
    app = NCERTOmniSimulator()
    app.mainloop()
