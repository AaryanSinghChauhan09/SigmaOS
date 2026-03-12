"""
SigmaOS Zenith Focus Engine (v1.0)
==================================
Sensory deprivation protocols, neural attention lock, and notification suppression.
USP: Biometric-paced focus sessions with absolute process isolation.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import time
import threading

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#9D4EDD", # Deep Purple
    "accent_dim": "#5A189A",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

class ZenithFocusEngine(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Zenith Focus")
        self.geometry("900x550")
        self.configure(bg=PAL["bg"])
        
        self.time_left = 0
        self.running = False
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')

    def _build_ui(self):
        # 1. Premium Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="ZENITH OMNI-FOCUS", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        nav_btns = [("⚙️ PARADIGMS", self._config_paradigms), ("🛑 BREACH LOCK", self._breach_lock)]
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=15, pady=8, command=cmd).pack(side="left", padx=5)

        # 2. Main Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Center Clock Panel
        self.clock_fr = tk.Frame(self.workspace, bg=PAL["panel"], padx=20, pady=20)
        self.clock_fr.pack(expand=True, fill="both")
        
        tk.Label(self.clock_fr, text="NEURAL ATTENTION TIMER", font=("Inter", 12, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(pady=(20, 0))
        
        self.timer_lbl = tk.Label(self.clock_fr, text="25:00", font=("Inter", 80, "bold"), fg=PAL["text"], bg=PAL["panel"])
        self.timer_lbl.pack(expand=True)
        
        self.status_lbl = tk.Label(self.clock_fr, text="READY TO ISOLATE", font=("Inter", 12, "italic"), fg=PAL["accent"], bg=PAL["panel"])
        self.status_lbl.pack(pady=10)

        # Control Panel below clock
        self.ctrl_fr = tk.Frame(self.clock_fr, bg=PAL["panel"])
        self.ctrl_fr.pack(pady=(0, 20))

        tk.Button(self.ctrl_fr, text="ENGAGE DEEP FOCUS", font=("Inter", 12, "bold"), bg=PAL["accent"], fg="white", 
                  relief="flat", padx=30, pady=12, command=self._toggle_focus).pack(side="left")

        # 3. Status Bar
        self.status = tk.Label(self, text="NOTIFICATIONS: ACTIVE | NETWORK: OPEN", 
                               bg=PAL["sidebar"], fg=PAL["text"], font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _toggle_focus(self):
        if not self.running:
            self.running = True
            self.time_left = 25 * 60 # 25 minutes mock
            self.status.config(text="NOTIFICATIONS: SUPPRESSED | DISTRACTION APPS: TERMINATED", bg=PAL["danger"])
            self.status_lbl.config(text="SENSORY DEPRIVATION ACTIVE: DO NOT DISTURB")
            self.timer_lbl.config(fg=PAL["accent"])
            
            def countdown():
                while self.running and self.time_left > 0:
                    mins, secs = divmod(self.time_left, 60)
                    self.timer_lbl.config(text=f"{mins:02d}:{secs:02d}")
                    time.sleep(1)
                    self.time_left -= 1
                    
                if self.time_left <= 0:
                    self.running = False
                    self.timer_lbl.config(text="00:00", fg=PAL["success"])
                    self.status_lbl.config(text="ATTENTION CYCLE COMPLETE | RESTORED")
                    messagebox.showinfo("Zenith Focus", "Deep Focus Cycle Exhausted. Restoring telemetry.")
                    
            threading.Thread(target=countdown, daemon=True).start()
            
        else:
            self.running = False
            self.status.config(text="NOTIFICATIONS: ACTIVE | NETWORK: OPEN", bg=PAL["sidebar"])
            self.status_lbl.config(text="CYCLE ABANDONED")
            self.timer_lbl.config(text="25:00", fg=PAL["text"])

    def _config_paradigms(self):
        messagebox.showinfo("Paradigms", "Select isolation level:\n\n1. Minimal (Mute Alerts)\n2. Standard (Block Social)\n3. Absolute (Kill non-essential network)")

    def _breach_lock(self):
        if self.running:
            messagebox.showwarning("Breach Lock", "Unauthorized termination detected. Logging failure. Overriding...")
            self._toggle_focus()
        else:
            messagebox.showinfo("Lock", "No active focus vector to pierce.")

if __name__ == "__main__":
    app = ZenithFocusEngine()
    app.mainloop()
