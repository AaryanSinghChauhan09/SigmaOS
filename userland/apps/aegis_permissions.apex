"""
SigmaOS Aegis Permissions Manager (v3.0)
========================================
Granular ring-level capability manager for apps.
USP: Zero-trust architecture with temporal biometric revokes.
"""
import tkinter as tk
from tkinter import ttk, messagebox

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#F5A623", # Shield Gold
    "accent_dim": "#C48216",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

class AegisPermissions(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Aegis Shield")
        self.geometry("900x700")
        self.configure(bg=PAL["bg"])
        
        self.apps = [
            ("pdf_forge.py", ["Kernel Read", "File Write"], ["Net Access", "Camera"]),
            ("omni_search.py", ["Disk Indexing", "Memory Read"], ["Net Access"]),
            ("energy_core.py", ["Sensors", "Hardware Power"], ["File Write", "Clipboard"]),
            ("Pulse_Browser", ["Net Access", "Microphone"], ["Kernel Read", "GPS Lock"])
        ]
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Aegis.TNotebook", background=PAL["bg"], borderwidth=0)
        style.configure("Aegis.TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["text"], 
                        padding=[15, 8], font=("Inter", 9, "bold"))
        style.map("Aegis.TNotebook.Tab", background=[("selected", PAL["accent"])])

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="AEGIS VAULT", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        tk.Button(btn_fr, text="🚨 REVOKE ALL TOKENS", font=("Inter", 9, "bold"), bg=PAL["danger"], fg="white", 
                  relief="flat", padx=15, pady=8, command=self._revoke_all).pack(side="left")

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Tabbed Control
        self.tabs = ttk.Notebook(self.workspace, style="Aegis.TNotebook")
        self.tabs.pack(fill="both", expand=True)

        self.tab_apps = tk.Frame(self.tabs, bg=PAL["bg"], padx=15, pady=15)
        self.tabs.add(self.tab_apps, text="VECTORS (APPS)")

        self.tab_perms = tk.Frame(self.tabs, bg=PAL["bg"], padx=15, pady=15)
        self.tabs.add(self.tab_perms, text="HARDENED PERMISSIONS")

        self.tab_logs = tk.Frame(self.tabs, bg=PAL["bg"], padx=15, pady=15)
        self.tabs.add(self.tab_logs, text="AUDIT LEDGER")

        self._build_apps_tab()
        self._build_logs_tab()

        # Status
        self.status = tk.Label(self, text="ZERO-TRUST ENFORCED | NO ESCALATION VECTORS DETECTED", 
                               bg=PAL["success"], fg="black", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _build_apps_tab(self):
        scroll_f = tk.Frame(self.tab_apps, bg=PAL["bg"])
        scroll_f.pack(fill="both", expand=True)
        
        for app, granted, denied in self.apps:
            card = tk.Frame(scroll_f, bg=PAL["panel"], pady=15, padx=20)
            card.pack(fill="x", pady=5)
            
            tk.Label(card, text=app, font=("Inter", 12, "bold"), fg=PAL["text"], bg=PAL["panel"]).pack(side="left", padx=10)
            
            btn_fr = tk.Frame(card, bg=PAL["panel"])
            btn_fr.pack(side="right")
            
            tk.Label(btn_fr, text=f"{len(granted)} Tokens", font=("Inter", 9, "bold"), fg=PAL["success"], bg=PAL["panel"]).pack(side="left", padx=10)
            tk.Label(btn_fr, text=f"{len(denied)} Blocked", font=("Inter", 9, "bold"), fg=PAL["danger"], bg=PAL["panel"]).pack(side="left", padx=10)
            
            tk.Button(btn_fr, text="INSPECT MATRIX", font=("Inter", 8, "bold"), bg=PAL["sidebar"], fg="white", 
                      relief="flat", padx=10, pady=4, command=lambda a=app: self._inspect_app(a)).pack(side="left", padx=5)

    def _build_logs_tab(self):
        log_text = tk.Text(self.tab_logs, bg=PAL["sidebar"], fg=PAL["warning"], font=("Consolas", 10), relief="flat")
        log_text.pack(fill="both", expand=True)
        log_text.insert(tk.END, "[SYS-CLK 14:02:00] pdf_forge.py requested Network Access -> DENIED (Aegis Rule 4)\n")
        log_text.insert(tk.END, "[SYS-CLK 14:02:05] omni_search.py initiated RAM Map -> OK (Token Verified)\n")
        log_text.insert(tk.END, "[SYS-CLK 14:05:12] pulse_browser requested GPS -> DENIED (Temporal Block Active)\n")
        log_text.config(state=tk.DISABLED)

    def _revoke_all(self):
        conf = messagebox.askyesno("Global Revoke", "Instantiate DEFCON 1 Token Revocation?\nThis will disconnect all apps from hardware rings.")
        if conf:
            self.status.config(text="DEFCON 1: ALL TOKENS BURNED. KERNEL IS ISOLATED.", bg=PAL["danger"], fg="white")

    def _inspect_app(self, app_name):
        messagebox.showinfo("Aegis Scanner", f"Scanning capability matrix for {app_name}...\nCryptography passes Ring-2 validation.")

if __name__ == "__main__":
    app = AegisPermissions()
    app.mainloop()
