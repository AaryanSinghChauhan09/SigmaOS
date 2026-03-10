"""
SigmaOS Sovereign ProjectFlow Apex Pro (v4.0)
=============================================
Professional Scrum, Gantt, and Time Tracking orchestration.
USP: Neural-Timeline Prediction & Zero-G Workflow Mapping.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

PAL = {
    "bg": "#08080A",
    "sidebar": "#111116",
    "card": "#1C1C24",
    "accent": "#5E5CE6", # Deep Purple
    "secondary": "#AF52DE",
    "text": "#E8E8E8",
    "dim": "#8E8E93",
    "border": "#2C2C34",
    "success": "#32D74B",
    "warning": "#FFD60A"
}

class ProjectFlow(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("ProjectFlow Apex Pro v4.0")
        self.geometry("1300x900")
        self.configure(bg=PAL["bg"])
        
        self._setup_style()
        self._build_ui()
        self._set_status("TIMELINE SYNCED | NEURAL ESTIMATION ACTIVE", PAL["accent"])

    def _setup_style(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure("PF.TNotebook", background=PAL["bg"], borderwidth=0)
        s.configure("PF.TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["text"], 
                    padding=[25, 12], font=("Inter", 9, "bold"))
        s.map("PF.TNotebook.Tab", background=[("selected", PAL["accent"])])

    def _build_ui(self):
        # Header
        head = tk.Frame(self, bg=PAL["bg"], padx=40, pady=30)
        head.pack(fill="x")
        
        tk.Label(head, text="PROJECTFLOW PRO", font=("Inter", 24, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        self.time_lbl = tk.Label(head, text="SESSION: 02:45:12", font=("JetBrains Mono", 12), fg=PAL["success"], bg=PAL["bg"])
        self.time_lbl.pack(side="right", padx=20)
        
        # Tabs
        self.tabs = ttk.Notebook(self, style="PF.TNotebook")
        self.tabs.pack(fill="both", expand=True, padx=40)

        # 1. Scrum Board
        scrum_tab = tk.Frame(self.tabs, bg=PAL["bg"])
        self.tabs.add(scrum_tab, text=" 📋 SCRUM BOARD ")
        self._init_scrum(scrum_tab)

        # 2. Gantt Chart
        gantt_tab = tk.Frame(self.tabs, bg=PAL["bg"])
        self.tabs.add(gantt_tab, text=" 📊 GANTT CHART ")
        self._init_gantt(gantt_tab)

        # 3. Time Tracker
        track_tab = tk.Frame(self.tabs, bg=PAL["bg"])
        self.tabs.add(track_tab, text=" ⏱️ TIME TRACKER ")
        self._init_tracker(track_tab)

        # Status
        self.status = tk.Label(self, text="", bg=PAL["accent"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _set_status(self, msg, color=PAL["accent"]):
        self.status.config(text=msg.upper(), bg=color)

    # --- Scrum Logic ---
    def _init_scrum(self, parent):
        cols = ["BACKLOG", "IN PROGRESS", "REVIEW", "DONE"]
        body = tk.Frame(parent, bg=PAL["bg"], pady=20)
        body.pack(fill="both", expand=True)

        for i, name in enumerate(cols):
            col_fr = tk.Frame(body, bg=PAL["sidebar"], width=280, highlightthickness=1, highlightbackground=PAL["border"])
            col_fr.pack(side="left", fill="both", expand=True, padx=10)
            col_fr.pack_propagate(False)
            
            tk.Label(col_fr, text=name, font=("Inter", 10, "bold"), bg=PAL["sidebar"], fg=PAL["dim"], pady=15).pack(fill="x")
            
            # Dummy Cards
            for _ in range(random.randint(1, 3)):
                card = tk.Frame(col_fr, bg=PAL["card"], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL["border"])
                card.pack(fill="x", padx=10, pady=5)
                tk.Label(card, text=f"Task {random.randint(100, 999)}", font=("Inter", 10, "bold"), bg=PAL["card"], fg="white").pack(anchor="w")
                tk.Label(card, text="Neural-Loom Optimization", font=("Inter", 8), bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w", pady=(5,0))

    # --- Gantt Logic ---
    def _init_gantt(self, parent):
        canvas = tk.Canvas(parent, bg="#000", highlightthickness=0)
        canvas.pack(fill="both", expand=True, padx=20, pady=20)
        
        # Grid
        for i in range(10):
            x = 100 + i*100
            canvas.create_line(x, 0, x, 800, fill="#1C1C24")
            canvas.create_text(x, 20, text=f"Wk {i+1}", fill=PAL["dim"], font=("Inter", 8))

        tasks = [("Core Kernel", 50, 200, PAL["accent"]), ("UI Refactor", 250, 450, PAL["secondary"]), ("Security Audit", 400, 700, PAL["success"])]
        for i, (name, x1, x2, color) in enumerate(tasks):
            y = 100 + i*60
            canvas.create_text(50, y+10, text=name, fill="white", anchor="e", font=("Inter", 9))
            canvas.create_rectangle(x1, y, x2, y+20, fill=color, outline="")

    # --- Time Tracker ---
    def _init_tracker(self, parent):
        body = tk.Frame(parent, bg=PAL["bg"], pady=40)
        body.pack()
        
        tk.Label(body, text="ACTIVE MODULE TRACKING", font=("Inter", 16, "bold"), fg=PAL["secondary"], bg=PAL["bg"]).pack()
        
        self.big_time = tk.Label(body, text="02:45:12", font=("JetBrains Mono", 72, "bold"), fg="white", bg=PAL["bg"])
        self.big_time.pack(pady=40)
        
        btn_fr = tk.Frame(body, bg=PAL["bg"])
        btn_fr.pack()
        
        tk.Button(btn_fr, text="PAUSE SESSION", bg=PAL["warning"], fg="black", font=("Inter", 10, "bold"), 
                  relief="flat", padx=30, pady=12).pack(side="left", padx=10)
        tk.Button(btn_fr, text="LOG ACTIVITY", bg=PAL["accent"], fg="white", font=("Inter", 10, "bold"), 
                  relief="flat", padx=30, pady=12).pack(side="left", padx=10)

if __name__ == "__main__":
    app = ProjectFlow()
    app.mainloop()
