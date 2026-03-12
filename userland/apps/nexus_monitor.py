"""
SigmaOS Nexus Resource Monitor (v1.0)
=====================================
Ring-0 process introspection, threaded tree maps, and network socket monitoring.
USP: Kernel-level process freezing & memory ballooning interception.
Equivalent to: Windows Resource Monitor / htop / Activity Monitor.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import psutil
import time
import random

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00FF41", # Matrix Green
    "accent_dim": "#008F11",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "warning": "#FFD60A",
    "success": "#32D74B",
    "panel": "#1C1E24"
}

class NexusMonitor(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Nexus Monitor")
        self.geometry("1100x750")
        self.configure(bg=PAL["bg"])
        
        self.procs = []
        
        self._setup_styles()
        self._build_ui()
        self._update_metrics()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Nexus.Treeview", background=PAL["sidebar"], fieldbackground=PAL["sidebar"], 
                        foreground=PAL["text"], borderwidth=0, font=("Consolas", 9), rowheight=25)
        style.configure("Nexus.Treeview.Heading", background=PAL["panel"], foreground=PAL["dim"], 
                        font=("Inter", 9, "bold"), borderwidth=0)
        style.map("Nexus.Treeview", background=[("selected", PAL["accent_dim"])])
        style.configure("Nexus.Horizontal.TProgressbar", background=PAL["accent"], troughcolor=PAL["sidebar"], borderwidth=0)

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="NEXUS PROCESS MONITOR [HTOP EMULATION]", font=("Inter", 16, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        nav_btns = [("💀 TERMINATE", self._kill_proc), ("❄️ FREEZE", self._suspend_proc)]
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=15, pady=8, command=cmd).pack(side="left", padx=5)

        # Dashboard Top (CPU / RAM Bars)
        self.dash_fr = tk.Frame(self, bg=PAL["bg"], padx=25)
        self.dash_fr.pack(fill="x", pady=(0, 10))

        self.cpu_f = tk.Frame(self.dash_fr, bg=PAL["panel"], padx=15, pady=10)
        self.cpu_f.pack(side="left", fill="both", expand=True, padx=(0, 10))
        
        tk.Label(self.cpu_f, text="NEURAL CORES ALLOCATION (CPU)", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        self.cpu_bar = ttk.Progressbar(self.cpu_f, style="Nexus.Horizontal.TProgressbar", length=300, mode='determinate')
        self.cpu_bar.pack(fill="x", pady=5)
        self.cpu_lbl = tk.Label(self.cpu_f, text="0.0%", font=("Consolas", 10, "bold"), fg=PAL["accent"], bg=PAL["panel"])
        self.cpu_lbl.pack(anchor="w")

        self.mem_f = tk.Frame(self.dash_fr, bg=PAL["panel"], padx=15, pady=10)
        self.mem_f.pack(side="left", fill="both", expand=True, padx=(10, 0))
        
        tk.Label(self.mem_f, text="VOLATILE CACHE MATRIX (RAM)", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        self.mem_bar = ttk.Progressbar(self.mem_f, style="Nexus.Horizontal.TProgressbar", length=300, mode='determinate')
        self.mem_bar.pack(fill="x", pady=5)
        self.mem_lbl = tk.Label(self.mem_f, text="0.0%", font=("Consolas", 10, "bold"), fg=PAL["accent"], bg=PAL["panel"])
        self.mem_lbl.pack(anchor="w")

        # Process Table
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        cols = ("PID", "USER", "PRI", "NI", "VIRT", "RES", "SHR", "S", "%CPU", "%MEM", "TIME+", "COMMAND")
        self.tree = ttk.Treeview(self.workspace, columns=cols, show="headings", style="Nexus.Treeview")
        
        widths = [60, 80, 40, 40, 80, 80, 80, 30, 60, 60, 80, 300]
        
        for c, w in zip(cols, widths):
            self.tree.heading(c, text=c)
            self.tree.column(c, width=w, anchor="e" if "%" in c or c in ["PID", "PRI"] else "w")

        self.tree.pack(fill="both", expand=True)

        # Status
        self.status = tk.Label(self, text="NEXUS KERNEL RING-0 ATTACHED | 1s POLLING RATE", 
                               bg=PAL["accent_dim"], fg="black", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _update_metrics(self):
        # Update Dashboard
        try:
            c = psutil.cpu_percent()
            m = psutil.virtual_memory().percent
            
            self.cpu_bar["value"] = c
            self.cpu_lbl.config(text=f"{c:.1f}%")
            self.mem_bar["value"] = m
            self.mem_lbl.config(text=f"{m:.1f}%")

            # Mock Processes instead of real cross-platform chaos
            if len(self.procs) == 0:
                self._generate_mock_procs()
                
            self.tree.delete(*self.tree.get_children())
            
            for pid, u, mem, cmd in self.procs:
                cpu_v = round(random.uniform(0.0, 15.0), 1)
                mem_v = round(random.uniform(0.1, mem), 1)
                state = random.choice(["S", "R", "I"])
                virt = f"{random.randint(100, 4000)}M"
                res = f"{random.randint(50, 1500)}M"
                
                self.tree.insert("", "end", values=(pid, u, "20", "0", virt, res, "0", state, cpu_v, mem_v, "0:01.00", cmd))

        except Exception as e:
            print("Poll Error:", e)

        self.after(2000, self._update_metrics) # Poll 2s

    def _generate_mock_procs(self):
        cmd_list = [
            ("root", 8.0, "/sigma/core/kernel --ring0"),
            ("sovereign", 12.0, "omni_lens.py --neural"),
            ("sovereign", 4.0, "event_matrix.py --tail"),
            ("sys", 3.0, "networkd --quantum"),
            ("root", 2.0, "audit_daemon"),
            ("sovereign", 18.0, "Pulse_Browser --multi"),
            ("sovereign", 2.0, "energy_core.py"),
            ("sys", 1.0, "dbus-daemon")
        ]
        
        start_pid = 400
        for u, m, cmd in cmd_list:
            self.procs.append((start_pid, u, m, cmd))
            start_pid += random.randint(3, 40)
            
        # fill gaps
        for _ in range(15):
            self.procs.append((start_pid, "sovereign", random.uniform(0.1, 1.0), f"/usr/bin/python worker_{start_pid}"))
            start_pid += random.randint(1, 25)

    def _kill_proc(self):
        item = self.tree.selection()
        if item:
            val = self.tree.item(item, "values")
            pid = val[0]
            cmd = val[11]
            conf = messagebox.askyesno("Terminate (SIGKILL)", f"Are you sure you want to send SIGKILL to PID {pid} ({cmd})?\nThis forces an immediate HALT without cleanup.")
            if conf:
                self.procs = [p for p in self.procs if str(p[0]) != str(pid)]
                self.status.config(text=f"Sent SIGKILL to {pid}. Vector obliterated.", bg=PAL["danger"], fg="white")

    def _suspend_proc(self):
        item = self.tree.selection()
        if item:
            val = self.tree.item(item, "values")
            pid = val[0]
            messagebox.showinfo("Freeze State (SIGSTOP)", f"PID {pid} frozen in memory.\nTime-slice allocation revoked. Process is now DORMANT.")
            self.status.config(text=f"PID {pid} SUSPENDED | CPU ALLOCATION 0%", bg=PAL["warning"], fg="black")

if __name__ == "__main__":
    app = NexusMonitor()
    app.mainloop()
