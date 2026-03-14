import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class VanguardPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Vanguard Security Hub", "Silo-Isolation & Zero-Persistence Engine")
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True)

        # 1. Active Silos (Left)
        l_fr = tk.Frame(body, bg=PAL["bg2"], width=450)
        l_fr.pack(side="left", fill="both", padx=5)
        l_fr.pack_propagate(False)

        inner = self.gui._card(l_fr, "App Isolation: Active Silos")
        inner.pack(fill="both", expand=True)
        
        # Simulated app list for isolation
        apps = ["Browser_Core", "Untrusted_Game", "Legacy_Win32", "P2P_Mesh_Node"]
        for app in apps:
             fr = tk.Frame(inner, bg=PAL["card"], pady=8)
             fr.pack(fill="x", pady=2)
             tk.Label(fr, text=f"📦 {app}", font=FONT_BOLD, fg=PAL["cyan"], bg=PAL["card"]).pack(side="left")
             tk.Label(fr, text=" Isolated", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(side="left", padx=5)
             ttk.Button(fr, text="Re-Silo", width=8).pack(side="right")

        # 2. Forensic Log (Right)
        r_fr = tk.Frame(body, bg=PAL["bg"], padx=10)
        r_fr.pack(side="right", fill="both", expand=True)
        
        audit_c = self.gui._card(r_fr, "Vanguard Audit Trail")
        audit_c.master.pack(fill="both", expand=True)
        self.v_log = self.gui._console(audit_c, height=25)
        self.v_log.pack(fill="both", expand=True)
        self.gui._log(self.v_log, "Vanguard Silo Engine ACTIVE. Pro-Persistence disabled.", "HEAD")
        
        # New USP: Neural Threat Lookup
        ttk.Button(r_fr, text="Query Global Threat DB (MeshIntel)", 
                   command=lambda: self.gui._log(self.v_log, "[VANGUARD] Requesting threat-hash verification from Mesh nodes...", "INFO")).pack(pady=10)
