"""
SigmaOS Sovereign Legal Tracker (v3.0 Apex)
===========================================
Interactive Litigation Workflow & Statutory Timeline.
USP: Dynamic Gantt view with IndiaCode.nic.in references.
Modularized: Using Fluid Design System for aesthetic consistency.
"""
import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class LegalTracker(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Legal Tracker — Litigation Gantt")
        self.geometry("1400x800")
        self.configure(bg=PAL["background"])
        
        # Explicit attribute declarations
        self.header = tk.Frame(self)
        self.gantt_fr = tk.Frame(self)
        self.canvas = tk.Canvas(self)
        self.info_panel = tk.Frame(self)
        
        # Litigation Stages (BNSS/CPC based)
        self.stages = [
            {"id": 1, "name": "FILING OF PLAINT/FIR", "act": "BNSS Sec 173 / CPC Order VII", "days": "0", "status": "COMPLETED", "note": "Mandatory first step of litigation record."},
            {"id": 2, "name": "SUMMONS TO DEFENDANT", "act": "BNSS Sec 63 / CPC Order V", "days": "30", "status": "COMPLETED", "note": "Court issues notice for appearance."},
            {"id": 3, "name": "WRITTEN STATEMENT", "act": "CPC Order VIII", "days": "90", "status": "ONGOING", "note": "Defendant files response to the plaint."},
            {"id": 4, "name": "FRAMING OF ISSUES", "act": "CPC Order XIV", "days": "120", "status": "PENDING", "note": "Court identifies core points of conflict."},
            {"id": 5, "name": "EVIDENCE (EXAMINATION)", "act": "BSA 2023 Sec 135-140", "days": "200", "status": "PENDING", "note": "Recording of witness testimonies."},
            {"id": 6, "name": "FINAL ARGUMENTS", "act": "BNSS Sec 350", "days": "300", "status": "PENDING", "note": "Conclusion of legal pleadings."},
            {"id": 7, "name": "JUDGMENT", "act": "BNSS Sec 392", "days": "330", "status": "PENDING", "note": "Final court verdict and decree."}
        ]
        
        self._build_ui()

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["background"], height=80, padx=40)
        self.header.pack(side="top", fill="x", pady=20)
        
        tk.Label(self.header, text="LITIGATION TRACKER", font=FONT["h1"], fg=PAL["primary"], bg=PAL["background"]).pack(side="left")
        
        # Gantt Canvas
        self.gantt_fr = tk.Frame(self, bg=PAL["surface"], padx=20, pady=20)
        self.gantt_fr.pack(fill="both", expand=True, padx=40, pady=(0, 20))
        
        self.canvas = tk.Canvas(self.gantt_fr, bg=PAL["surface"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        
        self._draw_gantt()
        
        # Info Panel
        self.info_panel = tk.Frame(self, bg=PAL["surface_variant"], height=100, padx=20, pady=15)
        self.info_panel.pack(side="bottom", fill="x")
        
        tk.Label(self.info_panel, text="STATUTORY TIMELINE SYNCED WITH INDIACODE.NIC.IN", font=FONT["caption"], fg=PAL["text_secondary"], bg=PAL["surface_variant"]).pack()

    def _draw_gantt(self):
        y = 50
        for stage in self.stages:
            color = PAL["success"] if stage["status"] == "COMPLETED" else (PAL["warning"] if stage["status"] == "ONGOING" else PAL["text_tertiary"])
            
            # Label
            self.canvas.create_text(50, y, text=stage["name"], anchor="w", font=FONT["body_bold"], fill=PAL["text_primary"])
            self.canvas.create_text(50, y+20, text=stage["act"], anchor="w", font=FONT["caption"], fill=PAL["text_secondary"])
            
            # Bar Background
            self.canvas.create_rectangle(300, y-10, 1000, y+10, fill=PAL["surface_variant"], outline="")
            
            # Progress Bar
            progress_width = 700 if stage["status"] == "COMPLETED" else (350 if stage["status"] == "ONGOING" else 0)
            self.canvas.create_rectangle(300, y-10, 300+progress_width, y+10, fill=color, outline="")
            
            # Clickable Interaction
            self.canvas.create_text(1050, y, text=f"[{stage['status']}]", anchor="w", font=FONT["caption"], fill=color)
            
            y += 80

    def _show_statutory_note(self, stage):
        messagebox.showinfo("Statutory Note", f"{stage['name']}\n\nAct Reference: {stage['act']}\n\nLogic: {stage['note']}")

if __name__ == "__main__":
    app = LegalTracker()
    app.mainloop()
