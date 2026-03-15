"""
SigmaOS Sovereign Claw Page v1.0
=================================
USP: Personal Computer Automation Interface.
"""
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_TITLE, FONT_MED, FONT_MONO
from sigma_core.ai.sovereign_claw import SovereignClaw

class ClawPage(SigmaPage):
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "SOVEREIGN CLAW", "Action Intelligence & AI Automation")
        self.claw = SovereignClaw(gui.kernel)
        self._build_interface()

    def _build_interface(self):
        main = tk.Frame(self, bg=PAL["bg"], padx=40, pady=20)
        main.pack(fill="both", expand=True)

        # Split: Dashboard | Console
        left = tk.Frame(main, bg=PAL["bg"], width=400)
        left.pack(side="left", fill="both", expand=True)
        
        # Agent Branding
        card = self._card(left, "Claw Agent Status")
        card.master.pack(fill="x", pady=(0, 20))
        
        self.status_lbl = tk.Label(card, text="● STANDBY", font=FONT_MED, fg=PAL["green"], bg=PAL["card"])
        self.status_lbl.pack(pady=10)
        
        tk.Label(card, text="Current Goal: N/A", font=FONT_MONO, fg=PAL["dim"], bg=PAL["card"]).pack()

        # Prompt Box
        prompt_fr = self._card(left, "Personal Computer Intent")
        prompt_fr.master.pack(fill="x")
        
        self.prompt_ent = tk.Entry(prompt_fr, bg=PAL["bg2"], fg=PAL["text"], insertbackground=PAL["accent"], font=FONT_MED, bd=0)
        self.prompt_ent.pack(fill="x", pady=10, ipady=8)
        self.prompt_ent.bind("<Return>", lambda e: self._execute())
        
        tk.Button(prompt_fr, text="ENGAGE SOVEREIGN CLAW", bg=PAL["accent"], fg="white", font=FONT_BOLD, 
                  relief="flat", command=self._execute).pack(pady=10, fill="x")

        # Right: Logs
        right = tk.Frame(main, bg=PAL["bg"], padx=(20, 0))
        right.pack(side="right", fill="both", expand=True)
        
        tk.Label(right, text="ACTION TRACE", font=FONT_BOLD, fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w")
        self.log = scrolledtext.ScrolledText(right, bg=PAL["card"], fg=PAL["cyan"], font=FONT_MONO, bd=0)
        self.log.pack(fill="both", expand=True, pady=10)
        self.log.tag_config("user", foreground=PAL["accent"])
        self.log.tag_config("sys", foreground="white")

    def _execute(self):
        prompt = self.prompt_ent.get()
        if not prompt: return
        
        self.prompt_ent.delete(0, tk.END)
        self.log.insert(tk.END, f"\n> {prompt}\n", "user")
        self.status_lbl.config(text="● EXECUTING INTENT", fg=PAL["gold"])
        
        # Step-by-step simulation
        def _task():
            res = self.claw.execute_prompt(prompt)
            self.after(500, lambda: self.log.insert(tk.END, f"Claw: {res}\n", "sys"))
            self.after(550, lambda: self.status_lbl.config(text="● STANDBY", fg=PAL["green"]))
            self._notify("Claw Agent", "Intent execution sequence finalized.", "OK")
            
        import threading
        threading.Thread(target=_task, daemon=True).start()

    def _card(self, parent, title):
        """Helper to create a styled card."""
        fr = tk.Frame(parent, bg=PAL["card"], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL["border"])
        tk.Label(fr, text=title.upper(), font=FONT_TITLE, fg=PAL["accent"], bg=PAL["card"]).pack(anchor="w")
        return fr
