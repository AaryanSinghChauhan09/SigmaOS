import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD

class CustomizerPage(SigmaPage):
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "VISUAL CUSTOMIZER", "Enterprise OS Layer Configurator (Automation, Security, Theming)")
        self._build_content()
        
    def _build_content(self):
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        # Instantiate kernel modules dynamically if not present
        if not hasattr(self.kernel, 'linux_automation'):
            try:
                from linux_automation import SigmaAutomationLayer
                self.kernel.linux_automation = SigmaAutomationLayer(self.kernel)
            except Exception: pass
                
        if not hasattr(self.kernel, 'linux_security'):
            try:
                from linux_security_layer import SigmaSecurityLayer
                self.kernel.linux_security = SigmaSecurityLayer(self.kernel)
            except Exception: pass
                
        # 1. Automation Module
        auto_panel = self._card(main_panel, "Server Automation Layer")
        auto_panel.master.pack(side="top", fill="x", pady=(0,10))
        
        auto_opt = tk.Frame(auto_panel, bg=PAL["card"])
        auto_opt.pack(fill="x", pady=5)
        
        cron_var = tk.StringVar(value="0 3 * * *")
        tk.Label(auto_opt, text="Full System Backup CRON:", font=FONT_SMALL, fg="white", bg=PAL["card"]).pack(side="left", padx=(0,10))
        ttk.Entry(auto_opt, textvariable=cron_var, width=15).pack(side="left")
        
        def _set_backup():
            if hasattr(self.kernel, 'linux_automation'):
                res = self.kernel.linux_automation.schedule_backup("/mnt/sigma_sec_storage", cron_var.get(), 30)
                self._notify("Sigma Automation", res, "OK")
                
        ttk.Button(auto_opt, text="Commit CRON Job", command=_set_backup).pack(side="right", padx=10)
        
        # 2. Security Module
        sec_panel = self._card(main_panel, "SELinux & UFW Defense")
        sec_panel.master.pack(side="top", fill="x", pady=(0,10))
        
        sec_opt = tk.Frame(sec_panel, bg=PAL["card"])
        sec_opt.pack(fill="x", pady=5)
        
        ufw_status = tk.StringVar(value="UFW: Detected")
        sel_status = tk.StringVar(value="SELinux: Active")
        tk.Label(sec_opt, textvariable=ufw_status, font=FONT_BOLD, fg=PAL["red"], bg=PAL["card"]).pack(side="left", padx=10)
        tk.Label(sec_opt, textvariable=sel_status, font=FONT_BOLD, fg=PAL["cyan"], bg=PAL["card"]).pack(side="left", padx=10)
        
        # 3. Morphic Engine
        morphic_panel = self._card(main_panel, "Sovereign Morphic Engine")
        morphic_panel.master.pack(side="top", fill="x", pady=(0,10))
        
        m_fr = tk.Frame(morphic_panel, bg=PAL["card"])
        m_fr.pack(fill="x", pady=5)
        
        def _apply_morphic(vibe):
            cust = self.kernel.registry.get("customizer")
            if cust:
                cust.apply_morphic_preset(vibe)
                self._notify("MORPHIC SHIFT", f"Visual DNA morphed to {vibe}.", "OK")
                self._morphic_island(f"MORPHIC: {vibe.upper()} ACTIVE", PAL["cyan"])

        for v in ["Brutalist", "Glass", "Classic", "Aura"]:
            tk.Button(m_fr, text=v.upper(), font=FONT_SMALL, bg=PAL["bg2"], fg=PAL["text"],
                      padx=10, relief="flat", command=lambda x=v: _apply_morphic(x)).pack(side="left", padx=5)

        # 4. Identity & Appearance
        id_panel = self._card(main_panel, "Identity & Appearance Controls")
        id_panel.master.pack(side="top", fill="x", pady=(10,10))
        
        id_opt = tk.Frame(id_panel, bg=PAL["card"])
        id_opt.pack(fill="x", pady=5)
        
        tk.Label(id_opt, text="Dashboard Alias:", font=FONT_SMALL, fg="white", bg=PAL["card"]).pack(side="left", padx=(0,5))
        ttk.Entry(id_opt, textvariable=self.gui._dashboard_title, width=15).pack(side="left")
        
        ttk.Button(id_opt, text="Pick Accent Color", command=lambda: self.gui._pick_accent()).pack(side="right", padx=5)
