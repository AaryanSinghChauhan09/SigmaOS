"""
SigmaOS Advocate Command Center (v2.0 Apex)
===========================================
The Definitive Desktop for Indian Advocates & Lawyers.
Modularized: Using Fluid Design System for aesthetic consistency.
USP: Unified litigation workspace outperforming dedicated legal SaaS.
"""
import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional, Union, Callable
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class AdvocateCommandCenter(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Advocate Command Center | SigmaOS Legal Sovereign")
        self.geometry("1450x900")
        self.configure(bg=PAL["background"])
        
        # Explicit attribute declarations
        self.sidebar_fr = tk.Frame(self)
        self.content_fr = tk.Frame(self)
        self.header_fr = tk.Frame(self)
        self.stats_fr = tk.Frame(self)
        self.hearing_tree = ttk.Treeview(self)
        self.active_view = "DASHBOARD"
        
        self._setup_styles()
        self._build_sidebar()
        self._build_main_view()
        self._refresh_data()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use("clam")
        style.configure("Custom.Treeview", background=PAL["surface"], foreground=PAL["text_primary"], fieldbackground=PAL["surface"], borderwidth=0, font=FONT["body"])
        style.configure("Custom.Treeview.Heading", background=PAL["surface_variant"], foreground=PAL["secondary"], font=FONT["body_bold"])
        style.map("Custom.Treeview", background=[('selected', PAL["secondary"])], foreground=[('selected', PAL["background"])])

    def _build_sidebar(self):
        self.sidebar_fr = tk.Frame(self, bg=PAL["surface"], width=300, padx=25, pady=30)
        self.sidebar_fr.pack(side="left", fill="y")
        self.sidebar_fr.pack_propagate(False)
        
        tk.Label(self.sidebar_fr, text="SIGMA LEGAL", font=FONT["h2"], fg=PAL["secondary"], bg=PAL["surface"]).pack(anchor="w", pady=(0, 40))
        
        nav_items = [
            ("🏠 DASHBOARD", "DASHBOARD"),
            ("📚 CASE FILES", "CASES"),
            ("🗓️ CAUSE LIST", "CAUSALIST"),
            ("✍️ FORM DRAFTER", "FORMS"),
            ("💰 BILLING", "BILLING"),
            ("⚙️ SETTINGS", "SETTINGS")
        ]
        
        for lbl, tag in nav_items:
            def make_nav(t: str) -> Callable[[], Any]:
                return lambda: self._switch_view(t)
            
            btn = tk.Button(self.sidebar_fr, text=lbl, font=FONT["body"], fg=PAL["text_primary"], bg=PAL["surface"],
                            relief="flat", anchor="w", padx=10, pady=12, command=make_nav(str(tag)))
            btn.pack(fill="x", pady=2)

        # Lawyer Info Tag
        bot_tag = tk.Frame(self.sidebar_fr, bg=PAL["surface_variant"], padx=15, pady=15)
        bot_tag.pack(side="bottom", fill="x")
        tk.Label(bot_tag, text="ADVOCATE ON RECORD", font=FONT["caption"], fg=PAL["text_secondary"], bg=PAL["surface_variant"]).pack(anchor="w")
        tk.Label(bot_tag, text="SOVEREIGN_USER", font=FONT["body_bold"], fg=PAL["text_primary"], bg=PAL["surface_variant"]).pack(anchor="w")

    def _build_main_view(self):
        self.content_fr = tk.Frame(self, bg=PAL["background"], padx=50, pady=40)
        self.content_fr.pack(side="left", fill="both", expand=True)
        
        # Header
        self.header_fr = tk.Frame(self.content_fr, bg=PAL["background"])
        self.header_fr.pack(fill="x", pady=(0, 30))
        
        tk.Label(self.header_fr, text="LITIGATION PULSE", font=FONT["h1"], fg=PAL["text_primary"], bg=PAL["background"]).pack(side="left")
        
        # Actions
        act_fr = tk.Frame(self.header_fr, bg=PAL["background"])
        act_fr.pack(side="right")
        tk.Button(act_fr, text="+ NEW CASE", bg=PAL["secondary"], fg=PAL["background"], font=FONT["caption"], relief="flat", padx=20, pady=10, command=self._mock_add_case).pack(side="left", padx=5)
        
        self._show_dashboard()

    def _show_dashboard(self):
        # Stats
        self.stats_fr = tk.Frame(self.content_fr, bg=PAL["background"])
        self.stats_fr.pack(fill="x", pady=(0, 30))
        
        metrics = [("ACTIVE CASES", "24"), ("HEARINGS TODAY", "3"), ("PENDING DRAFTS", "12"), ("BILLABLES (MTD)", "₹2.4L")]
        for lbl, val in metrics:
            f = tk.Frame(self.stats_fr, bg=PAL["surface"], padx=20, pady=20)
            f.pack(side="left", expand=True, fill="both", padx=5)
            tk.Label(f, text=lbl, font=FONT["caption"], fg=PAL["text_secondary"], bg=PAL["surface"]).pack(anchor="w")
            tk.Label(f, text=val, font=FONT["h2"], fg=PAL["text_primary"], bg=PAL["surface"]).pack(anchor="w")

        # Hearing Schedule
        sched_fr = tk.Frame(self.content_fr, bg=PAL["surface"], padx=20, pady=20)
        sched_fr.pack(fill="both", expand=True)
        
        tk.Label(sched_fr, text="UPCOMING HEARINGS (COMPLIANCE SYNC)", font=FONT["h3"], fg=PAL["secondary"], bg=PAL["surface"]).pack(anchor="w", pady=(0, 15))
        
        cols = ("DATE", "CASE NAME", "COURT", "PURPOSE", "STATUS")
        self.hearing_tree = ttk.Treeview(sched_fr, columns=cols, show="headings", style="Custom.Treeview")
        for c in cols:
            self.hearing_tree.heading(c, text=c)
            self.hearing_tree.column(c, anchor="center")
            
        self.hearing_tree.pack(fill="both", expand=True)
        
    def _switch_view(self, view_tag: str):
        self.active_view = view_tag
        messagebox.showinfo("Legal Switch", f"Navigating to Sovereign {view_tag} Workspace...")

    def _mock_add_case(self):
        messagebox.showinfo("Case Filing", "Launching Sovereign Case Filing Wizard... Integrity Hashing Engaged.")

    def _refresh_data(self):
        data = [
            ("2026-03-15", "Union of India vs XYZ", "SC - Court 1", "Final Arguments", "CONFIRMED"),
            ("2026-03-16", "Sharma vs State of UP", "HC - Alld", "Charge Framing", "LISTED"),
            ("2026-03-18", "Asset Recovery #404", "DRT - Delhi", "Evidence", "ADJOURNED"),
        ]
        
        for item in self.hearing_tree.get_children():
            self.hearing_tree.delete(item)
            
        for entry in data:
            self.hearing_tree.insert("", "end", values=entry)

if __name__ == "__main__":
    app = AdvocateCommandCenter()
    app.mainloop()
