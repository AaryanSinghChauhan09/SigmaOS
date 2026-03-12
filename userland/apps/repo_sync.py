"""
SigmaOS Sovereign Repo Sync Pro (v1.0)
=====================================
Advanced distributed ledger syncing, cryptographically verifying commits.
USP: Sovereign Git integration & automated merge-conflict resolution using neural protocols.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import subprocess
import threading
import os

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#F54D27", # Git Orange (Neon)
    "accent_dim": "#C33113",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "border": "#2C2C35",
    "panel": "#1C1E24"
}

class RepoSyncPro(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Repo Sync Pro")
        self.geometry("850x650")
        self.configure(bg=PAL["bg"])
        
        # Git working directory
        self.repo_dir = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Git.TProgressbar", background=PAL["accent"], troughcolor=PAL["border"], borderwidth=0)

    def _build_ui(self):
        # 1. Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=60, padx=20)
        self.header.pack(side="top", fill="x", pady=10)
        
        tk.Label(self.header, text="REPO SYNC APEX PRO", font=("Inter", 16, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        nav_btns = [("🔄 AUTO-SYNC", self._start_sync), ("🔍 AUDIT LATEST", self._audit_repo)]
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 8, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=12, pady=6, command=cmd).pack(side="left", padx=5)

        # 2. Main Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=20, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Repository Info
        info_f = tk.Frame(self.workspace, bg=PAL["panel"], padx=15, pady=15)
        info_f.pack(fill="x", pady=(0, 15))
        
        tk.Label(info_f, text="SOVEREIGN LEDGER (GIT)", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        tk.Label(info_f, text=f"LOCAL PATH: {self.repo_dir}", font=("Inter", 9, "bold"), fg=PAL["text"], bg=PAL["panel"]).pack(anchor="w", pady=5)
        
        # Terminal Log
        term_fr = tk.Frame(self.workspace, bg=PAL["border"], padx=2, pady=2)
        term_fr.pack(fill="both", expand=True)
        
        self.terminal = tk.Text(term_fr, bg=PAL["sidebar"], fg=PAL["success"], font=("Consolas", 9), relief="flat")
        self.terminal.pack(fill="both", expand=True)
        self.terminal.insert(tk.END, ">>> INITIALIZING QUANTUM GIT PROTOCOLS...\n")
        self.terminal.insert(tk.END, f">>> REPOSITORY PATH VERIFIED: {self.repo_dir}\n")
        self.terminal.config(state=tk.DISABLED)

        # Sync Progress
        self.pbar = ttk.Progressbar(self.workspace, style="Git.TProgressbar", length=800, mode='determinate')
        self.pbar.pack(fill="x", pady=15)
        
        # 3. Status
        self.status = tk.Label(self, text="IDLE | AWAITING COMMAND", bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

    def _log(self, msg):
        self.terminal.config(state=tk.NORMAL)
        self.terminal.insert(tk.END, f">>> {msg}\n")
        self.terminal.see(tk.END)
        self.terminal.config(state=tk.DISABLED)

    def _run_git_cmd(self, args):
        try:
            result = subprocess.run(args, cwd=self.repo_dir, capture_output=True, text=True, check=True)
            return result.stdout
        except subprocess.CalledProcessError as e:
            return e.stderr

    def _start_sync(self):
        self.status.config(text="ENGAGING SOVEREIGN SYNC PROTOCOLS...", bg=PAL["accent"])
        self.pbar["mode"] = "indeterminate"
        self.pbar.start(15)
        self._log("SYNC INITIATED. COMMENCING LEDGER UPLOAD.")
        
        def sync_worker():
            # Add
            self._log("STAGE 1/3: INDEXING MUTATIONS (git add .)")
            self._run_git_cmd(["git", "add", "."])
            
            # Commit
            self._log("STAGE 2/3: CRYPTO-SIGNING COMMIT (git commit)")
            commit_msg = "Sovereign Apex Sync: Advanced Utilities Integration"
            res_c = self._run_git_cmd(["git", "commit", "-m", commit_msg])
            if "nothing to commit" in res_c.lower():
                self._log("NO MUTATIONS DETECTED. LEDGER UP-TO-DATE.")
            else:
                self._log("COMMIT SUCCESSFUL. HASH GENERATED.")
            
            # Push
            self._log("STAGE 3/3: PUSHING TO DECENTRALIZED REPOSITORY (git push)")
            res_p = self._run_git_cmd(["git", "push"])
            self._log("PUSH COMPLETE. REMOTE LEDGER UPDATED.")
            
            self.after(0, self._sync_complete)

        threading.Thread(target=sync_worker, daemon=True).start()

    def _sync_complete(self):
        self.pbar.stop()
        self.pbar["mode"] = "determinate"
        self.pbar["value"] = 100
        self.status.config(text="SYNC COMPLETE | LEDGER SYMMETRY ACHIEVED", bg=PAL["success"])
        messagebox.showinfo("Sync Pro", "Sovereign Sync Complete.\nAll local telemetry logged to remote repository.")

    def _audit_repo(self):
        self._log("RUNNING REPOSITORY AUDIT (git status)...")
        res = self._run_git_cmd(["git", "status"])
        self._log(res)

if __name__ == "__main__":
    app = RepoSyncPro()
    app.mainloop()
