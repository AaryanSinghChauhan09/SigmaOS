import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_LOGO

class SovereignCommsPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Communication Engine", "Unified Sovereign Messaging — CRM/WhatsApp/AI Synthesis")
        self._build_ui()

    def _build_ui(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # Top Section: Integration Status
        stat_fr = tk.Frame(body, bg=PAL["bg2"])
        stat_fr.pack(fill="x", pady=(0, 10))
        integrations = [("WhatsApp", "CONNECTED", PAL["green"]), ("Salesforce", "SYNCING", PAL["gold"]), ("Zoho CRM", "READY", PAL["cyan"]), ("Zalo", "STANDBY", PAL["dim"])]
        for name, status, col in integrations:
            lbl = tk.Label(stat_fr, text=f" ● {name}: {status}", font=FONT_SMALL, fg=col, bg=PAL["bg2"], padx=10)
            lbl.pack(side="left")

        # Main Workspace
        panes = tk.Frame(body, bg=PAL["bg"])
        panes.pack(fill="both", expand=True)

        # Left: Unified Inbox (Mock)
        inbox_fr = self._card(panes, "Unified Sovereign Inbox")
        inbox_fr.master.pack(side="left", fill="both", expand=True, padx=(0, 10))
        
        msgs = [
            ("Zoho Lead", "New inquiry regarding Apex v4 license", "Just now"),
            ("WA #9876", "Bulk send request confirmed - 10k messages", "5m ago"),
            ("Salesforce", "Opportunity: 'Global Tech' moved to CLOSED", "1h ago"),
            ("Google Meet", "EmotionSense Pro: Participation Low (32%)", "2h ago"),
        ]
        for sender, snippet, time_str in msgs:
            m = tk.Frame(inbox_fr, bg=PAL["bg2"], pady=8, padx=10, highlightthickness=1, highlightbackground=PAL["bg3"])
            m.pack(fill="x", pady=2)
            tk.Label(m, text=sender, font=FONT_BOLD, fg=PAL["cyan"], bg=PAL["bg2"]).pack(anchor="w")
            tk.Label(m, text=snippet, font=FONT_SMALL, fg=PAL["text"], bg=PAL["bg2"]).pack(anchor="w")
            tk.Label(m, text=time_str, font=("Inter", 7), fg=PAL["dim"], bg=PAL["bg2"]).pack(anchor="e")

        # Right: Privacy & Utilities Hub
        util_fr = self._card(panes, "Privacy & Intelligence Hub")
        util_fr.master.pack(side="right", fill="both", width=380)
        util_fr.pack_propagate(False)

        # Temp Mail
        temp_mail_fr = tk.Frame(util_fr, bg=PAL["card"], pady=10)
        temp_mail_fr.pack(fill="x")
        tk.Label(temp_mail_fr, text="🛡️ Sovereign Temp Mail", font=FONT_BOLD, fg=PAL["accent"], bg=PAL["card"]).pack(anchor="w")
        addr_var = tk.StringVar(value="sigma_ghost_42@tempmail.info")
        ent = ttk.Entry(temp_mail_fr, textvariable=addr_var, font=("Consolas", 9))
        ent.pack(fill="x", pady=5)
        ttk.Button(temp_mail_fr, text="♻️ Generate New Identity", command=lambda: addr_var.set(f"ghost_{random.randint(100,999)}@tempmail.info")).pack(fill="x")
        
        # USP: WaitAIMinute
        wait_fr = tk.Frame(util_fr, bg=PAL["bg3"], pady=5, padx=5)
        wait_fr.pack(fill="x", pady=5)
        self._wait_guard = tk.BooleanVar(value=True)
        tk.Checkbutton(wait_fr, text="🛑 Wait Guard (WaitAIMinute USP)", variable=self._wait_guard, 
                       bg=PAL["bg3"], fg=PAL["gold"], selectcolor=PAL["bg2"],
                       command=lambda: self.gui._log_voice("Comms: Wait Guard active. 60s delay enforced on replies.")).pack(anchor="w")

        # AI Message Generator
        tk.Label(util_fr, text="✨ AI Message Generator (Multi-Platform)", font=FONT_BOLD, fg=PAL["text"], bg=PAL["card"]).pack(anchor="w", pady=(20, 5))
        platforms = ["WhatsApp Bulk", "CRM Lead Follow-up", "Discord Mass Delete", "LinkedIn Connection", "Mass SMS Sender"]
        for p in platforms:
            ttk.Button(util_fr, text=f"Generate {p} Payload", command=lambda pl=p: self.gui._log_voice(f"AI: Drafting {pl} payload based on context...")).pack(fill="x", pady=2)
        
        # USP: Global Messaging & Discord Pulse
        tk.Label(util_fr, text="🌍 Global reach Utilities", font=FONT_BOLD, fg=PAL["text"], bg=PAL["card"]).pack(anchor="w", pady=(20, 5))
        ttk.Button(util_fr, text="🚀 Global Mass Message (Multi-Region)", command=lambda: self.gui._log_voice("GlobalTech: Initiating mass broadcast across 12 GSM gateways...")).pack(fill="x")
        ttk.Button(util_fr, text="🧹 Discord Bulk Cleanup (Seerraze Clone)", command=lambda: self.gui._log_voice("Seerraze: Scanning 200+ DMs for PII/Spam removal...")).pack(fill="x", pady=5)

        # USP: GetHookd.AI - Save & Generate Ads
        tk.Label(util_fr, text="🎯 Marketing Accelerator (GetHookd USP)", font=FONT_BOLD, fg=PAL["text"], bg=PAL["card"]).pack(anchor="w", pady=(20, 5))
        ttk.Button(util_fr, text="💾 Save Ads from Libraries", command=lambda: self.gui._log_voice("GetHookd: Saving 12 competitive ads to Sovereign Vault...")).pack(fill="x")
        ttk.Button(util_fr, text="✨ Generate High-Converting Ads", command=lambda: self.gui._log_voice("GetHookd: Synthesizing 5 new ad variations based on trend analysis...")).pack(fill="x", pady=5)

        # Video/Meet Assistant Mock
        tk.Label(util_fr, text="🎞️ Media Synthesis", font=FONT_BOLD, fg=PAL["text"], bg=PAL["card"]).pack(anchor="w", pady=(20, 5))
        ttk.Button(util_fr, text="Generate Personalized AI Video (Potion Clone)", command=lambda: self.gui._log_voice("Synthesizing personalized AI video for 452 leads...")).pack(fill="x")
        ttk.Button(util_fr, text="EmotionSense Analysis (Google Meet)", command=lambda: self.gui._log_voice("Analyzing cognitive participation in active session...")).pack(fill="x", pady=5)

import random
