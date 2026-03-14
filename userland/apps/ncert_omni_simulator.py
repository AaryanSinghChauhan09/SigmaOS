"""
SigmaOS NCERT Omni-Simulator & Universal Calculator v1.0
========================================================
The Ultimate Academic Suite for Physics, Chemistry, Biology & Math (1–12)
100% Native Python | Standard Library | Sovereign Design
"""
import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext
import math, json, os, sys
# Robust System Path Injection for Sovereign Interop
_p = os.path.abspath(__file__)
while _p and not os.path.exists(os.path.join(os.path.dirname(_p), "sigma_core")):
    _p = os.path.dirname(_p)
    if _p == os.path.dirname(_p): break # Root reached
root = str(os.path.dirname(_p))
if root and root not in sys.path: sys.path.insert(0, root)

from sigma_core.app_discovery import AppDiscovery
from userland.system_api.settings_manager import SettingsManager
from sigma_core.gamification_engine import GamificationEngine
from sigma_core.system_monitor import SystemMonitor
from sigma_core.data_visualizer import DataVisualizer
from sigma_core.plugin_hub import PluginHub
from sigma_core.privacy_sentinel import PrivacySentinel

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
        
        self.settings = SettingsManager.load()
        self.game = GamificationEngine()
        PluginHub.initialize()
        self.search_var: tk.StringVar = tk.StringVar()
        self.main_area: tk.Frame = tk.Frame()
        self.status_tray: tk.Frame = tk.Frame()
        self.content_frame: tk.Frame = tk.Frame()
        self.health_lbl: tk.Label = tk.Label()
        
        self._build_ui()
        self._update_loop()

    def _build_ui(self):
        # 1. Dashboard Header
        head = tk.Frame(self, bg=PAL["panel"], height=80)
        head.pack(fill="x"); head.pack_propagate(False)
        
        user = self.settings.get("user_name", "Researcher")
        tk.Label(head, text=f"⚛ OMNI-LAB • Welcome, {user}", font=("Segoe UI", 24, "bold"), fg=PAL["accent"], bg=PAL["panel"]).pack(side="left", padx=30)
        
        search_ent = tk.Entry(head, textvariable=self.search_var, bg=PAL["bg"], fg="white", 
                              font=("Segoe UI", 11), relief="flat", width=40, insertbackground="white")
        search_ent.pack(side="right", padx=30, pady=25)
        search_ent.insert(0, "Search NCERT Concepts (e.g. Optics, Titration)...")
        search_ent.bind("<FocusIn>", lambda e: search_ent.delete(0, tk.END))

        # 2. Main Layout (Sidebar + Context)
        self.content_frame = tk.Frame(self, bg=PAL["bg"])
        self.content_frame.pack(fill="both", expand=True, padx=20, pady=20)
        
        # Sidebar: Subject Hubs + Gamification
        side = tk.Frame(self.content_frame, bg=PAL["panel"], width=280)
        side.pack(side="left", fill="y", padx=(0, 20))
        side.pack_propagate(False)

        # Gamified Status Bar
        self.status_tray = tk.Frame(side, bg=PAL["card"], height=120)
        self.status_tray.pack(fill="x", padx=10, pady=10)
        self._render_game_status()

        tk.Label(side, text="CENTRAL COMMAND", font=("Segoe UI Bold", 10), fg=PAL["dim"], bg=PAL["panel"]).pack(pady=(20, 10), anchor="w", padx=20)
        
        subjects = [
            ("PHYSICS LAB", PAL["phys"], self._show_phys),
            ("CHEMISTRY SUITE", PAL["chem"], self._show_chem),
            ("BIOLOGY MAPS", PAL["bio"], self._show_bio),
            ("MATHEMATICA", PAL["math"], self._show_math),
            ("PRIMARY HUB", PAL["accent"], self._show_primary),
            ("ANALYTICS HUB", PAL["success"], self._show_analytics),
            ("COMMUNITY PLUGINS", PAL["dim"], self._show_plugins),
            ("OS SETTINGS", PAL["dim"], self._show_settings)
        ]
        
        for name, color, cmd in subjects:
            btn = tk.Button(side, text=name, font=("Segoe UI Bold", 11), bg=PAL["card"], 
                            fg=color, relief="flat", anchor="w", padx=20, pady=15, 
                            cursor="hand2", command=cmd)
            btn.pack(fill="x", pady=5, padx=10)

        # Bottom Sidebar: System Health (Resilience & Transparency)
        tk.Label(side, text="SYSTEM INTEG", font=("Segoe UI Bold", 10), fg=PAL["dim"], bg=PAL["panel"]).pack(pady=(30, 5), anchor="w", padx=20)
        self.health_lbl = tk.Label(side, text="CPU: -- | RAM: --", font=("Consolas", 8), fg=PAL["chem"], bg=PAL["bg"], pady=10)
        self.health_lbl.pack(fill="x", padx=10)

        # 3. Dynamic Display Area
        self.main_area = tk.Frame(self.content_frame, bg=PAL["bg"])
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
        
        discovered = AppDiscovery.find_apps()
        
        for name_obj, mod_obj in discovered.items():
            name = str(name_obj)
            mod = str(mod_obj)
            if name in ["Periodic Table", "Logic Lab", "Optics Bench", "Titration", "Physio Master"]:
                # High-priority specialized apps
                color = PAL["chem"] if "Table" in name else PAL["phys"]
                desc = "Specialized Research Simulation"
                
                c = tk.Frame(grid, bg=PAL["card"], width=200, height=200, padx=20, pady=20)
                c.pack(side="left", padx=15)
                c.pack_propagate(False)
                tk.Label(c, text=str(name), font=("Segoe UI Bold", 12), fg=color, bg=PAL["card"]).pack(pady=10)
                tk.Label(c, text=desc, font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["card"], wraplength=160).pack()
                tk.Button(c, text="LAUNCH", font=("Segoe UI Bold", 8), bg=color, fg="black", relief="flat", command=self._mk_cmd(str(mod))).pack(side="bottom", pady=5)

    def _mk_cmd(self, m):
        return lambda: self._launch_sublab(m)

    def _clear_area(self):
        for w in self.main_area.winfo_children(): w.destroy()

    def _show_phys(self): self._launch_sublab("ncert_physics_lab")
    def _show_chem(self): self._launch_sublab("ncert_chemistry_lab")
    def _show_bio(self): self._launch_sublab("ncert_biology_lab")
    def _show_math(self): self._launch_sublab("ncert_maths_lab")
    def _show_primary(self):
        self._clear_area()
        pane = tk.Frame(self.main_area, bg=PAL["card"], padx=20, pady=20)
        pane.pack(expand=True)
        tk.Label(pane, text="PRIMARY KNOWLEDGE HUB", font=("Segoe UI Bold", 16), fg=PAL["accent"], bg=PAL["card"]).pack(pady=20)
        tk.Button(pane, text="MATHEMATICS (1-5)", bg=PAL["math"], fg="black", relief="flat", command=lambda: self._launch_sublab("ncert_primary_maths"), width=30, pady=10).pack(pady=5)
        tk.Button(pane, text="SCIENCE (1-5)", bg=PAL["phys"], fg="black", relief="flat", command=lambda: self._launch_sublab("ncert_primary_science"), width=30, pady=10).pack(pady=5)

    def _render_game_status(self):
        for w in self.status_tray.winfo_children(): w.destroy()
        st = self.game.get_status()
        tk.Label(self.status_tray, text=f"RANK: {self.settings.get('user_name', 'Researcher')}", fg=PAL["accent"], bg=PAL["card"], font=("Segoe UI Bold", 10)).pack(pady=5)
        tk.Label(self.status_tray, text=f"Level {st['Level']} Scientific Pioneer", fg="white", bg=PAL["card"], font=("Segoe UI", 9)).pack()
        
        # Micro Progress Bar (XP)
        prog = tk.Frame(self.status_tray, bg="#1A1E30", height=4)
        prog.pack(fill="x", padx=20, pady=10)
        fill_w = int((st["Total XP"] % 500) / 500 * 160)
        tk.Frame(prog, bg=PAL["accent"], width=fill_w, height=4).pack(side="left")
        
        tk.Label(self.status_tray, text=f"XP: {st['Total XP']} | Labs: {st['Labs Done']}", fg=PAL["dim"], bg=PAL["card"], font=("Segoe UI", 8)).pack()

    def _show_settings(self):
        self._clear_area()
        pane = tk.Frame(self.main_area, bg=PAL["card"], padx=40, pady=40)
        pane.pack(expand=True)
        
        tk.Label(pane, text="OS PERSONALIZATION", font=("Segoe UI Bold", 16), fg=PAL["accent"], bg=PAL["card"]).pack(pady=(0, 20))
        
        tk.Label(pane, text="User Identity:", fg="white", bg=PAL["card"]).pack(anchor="w")
        name_ent = tk.Entry(pane, bg=PAL["bg"], fg="white", relief="flat")
        name_ent.pack(fill="x", pady=5)
        name_ent.insert(0, self.settings.get("user_name", ""))
        
        def save():
            SettingsManager.update_key("user_name", name_ent.get())
            messagebox.showinfo("Sync", "Sovereign Profile Updated.")
            self._show_welcome()
            
        tk.Button(pane, text="SAVE PROFILE", bg=PAL["accent"], fg="white", relief="flat", command=save).pack(pady=10)
        
        def run_audit():
            leaks = PrivacySentinel.audit_directory()
            if not leaks:
                messagebox.showinfo("Privacy Audit", "Compliance Verified: 0 PII Leaks Detected.")
            else:
                messagebox.showwarning("Privacy Audit", f"Security Warning: {len(leaks)} potential PII leaks detected.")
                
        tk.Button(pane, text="RUN PRIVACY AUDIT", bg=PAL["dim"], fg="white", relief="flat", command=run_audit).pack(pady=5)

    def _show_analytics(self):
        self._clear_area()
        pane = tk.Frame(self.main_area, bg=PAL["card"], padx=20, pady=20)
        pane.pack(fill="both", expand=True)
        
        tk.Label(pane, text="RESEARCH PERFORMANCE ANALYTICS", font=("Segoe UI Bold", 16), fg=PAL["success"], bg=PAL["card"]).pack(pady=10)
        
        canv = tk.Canvas(pane, bg=PAL["bg"], height=300, highlightthickness=0)
        canv.pack(fill="x", pady=20)
        
        # Simulate experimental data for visualization
        sim_data = [10, 25, 15, 45, 30, 60, 50, 85, 70, 100]
        DataVisualizer.draw_line_graph(canv, sim_data, 1000, 300)
        
        tk.Label(pane, text="Lab Proficiency Trends (Historical Sync)", font=("Segoe UI", 9), fg=PAL["dim"], bg=PAL["card"]).pack()

    def _show_plugins(self):
        self._clear_area()
        pane = tk.Frame(self.main_area, bg=PAL["card"], padx=40, pady=40)
        pane.pack(expand=True)
        
        tk.Label(pane, text="COMMUNITY PLUGIN VAULT", font=("Segoe UI Bold", 16), fg=PAL["dim"], bg=PAL["card"]).pack(pady=20)
        
        plugins = PluginHub.list_plugins()
        if not plugins:
            tk.Label(pane, text="No Community Plugins Found.", fg=PAL["dim"], bg=PAL["card"]).pack()
        else:
            for p in plugins:
                btn = tk.Button(pane, text=f"LOAD {p.upper()}", bg=PAL["bg"], fg="white", relief="flat", padx=20, pady=10)
                btn.pack(fill="x", pady=5)

    def _update_loop(self):
        """Adaptive System Pulse."""
        self._refresh_health()
        self.after(5000, self._update_loop)

    def _refresh_health(self):
        # Transparency: System health in footer
        report = SystemMonitor.get_health_report()
        self.health_lbl.config(text=f"CPU: {report['CPU']} | RAM: {report['RAM']}")
        if "PERFORMANCE" in report["PowerState"]:
            self.health_lbl.config(fg="#F87171") # Alert red
        else:
            self.health_lbl.config(fg=PAL["chem"])

    def _launch_sublab(self, mod_name):
        # Gamified Reward
        self.game.record_experiment(mod_name)
        self._render_game_status()
        
        # Open the specific lab or hub
        try:
            import subprocess
            path = os.path.join("userland", "apps", f"{mod_name}.py")
            if os.path.exists(path):
                subprocess.Popen([sys.executable, path])
            else:
                SystemMonitor.log_incident("SublabLauncher", f"Module {mod_name} not found at {path}")
        except Exception as e:
            SystemMonitor.log_incident("SublabLauncher", str(e))
            messagebox.showerror("Error", f"Link failure: {e}")

if __name__ == "__main__":
    app = NCERTOmniSimulator()
    app.mainloop()
