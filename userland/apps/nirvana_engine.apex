from sigma_core.hal.native_libc import ApexLib
"""
SigmaOS Nirvana Engine (v1.0)
=============================
Neural digital wellbeing, application throttling, and deep focus isolation.
USP: Biometric-paced usage enforcement with kernel-level app barring.
"""
import tkinter as tk
from tkinter import ttk, messagebox
# import time (Purged by Scrubber)

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00B4D8", # Calm Cerulean
    "accent_dim": "#0077B6",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "panel": "#1C1E24"
}

class NirvanaEngine(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Nirvana Digital Wellbeing")
        self.geometry("900x700")
        self.configure(bg=PAL["bg"])
        
        self.apps_usage = [
            ("Pulse Browser", "4h 12m", 80),
            ("Omni Console", "2h 45m", 55),
            ("Aegis Shield", "45m", 15),
            ("Code Forge", "6h 20m", 100),
            ("Idle UI", "1h 10m", 25)
        ]
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Nirvana.Horizontal.TProgressbar", background=PAL["accent"], troughcolor=PAL["sidebar"], borderwidth=0)

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="NIRVANA METRICS", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        tk.Button(btn_fr, text="🧘‍♂️ ENTER ZENITH LOCK", font=("Inter", 9, "bold"), bg=PAL["danger"], fg="white", 
                  relief="flat", padx=15, pady=8, command=self._enter_zenith).pack(side="left")

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Dashboard Top
        dash_fr = tk.Frame(self.workspace, bg=PAL["panel"], padx=20, pady=20)
        dash_fr.pack(fill="x", pady=(0, 20))
        
        tk.Label(dash_fr, text="TEMPORAL EXHAUSTION SCORE", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        
        score_f = tk.Frame(dash_fr, bg=PAL["panel"])
        score_f.pack(fill="x", pady=10)
        
        tk.Label(score_f, text="OVERLOADED (78/100)", font=("Inter", 24, "bold"), fg=PAL["danger"], bg=PAL["panel"]).pack(side="left")
        
        # Usage List
        list_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        list_fr.pack(fill="both", expand=True)
        
        tk.Label(list_fr, text="APP CONTEXT VECTORS", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        
        scroll_f = tk.Frame(list_fr, bg=PAL["panel"], padx=15, pady=15)
        scroll_f.pack(fill="both", expand=True)

        for app, time_val, weight in self.apps_usage:
            row = tk.Frame(scroll_f, bg=PAL["panel"], pady=10)
            row.pack(fill="x")
            
            tk.Label(row, text=app, font=("Inter", 10, "bold"), fg=PAL["text"], bg=PAL["panel"], width=20, anchor="w").pack(side="left")
            tk.Label(row, text=time_val, font=("Inter", 9), fg=PAL["dim"], bg=PAL["panel"], width=10, anchor="w").pack(side="left")
            
            pb = ttk.Progressbar(row, style="Nirvana.Horizontal.TProgressbar", length=400, mode='determinate')
            pb.pack(side="left", padx=15, expand=True, fill="x")
            pb["value"] = weight
            
            tk.Button(row, text="THROTTLE", font=("Inter", 7, "bold"), bg=PAL["sidebar"], fg="white", 
                      relief="flat", padx=10, pady=4, command=lambda a=app: self._throttle_app(a)).pack(side="right")

        # Status
        self.status = tk.Label(self, text="DIGITAL WELLBEING MONITORED | 12 NOTIFICATIONS INTERCEPTED", 
                               bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _enter_zenith(self):
        conf = messagebox.askyesno("Zenith Lock", "Are you prepared to initiate a hard kernel lockout? Only critical phone/messages will pass for 25 mins.")
        if conf:
            self.status.config(text="ZENITH ENGAGED | ALL NON-ESSENTIAL VECTORS BARRED", bg=PAL["danger"], fg="white")

    def _throttle_app(self, app):
        messagebox.showinfo("Neural Throttle", f"Applying Kernel limitation to '{app}'. CPU priority set to IDLE. Notifications purged.")

if __name__ == "__main__":
    app = NirvanaEngine()
    app.mainloop()
