"""
SigmaOS Sovereign Legal Tracker (Gantt v1.0)
=============================================
USP: Indian Litigation Compliance Dashboard (BNS, BNSS, BSA).
Interactive Gantt chart with real-time e-Court simulation & SC Update absorption.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import time
import random
from typing import List, Dict, Any, cast

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#FFD60A", # Legal Gold
    "accent_dim": "#B89B00",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "pending": "#5856D6", # Purple
    "panel": "#1C1E24"
}

class LegalGantt(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Legal Tracker | Indian Law Compliance")
        self.geometry("1400x850")
        self.configure(bg=PAL["bg"])
        
        # Initialize attributes to satisfy linter before _build_ui
        self.case_data: List[Dict[str, Any]] = self._init_mock_case()
        self.sc_updates = ["SC: Finality of Investigation Timelines (2025)", "SC: Digital Evidence Admissibility Guidelines (2026)"]
        
        # UI components
        self.header = tk.Frame(self)
        self.workspace = tk.Frame(self)
        self.info_fr = tk.Frame(self)
        self.gantt_fr = tk.Frame(self)
        self.canvas = tk.Canvas(self)
        
        self._build_ui()
        self._animate_sync()

    def _init_mock_case(self) -> List[Dict[str, Any]]:
        return [
            {"name": "FIR Registration", "start": -20, "end": -19, "status": "DONE"},
            {"name": "Investigation", "start": -19, "end": -5, "status": "DONE"},
            {"name": "Evidence Audit", "start": -5, "end": 0, "status": "ACTIVE"},
            {"name": "Charge Framing", "start": 1, "end": 3, "status": "PENDING"},
            {"name": "Trial Stage", "start": 4, "end": 15, "status": "PENDING"},
            {"name": "Arguments", "start": 16, "end": 18, "status": "PENDING"},
            {"name": "Judgment", "start": 19, "end": 20, "status": "PENDING"},
            {"name": "Appeal Window", "start": 21, "end": 30, "status": "PENDING"},
        ]

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=80, padx=30)
        self.header.pack(side="top", fill="x", pady=20)
        
        title_fr = tk.Frame(self.header, bg=PAL["bg"])
        title_fr.pack(side="left")
        
        tk.Label(title_fr, text="SOVEREIGN LEGAL COMPLIANCE", font=("Inter", 24, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(anchor="w")
        tk.Label(title_fr, text="Indian Litigation Tracker — BNS | BNSS | BSA (2023 Codes)", font=("Inter", 10), fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w")
        
        ctrl_fr = tk.Frame(self.header, bg=PAL["bg"])
        ctrl_fr.pack(side="right")
        
        tk.Button(ctrl_fr, text="🔄 SYNC E-COURTS", bg=PAL["sidebar"], fg="white", font=("Inter", 9, "bold"), relief="flat", padx=20, pady=10, command=self._sync_ecourts).pack(side="left", padx=5)
        tk.Button(ctrl_fr, text="📜 SC UPDATES", bg=PAL["success"], fg="black", font=("Inter", 9, "bold"), relief="flat", padx=20, pady=10, command=self._show_sc_updates).pack(side="left", padx=5)

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=30)
        self.workspace.pack(fill="both", expand=True)

        # Left Sidebar (Case Info)
        self.info_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=350, padx=20, pady=20)
        self.info_fr.pack(side="left", fill="y", padx=(0, 30))
        self.info_fr.pack_propagate(False)

        tk.Label(self.info_fr, text="CASE OVERVIEW", font=("Inter", 11, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 15))
        
        case_details = [
            ("Case Number", "CRL/2026/SC/402"),
            ("Jurisdiction", "Supreme Court of India"),
            ("Case Type", "Criminal Plaint (BNS)"),
            ("Petitioner", "Sovereign Shard #1"),
            ("Respondent", "State of Sigma"),
            ("Compliance Score", "98.4%")
        ]
        for k, v in case_details:
            tk.Label(self.info_fr, text=k.upper(), font=("Inter", 8, "bold"), fg=PAL["accent_dim"], bg=PAL["panel"]).pack(anchor="w", pady=(5,0))
            tk.Label(self.info_fr, text=v, font=("Inter", 10), fg=PAL["text"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 10))

        # Right Panel (Gantt Chart)
        self.gantt_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        self.gantt_fr.pack(side="left", fill="both", expand=True)
        
        self.canvas = tk.Canvas(self.gantt_fr, bg=PAL["sidebar"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        
        self.draw_gantt()

    def draw_gantt(self):
        cv = self.canvas
        cv.delete("all")
        w = float(cv.winfo_width())
        h = float(cv.winfo_height())
        if w < 100.0: self.after(100, self._deferred_draw); return

        cv.create_rectangle(0, 0, w, 50, fill=PAL["panel"], outline="")
        
        start_day = -25.0
        end_day = 35.0
        range_days = end_day - start_day
        day_w = w / range_days

        for i in range(int(start_day), int(end_day), 5):
            x = (float(i) - start_day) * day_w
            cv.create_line(x, 50, x, h, fill="#252830", dash=(4, 4))
            label = "NOW" if i == 0 else f"Day {i}"
            col = PAL["accent"] if i == 0 else PAL["dim"]
            cv.create_text(x, 25, text=label, fill=col, font=("Inter", 8, "bold"))

        y_off = 80.0
        for i, task in enumerate(self.case_data):
            x_start = (float(task["start"]) - start_day) * day_w
            x_end = (float(task["end"]) - start_day) * day_w
            row_col = "#1E2026" if i % 2 == 0 else "#16181C"
            cv.create_rectangle(0, y_off-20, w, y_off+40, fill=row_col, outline="")
            color = PAL["success"] if task["status"] == "DONE" else PAL["pending"]
            if task["status"] == "ACTIVE": color = PAL["accent"]
            cv.create_rectangle(x_start, y_off, x_end, y_off+20, fill=color, outline="", width=0)
            cv.create_text(10, y_off+10, text=task["name"], fill=PAL["text"], font=("Inter", 9, "bold"), anchor="w")
            cv.create_text(x_end + 10, y_off+10, text=str(task["status"]), fill=PAL["dim"], font=("Inter", 8, "bold"), anchor="w")
            y_off += 70.0

    def _deferred_draw(self):
        self.draw_gantt()

    def _sync_ecourts(self):
        self.case_data[2]["status"] = "DONE"
        self.case_data[3]["status"] = "ACTIVE"
        self.draw_gantt()
        messagebox.showinfo("Sovereign Legal Tracker", "E-Courts Pulse Received: Evidence stage marked as COMPLIANT. Charge Framing initiated under BNSS 243.")

    def _show_sc_updates(self):
        msg = "\n\n".join(self.sc_updates)
        messagebox.showinfo("Supreme Court Sovereign Feed", f"Latest Absorption Matrix:\n\n{msg}")

    def _animate_sync(self):
        self.after(5000, self._animate_sync)

if __name__ == "__main__":
    app = LegalGantt()
    app.mainloop()
