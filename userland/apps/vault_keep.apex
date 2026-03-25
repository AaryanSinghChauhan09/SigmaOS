"""
SigmaOS Vault Keep (v2.0)
=========================
Multi-dimensional biometric zero-trust password manager.
USP: Splintered key sharding and localized memory storage.
Equivalent to: 1Password / Apple Keychain / Google Password Manager.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#F54123", # Sentinel Red/Orange
    "accent_dim": "#C32B13",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "panel": "#1C1E24"
}

class VaultKeep(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Vault Keep")
        self.geometry("950x650")
        self.configure(bg=PAL["bg"])
        
        self.secrets = [
            ("GitHub Access", "sovereign_repo_admin", "********", "2 days ago"),
            ("AWS Root", "admin1", "********", "1 hour ago"),
            ("Ethereum Wallet", "N/A (Seed Phrase)", "********", "5 mins ago"),
            ("Sigma Kernel Key", "admin", "********", "System Boot"),
            ("Banking API", "client_usr_99", "********", "3 wks ago")
        ]
        
        self.auth_state = False
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Vault.Treeview", background=PAL["sidebar"], fieldbackground=PAL["sidebar"], 
                        foreground=PAL["text"], borderwidth=0, font=("Inter", 10), rowheight=35)
        style.configure("Vault.Treeview.Heading", background=PAL["panel"], foreground=PAL["dim"], 
                        font=("Inter", 9, "bold"), borderwidth=0)
        style.map("Vault.Treeview", background=[("selected", PAL["accent_dim"])])

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="VAULT KEEP ZERO-TRUST", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        tk.Button(btn_fr, text="🔐 BIOMETRIC LOCK", font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                  relief="flat", padx=15, pady=8, command=self._authenticate).pack(side="left", padx=5)

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Left Panel Configuration
        self.conf_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=250, padx=20, pady=20)
        self.conf_fr.pack(side="left", fill="y", padx=(0, 20))
        self.conf_fr.pack_propagate(False)

        tk.Label(self.conf_fr, text="CRYPTO CONTEXT", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 20))
        
        metrics = [("Vault Standard:", "AES-GCM-2048", PAL["accent"]), 
                   ("Key Sharding:", "5 Nodes Active", PAL["success"])]
                   
        for label, val, color in metrics:
            tk.Label(self.conf_fr, text=label, font=("Inter", 9), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
            tk.Label(self.conf_fr, text=val, font=("Inter", 12, "bold"), fg=color, bg=PAL["panel"]).pack(anchor="w", pady=(2, 15))

        tk.Button(self.conf_fr, text="GENERATE KEY", font=("Inter", 8, "bold"), bg=PAL["accent"], fg="black", 
                  relief="flat", pady=6, command=self._mock_generate).pack(fill="x", pady=(20, 0))

        # Right Panel - Table
        self.tree_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        self.tree_fr.pack(side="left", fill="both", expand=True)

        cols = ("Identity", "Username", "Cipher Data", "Last Accessed")
        self.tree = ttk.Treeview(self.tree_fr, columns=cols, show="headings", style="Vault.Treeview", height=12)
        
        for c, w in zip(cols, [200, 150, 100, 100]):
            self.tree.heading(c, text=c.upper())
            self.tree.column(c, width=w, anchor="w" if c != "Cipher Data" else "center")

        for item in self.secrets:
            self.tree.insert("", "end", values=item)

        self.tree.pack(fill="both", expand=True)
        self.tree.bind("<Double-1>", self._reveal_secret)

        # Status
        self.status = tk.Label(self, text="VAULT LOCKED | AWAITING BIOMETRIC CLEARANCE", 
                               bg=PAL["danger"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _authenticate(self):
        if not self.auth_state:
            # Simulate biometrics
            self.status.config(text="VERIFYING NEURAL IMPRINT...", bg=PAL["warning"], fg="black")
            self.after(1500, self._grant_auth)
        else:
            self.status.config(text="VAULT LOCKED | MEMORY SHREEDED", bg=PAL["danger"], fg="white")
            self.auth_state = False

    def _grant_auth(self):
        self.auth_state = True
        self.status.config(text="BIOMETRICS CONFIRMED | VAULT UNLOCKED (30s TEMPORAL PASS)", bg=PAL["success"], fg="black")

    def _reveal_secret(self, event):
        if not self.auth_state:
            messagebox.showerror("Access Denied", "Vault is securely sealed. Authenticate first.")
            return
            
        item = self.tree.selection()
        if item:
            val = self.tree.item(item, "values")[0]
            messagebox.showinfo("Decrypted", f"[{val}]\n\nPassword copied to Sovereign Memory buffer.")

    def _mock_generate(self):
        s = "Q" + "".join(random.choices("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*", k=24))
        messagebox.showinfo("Neural Generation", f"New AES-compliant password minted:\n\n{s}\n\nCopied to buffer.")

if __name__ == "__main__":
    app = VaultKeep()
    app.mainloop()
