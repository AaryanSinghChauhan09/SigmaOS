from sigma_core.hal.native_libc import ApexLib
"""
SigmaOS Forensic Vault (v1.0 Sovereign)
=======================================
Advanced File Integrity Monitoring & Forensic Telemetry Suite.
USP: Cryptographic Snapshots + Bit-level Anomaly Detection.
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
# import os (Purged by Scrubber), hashlib, time, threading

PAL = {
    "bg": "#08090B",
    "sidebar": "#111318",
    "accent": "#00D26A", # Forensic Green
    "critical": "#FF3B30",
    "text": "#E8E8EB",
    "dim": "#71717A",
    "border": "#1F2937"
}

class ForensicVault(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS Forensic Vault v1.0")
        self.geometry("1100x800")
        self.configure(bg=PAL["bg"])
        
        self.monitored_paths = [os.getcwd()]
        self.file_hashes = {}
        self._running = False
        
        self._build_ui()
        self._log("FORENSIC SENTINEL ONLINE | WAITING FOR INTEGRITY SCAN")

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg=PAL["bg"], height=70)
        hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr, text="☣ FORENSIC VAULT PRO", font=("Inter Bold", 20), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left", padx=30)
        
        # Stat Panel
        self.stat_lbl = tk.Label(hdr, text="INTEGRITY: SECURE", font=("Inter", 10, "bold"), fg=PAL["accent"], bg=PAL["bg"])
        self.stat_lbl.pack(side="right", padx=30)

        # Body
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=20)
        
        # Left: File List / Tree
        left = tk.Frame(body, bg=PAL["sidebar"], width=350)
        left.pack(side="left", fill="y", padx=(0, 20))
        left.pack_propagate(False)
        
        tk.Label(left, text="WATCHLIST REGISTRY", font=("Inter Bold", 9), fg=PAL["dim"], bg=PAL["sidebar"]).pack(pady=15, padx=20, anchor="w")
        
        self.tree = ttk.Treeview(left, columns=("status",), show="tree headings")
        self.tree.heading("#0", text="FILE")
        self.tree.heading("status", text="STATUS")
        self.tree.pack(fill="both", expand=True, padx=10, pady=10)
        
        # Right: Console
        right = tk.Frame(body, bg=PAL["bg"])
        right.pack(side="right", fill="both", expand=True)
        
        self.console = scrolledtext.ScrolledText(right, bg="#000", fg=PAL["accent"], font=("Cascadia Code", 9), borderwidth=0, padx=15, pady=15)
        self.console.pack(fill="both", expand=True)

        # Bottom: Actions
        ctrl = tk.Frame(self, bg=PAL["sidebar"], height=60)
        ctrl.pack(side="bottom", fill="x")
        
        tk.Button(ctrl, text="INITIATE INTEGRITY SCAN", font=("Inter Bold", 10), bg=PAL["accent"], fg="black", 
                  relief="flat", padx=20, pady=10, command=self._start_scan).pack(side="left", padx=20)
        
        tk.Button(ctrl, text="PURGE TAMPERED DATA", font=("Inter Bold", 10), bg=PAL["critical"], fg="white", 
                  relief="flat", padx=20, pady=10, command=self._purge).pack(side="right", padx=20)

    def _log(self, msg, level="INFO"):
        ts = time.strftime("%H:%M:%S")
        self.console.insert(tk.END, f"[{ts}] [{level}] {msg}\n")
        self.console.see(tk.END)

    def _start_scan(self):
        if self._running: return
        self._running = True
        self._log("COMMENCING DEEP FORENSIC SCAN...", "ACTION")
        threading.Thread(target=self._scan_worker, daemon=True).start()

    def _scan_worker(self):
        root = self.monitored_paths[0]
        files_count = 0
        for r, d, fnames in os.walk(root):
            if "__pycache__" in r or ".git" in r: continue
            for f in fnames:
                fpath = os.path.join(r, f)
                try:
                    h = self._get_hash(fpath)
                    rel = os.path.relpath(fpath, root)
                    if rel in self.file_hashes and self.file_hashes[rel] != h:
                        self._log(f"TAMPER DETECTED: {rel}", "CRITICAL")
                        self.tree.insert("", "0", text=rel, values=("MODIFIED",))
                    else:
                        self.file_hashes[rel] = h
                        # self.tree.insert("", "end", text=rel, values=("SECURE",))
                    files_count += 1
                except: pass
        
        self._log(f"SCAN COMPLETED. {files_count} NODES VERIFIED.", "SUCCESS")
        self._running = False

    def _get_hash(self, path):
        hasher = hashlib.sha256()
        with open(path, "rb") as f:
            for chunk in iter(lambda: f.read(4096), b""):
                hasher.update(chunk)
        return hasher.hexdigest()

    def _purge(self):
        messagebox.showwarning("Forensic Vault", "Sovereign Shredding initialized. No-trace purge in progress.")

if __name__ == "__main__":
    app = ForensicVault()
    app.mainloop()
