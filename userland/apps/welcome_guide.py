"""
SigmaOS Sovereign Welcome & Setup Assistant (v1.0)
==================================================
The first-run experience for SigmaOS. 
Guides users through Zero-Trust concepts and UI navigation.
"""

import tkinter as tk
from tkinter import ttk
import time

PAL = {
    "bg": "#0A0B10",
    "card": "#141620",
    "accent": "#5E5CE6",
    "text": "#FFFFFF",
    "dim": "#8E8E93",
    "btn": "#2C2C3C"
}

class WelcomeAssistant(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Welcome to SigmaOS Sovereign")
        self.geometry("900x650")
        self.configure(bg=PAL["bg"])
        self.overrideredirect(True) # Borderless for premium feel
        self._center_window()
        
        self.step = -1
        self.content = [
            {
                "title": "Welcome to the Future of Privacy.",
                "desc": "SigmaOS is a Zero-Trust, Neuro-Native environment. \nYour data never leaves this machine without your explicit cryptographic consent.",
                "icon": "🛡️",
                "color": "#5AC8FA"
            },
            {
                "title": "Meet the AI Nexus.",
                "desc": "A dedicated Sovereign AI Agent (🧬) is available in your sidebar. \nIt guides you through the OS, automates tasks, and audits your security.",
                "icon": "🧬",
                "color": "#AF52DE"
            },
            {
                "title": "The Antigravity Fleet.",
                "desc": "Orchestrate 13+ AI platforms from a single hub (⚡). \nManage quotas, presets, and high-speed prompt distribution.",
                "icon": "⚡",
                "color": "#FFCC00"
            },
            {
                "title": "Sovereign App Store.",
                "desc": "Every app is an OCI-compliant isolated sandbox. \nExplore industry-standard tools for Dev, Media, and Security.",
                "icon": "📦",
                "color": "#32D74B"
            }
        ]
        
        self._setup_styles()
        self._build_ui()
        self._next()

    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use("clam")
        s.configure("Welcome.TProgressbar", thickness=8, troughcolor="#1A1A24", background=PAL["accent"])

    def _center_window(self):
        self.update_idletasks()
        w = self.winfo_width()
        h = self.winfo_height()
        extra_w = (self.winfo_screenwidth() - w) // 2
        extra_h = (self.winfo_screenheight() - h) // 2
        self.geometry(f"+{extra_w}+{extra_h}")

    def _build_ui(self):
        self.main_fr = tk.Frame(self, bg=PAL["bg"], padx=60, pady=50)
        self.main_fr.pack(fill="both", expand=True)
        
        self.icon_lbl = tk.Label(self.main_fr, text="🚀", font=("Segoe UI Symbol", 82), bg=PAL["bg"], fg=PAL["accent"])
        self.icon_lbl.pack(pady=(20, 10))
        
        self.title_lbl = tk.Label(self.main_fr, text="Initializing SigmaOS...", font=("Inter Bold", 26), fg="white", bg=PAL["bg"])
        self.title_lbl.pack()
        
        self.desc_lbl = tk.Label(self.main_fr, text="Setting up your Sovereign workspace environment...", 
                                 font=("Inter", 12), fg=PAL["dim"], bg=PAL["bg"], pady=30, wraplength=700)
        self.desc_lbl.pack()

        self.prog_var = tk.DoubleVar(value=0)
        self.prog = ttk.Progressbar(self.main_fr, style="Welcome.TProgressbar", variable=self.prog_var, length=500, mode='determinate')
        self.prog.pack(pady=20)
        
        self.btn_fr = tk.Frame(self, bg=PAL["bg"], pady=40)
        self.btn_fr.pack(side="bottom", fill="x", padx=60)
        
        self.skip_btn = tk.Button(self.btn_fr, text="SKIP SETUP", font=("Inter Bold", 8), bg=PAL["bg"], 
                                  fg=PAL["dim"], relief="flat", command=self.destroy)
        self.skip_btn.pack(side="left")
        
        self.next_btn = tk.Button(self.btn_fr, text="INITIALIZE ➔", font=("Inter Bold", 11), bg=PAL["accent"], 
                                  fg="white", relief="flat", padx=40, pady=12, command=self._next)
        self.next_btn.pack(side="right")

    def _next(self):
        self.step += 1
        if self.step < len(self.content):
            c = self.content[self.step]
            self.icon_lbl.config(text=c["icon"], fg=c["color"])
            self.title_lbl.config(text=c["title"])
            self.desc_lbl.config(text=c["desc"])
            
            # Update progress
            target = (self.step + 1) * (100 / len(self.content))
            self._animate_progress(target)
            
            if self.step == len(self.content) - 1:
                self.next_btn.config(text="ENTER SIGMAOS 🚀")
        else:
            self._finalize()

    def _animate_progress(self, target):
        curr = self.prog_var.get()
        if curr < target:
            self.prog_var.set(curr + 5)
            self.after(50, lambda: self._animate_progress(target))

    def _finalize(self):
        self.title_lbl.config(text="Sovereignty Established.", fg=PAL["accent"])
        self.desc_lbl.config(text="All systems operational. Zero-Trust policy enforced.\nWelcome home, Aaryan. Your workspace is ready.")
        self.icon_lbl.config(text="✓", fg="#32D74B")
        self.next_btn.config(state="disabled", text="ESTABLISHING...")
        self.update()
        
        # Simulated establishment delay
        self.after(2000, self.destroy)
        
    def _poll_establishment(self):
        # Additional logic for setup completion if needed
        pass

if __name__ == "__main__":
    app = WelcomeAssistant()
    app.mainloop()
