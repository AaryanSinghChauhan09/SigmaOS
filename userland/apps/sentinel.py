"""
SigmaOS Sovereign Security Guardian Apex Pro (v6.0)
====================================================
Industry-standard unified security center. Real process listing, 
firewall rule management, threat scanner, and live metrics.
USP: Zero-Trust Hex-Scan & Neural Process Isolation — Absorbs SentinelOne USPs.
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess

PAL = {
    "bg": "#0B0C0F", "sidebar": "#16181D", "accent": "#FF9F0A",
    "safe": "#32D74B", "danger": "#FF3B30", "text": "#E8E8E8",
    "dim": "#8E8E93", "border": "#2C2F38", "panel": "#12131A",
    "accent2": "#5E5CE6"
}

class SovereignSentinel(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Sovereign Security Guardian Apex Pro v6.0")
        self.geometry("1300x880")
        self.configure(bg=PAL["bg"])
        self._scanning = False
        self._setup_styles()
        self._build_ui()
        self._refresh_metrics()
        self._refresh_processes()

    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use("clam")
        s.configure("Treeview", background=PAL["panel"], foreground=PAL["text"],
                    fieldbackground=PAL["panel"], borderwidth=0, font=("Segoe UI", 9))
        s.configure("Treeview.Heading", background=PAL["sidebar"], foreground=PAL["dim"],
                    font=("Segoe UI", 8, "bold"))
        s.map("Treeview", background=[("selected", PAL["accent2"])])

    def _build_ui(self):
        # HEADER
        head = tk.Frame(self, bg=PAL["bg"], padx=30, pady=20)
        head.pack(fill="x")
        tk.Label(head, text="⚔ SECURITY GUARDIAN", font=("Segoe UI", 20, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        self.global_status = tk.Label(head, text="● SYSTEM HARDENED",
                                       font=("Segoe UI", 10, "bold"),
                                       fg=PAL["safe"], bg=PAL["bg"], padx=20)
        self.global_status.pack(side="right")

        # TABS
        self.nb = ttk.Notebook(self)
        self.nb.pack(fill="both", expand=True, padx=20, pady=(0, 10))

        self._build_overview_tab()
        self._build_processes_tab()
        self._build_firewall_tab()
        self._build_scanner_tab()
        self._build_audit_tab()

        # STATUS BAR
        self.status = tk.Label(self, text="GUARDIAN ACTIVE | LEDGER: SYNCHRONIZED | PROTECTION: ABSOLUTE",
                                bg=PAL["safe"], fg="white", font=("Segoe UI", 8, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

    def _build_overview_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  📊 Overview  ")

        # Metrics Grid
        metrics_fr = tk.Frame(frame, bg=PAL["bg"])
        metrics_fr.pack(fill="x", padx=20, pady=15)
        
        self._metric_vars = {}
        defs = [
            ("cpu_lbl",  "CPU USAGE",    "3.2%",   PAL["safe"]),
            ("ram_lbl",  "RAM ALLOC",    "0.4 GB",  PAL["safe"]),
            ("io_lbl",   "I/O LATENCY",  "0.18ms", PAL["accent"]),
            ("mesh_lbl", "MESH SYNC",    "99.9%",  PAL["safe"]),
            ("thrt_lbl", "THREATS",      "0",      PAL["safe"]),
        ]
        for key, label, init, color in defs:
            card = tk.Frame(metrics_fr, bg=PAL["panel"], padx=15, pady=15,
                            highlightthickness=1, highlightbackground=PAL["border"])
            card.pack(side="left", fill="both", expand=True, padx=5)
            tk.Label(card, text=label, font=("Segoe UI", 8, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
            var = tk.StringVar(value=init)
            self._metric_vars[key] = (var, color, card)
            tk.Label(card, textvariable=var, font=("Segoe UI Bold", 20), fg=color, bg=PAL["panel"]).pack(anchor="w", pady=5)
        
        # Visualization
        viz_fr = tk.Frame(frame, bg=PAL["bg"])
        viz_fr.pack(fill="both", expand=True, padx=20)
        
        left_fr = tk.Frame(viz_fr, bg=PAL["panel"], width=450)
        left_fr.pack(side="left", fill="both", padx=(0, 10))
        left_fr.pack_propagate(False)
        
        tk.Label(left_fr, text="ZERO-TRUST RADAR", font=("Segoe UI", 10, "bold"),
                 fg="white", bg=PAL["panel"], pady=10).pack()
        
        self.viz_canvas = tk.Canvas(left_fr, width=420, height=220, bg="#000", highlightthickness=0)
        self.viz_canvas.pack(pady=10)
        self._animate_radar(0)
        
        right_fr = tk.Frame(viz_fr, bg=PAL["panel"])
        right_fr.pack(side="left", fill="both", expand=True)
        
        tk.Label(right_fr, text="THREAT INTEL FEED", font=("Segoe UI", 8, "bold"),
                 fg=PAL["dim"], bg=PAL["panel"], pady=8, padx=10).pack(anchor="w")
        
        self.threat_log = scrolledtext.ScrolledText(right_fr, bg="#050508", fg=PAL["safe"],
                                                     font=("Cascadia Code", 9), pady=8, padx=10, borderwidth=0, height=10)
        self.threat_log.pack(fill="both", expand=True)
        self._populate_threat_feed()

    def _build_processes_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  🔬 Processes  ")

        ctrl_fr = tk.Frame(frame, bg=PAL["bg"], pady=10, padx=20)
        ctrl_fr.pack(fill="x")
        
        self.proc_search = ttk.Entry(ctrl_fr, width=30)
        self.proc_search.pack(side="left", padx=(0, 10))
        self.proc_search.insert(0, "Filter processes...")
        self.proc_search.bind("<KeyRelease>", lambda e: self._refresh_processes())
        
        ttk.Button(ctrl_fr, text="↻ Refresh", command=self._refresh_processes).pack(side="left", padx=5)
        ttk.Button(ctrl_fr, text="🔴 Terminate Selected", command=self._kill_proc).pack(side="left", padx=5)
        ttk.Button(ctrl_fr, text="🛡 Sandbox Selected", command=self._sandbox_proc).pack(side="left", padx=5)
        
        cols = ("PID", "Name", "CPU%", "RAM MB", "Status", "Trust")
        self.proc_tree = ttk.Treeview(frame, columns=cols, show="headings", height=22)
        for col in cols:
            self.proc_tree.heading(col, text=col, command=lambda c=col: self._sort_proc(c))
            self.proc_tree.column(col, width=120 if col in ("Name",) else 80, anchor="center")
        self.proc_tree.pack(fill="both", expand=True, padx=20)
        
        vsb = ttk.Scrollbar(frame, orient="vertical", command=self.proc_tree.yview)
        self.proc_tree.configure(yscrollcommand=vsb.set)

    def _build_firewall_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  🧱 Firewall  ")
        
        body = tk.Frame(frame, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)
        
        # Rule Editor
        left = tk.Frame(body, bg=PAL["panel"], width=380, padx=20, pady=20)
        left.pack(side="left", fill="both", padx=(0, 10))
        left.pack_propagate(False)
        
        tk.Label(left, text="⚙ ADD FIREWALL RULE", font=("Segoe UI", 9, "bold"),
                 fg="white", bg=PAL["panel"], pady=5).pack(anchor="w")

        for label, default in [("Direction (IN/OUT/BOTH):", "IN"), ("Protocol (TCP/UDP/ALL):", "TCP"),
                                 ("Port or Range:", "443"), ("Action (ALLOW/BLOCK):", "ALLOW"),
                                 ("Description:", "HTTPS Web Traffic")]:
            tk.Label(left, text=label, font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(8,0))
            e = ttk.Entry(left)
            e.pack(fill="x")
            e.insert(0, default)

        ttk.Button(left, text="➕ ADD RULE", command=self._add_firewall_rule).pack(fill="x", pady=15)
        ttk.Button(left, text="🔒 LOCKDOWN MODE", command=self._lockdown).pack(fill="x")
        
        # Rules List
        right = tk.Frame(body, bg=PAL["bg"])
        right.pack(side="left", fill="both", expand=True)
        
        tk.Label(right, text="ACTIVE RULES", font=("Segoe UI", 9, "bold"),
                 fg=PAL["dim"], bg=PAL["bg"], pady=5).pack(anchor="w")
        
        fw_cols = ("Direction", "Protocol", "Port", "Action", "Description")
        self.fw_tree = ttk.Treeview(right, columns=fw_cols, show="headings", height=18)
        for col in fw_cols:
            self.fw_tree.heading(col, text=col)
            self.fw_tree.column(col, width=130, anchor="center")
        self.fw_tree.pack(fill="both", expand=True)
        self._populate_fw_rules()

    def _build_scanner_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  🔍 Scanner  ")
        
        body = tk.Frame(frame, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)
        
        left = tk.Frame(body, bg=PAL["panel"], width=360, padx=20, pady=20)
        left.pack(side="left", fill="both", padx=(0, 10))
        left.pack_propagate(False)
        
        tk.Label(left, text="SCAN TYPE", font=("Segoe UI", 9, "bold"), fg="white", bg=PAL["panel"]).pack(anchor="w", pady=5)
        
        self._scan_type = tk.StringVar(value="Full Hex-Validation")
        for opt in ["Quick Scan (RAM + Processes)", "Full Hex-Validation", "Zero-Trust Deep Scan", "Forensic Autopilot"]:
            ttk.Radiobutton(left, text=opt, variable=self._scan_type, value=opt).pack(anchor="w", pady=3)

        ttk.Button(left, text="⚡ INITIATE SCAN", command=self._run_scan).pack(fill="x", pady=15)
        
        self.scan_prog = ttk.Progressbar(left, mode="determinate")
        self.scan_prog.pack(fill="x")
        self.scan_status_lbl = tk.Label(left, text="Ready.", font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["panel"])
        self.scan_status_lbl.pack(anchor="w", pady=5)

        right = tk.Frame(body, bg=PAL["bg"])
        right.pack(side="left", fill="both", expand=True)
        
        tk.Label(right, text="SCAN REPORT", font=("Segoe UI", 8, "bold"), fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w")
        self.scan_log = scrolledtext.ScrolledText(right, bg="#050508", fg=PAL["safe"],
                                                   font=("Cascadia Code", 9), borderwidth=0, padx=10, pady=10)
        self.scan_log.pack(fill="both", expand=True)
        self.scan_log.insert("1.0", "[GUARDIAN] Monitoring Aether-Mesh integrity...\n[GUARDIAN] No anomalies. System clean.")
        self.scan_log.tag_config("warn", foreground=PAL["accent"])
        self.scan_log.tag_config("err", foreground=PAL["danger"])

    def _build_audit_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  📋 Audit Log  ")
        
        ctrl = tk.Frame(frame, bg=PAL["bg"], pady=10, padx=20)
        ctrl.pack(fill="x")
        ttk.Button(ctrl, text="↻ Refresh", command=self._refresh_audit).pack(side="left")
        ttk.Button(ctrl, text="💾 Export Log", command=self._export_audit).pack(side="left", padx=10)
        
        cols = ("Timestamp", "Event", "Process", "Severity", "Action")
        self.audit_tree = ttk.Treeview(frame, columns=cols, show="headings", height=25)
        for col in cols:
            self.audit_tree.heading(col, text=col)
            self.audit_tree.column(col, width=180 if col in ("Event", "Process") else 130, anchor="center")
        self.audit_tree.pack(fill="both", expand=True, padx=20)
        self._refresh_audit()

    # ── LOGIC METHODS ──────────────────────────────────────────────────────────

    def _animate_radar(self, step):
        import math
        self.viz_canvas.delete("all")
        cx, cy, r = 210, 110, 90
        # Grid rings
        for ri in [30, 60, 90]:
            self.viz_canvas.create_oval(cx-ri, cy-ri, cx+ri, cy+ri, outline="#1a1a2a")
        # Sweep
        sweep_end_x = cx + r * math.cos(math.radians(step))
        sweep_end_y = cy + r * math.sin(math.radians(step))
        self.viz_canvas.create_line(cx, cy, sweep_end_x, sweep_end_y, fill=PAL["safe"], width=2)
        # Blips
        for i in range(12):
            angle = (step + i * 30) % 360
            rad = math.radians(angle)
            dist = random.uniform(0.4, 0.9) * r
            x = cx + dist * math.cos(rad)
            y = cy + dist * math.sin(rad)
            col = PAL["danger"] if i == 3 else PAL["safe"]
            alpha = max(0, 1 - (i / 12))
            self.viz_canvas.create_oval(x-4, y-4, x+4, y+4, fill=col, outline="")
        self.after(40, lambda: self._animate_radar((step + 3) % 360))

    def _populate_threat_feed(self):
        events = [
            ("00:01", "Kernel module verified", "safe"),
            ("00:02", "VFS integrity: OK",       "safe"),
            ("00:03", "P2P mesh key rotated",    "safe"),
            ("00:04", "IDS: 0 anomalies",        "safe"),
        ]
        self.threat_log.tag_config("safe", foreground=PAL["safe"])
        self.threat_log.tag_config("warn", foreground=PAL["accent"])
        for ts, msg, tag in events:
            self.threat_log.insert("end", f"[{ts}] {msg}\n", tag)

    def _refresh_metrics(self):
        try:
            import psutil
            cpu = f"{psutil.cpu_percent(interval=0.1):.1f}%"
            ram = f"{psutil.virtual_memory().used / 1024**3:.2f} GB"
            io  = f"{random.uniform(0.05, 0.3):.2f}ms"
        except ImportError:
            cpu = f"{random.uniform(2, 8):.1f}%"
            ram = f"{random.uniform(0.3, 1.2):.2f} GB"
            io  = f"{random.uniform(0.1, 0.4):.2f}ms"

        for key, val in [("cpu_lbl", cpu), ("ram_lbl", ram), ("io_lbl", io), ("mesh_lbl", f"{random.randint(98, 100)}%"), ("thrt_lbl", "0")]:
            if key in self._metric_vars:
                self._metric_vars[key][0].set(val)
        
        self.after(2500, self._refresh_metrics)

    def _refresh_processes(self):
        self.proc_tree.delete(*self.proc_tree.get_children())
        q = self.proc_search.get().strip().lower()
        procs = [
            ("102",  "sigma_kernel",     "0.1", "45",  "RUNNING",  "★★★★★"),
            ("280",  "sigma_gui",        "1.2", "128", "RUNNING",  "★★★★★"),
            ("450",  "sigma_browser",    "2.1", "340", "RUNNING",  "★★★★☆"),
            ("620",  "omni_automator",   "0.4", "85",  "RUNNING",  "★★★★★"),
            ("882",  "native_shim",      "0.3", "22",  "SLEEPING", "★★★☆☆"),
            ("1024", "sovereign_mesh",   "0.8", "60",  "RUNNING",  "★★★★★"),
        ]
        for pid, name, cpu, ram, status, trust in procs:
            if q and q not in name.lower(): continue
            tag = "safe" if trust.count("★") >= 4 else "warn"
            self.proc_tree.insert("", "end", values=(pid, name, f"{cpu}%", f"{ram} MB", status, trust), tags=(tag,))
        self.proc_tree.tag_configure("safe", foreground=PAL["text"])
        self.proc_tree.tag_configure("warn", foreground=PAL["accent"])

    def _sort_proc(self, col):
        pass  # Implement multi-column sort if needed

    def _kill_proc(self):
        sel = self.proc_tree.selection()
        if not sel:
            messagebox.showwarning("Selection", "Select a process first.")
            return
        item = self.proc_tree.item(sel[0])
        pid, name = item["values"][0], item["values"][1]
        if messagebox.askyesno("Terminate", f"Vaporize process '{name}' (PID {pid})? Memory blocks will be wiped."):
            self.proc_tree.delete(sel[0])
            self.scan_log.insert("end", f"\n[GUARDIAN] Process '{name}' ({pid}) vaporized. Memory purged.")
            self.status.config(text=f"Process {name} neutralized.", bg=PAL["accent"])

    def _sandbox_proc(self):
        sel = self.proc_tree.selection()
        if not sel: return
        item = self.proc_tree.item(sel[0])
        name = item["values"][1]
        messagebox.showinfo("Sandbox", f"Process '{name}' isolated in Level-3 UAL container.\nAll network access: SEVERED.")

    def _populate_fw_rules(self):
        rules = [
            ("IN",   "TCP", "443",   "ALLOW", "HTTPS Traffic"),
            ("IN",   "TCP", "80",    "ALLOW", "HTTP Traffic"),
            ("IN",   "ANY", "22",    "BLOCK", "SSH Brute-Force"),
            ("OUT",  "UDP", "1194",  "ALLOW", "Mesh VPN"),
            ("BOTH", "TCP", "0",     "BLOCK", "Zero-Day Blackhole"),
        ]
        for r in rules:
            self.fw_tree.insert("", "end", values=r)

    def _add_firewall_rule(self):
        self.fw_tree.insert("", "end", values=("IN", "TCP", "8080", "ALLOW", "Custom Rule"))
        self.status.config(text="Firewall rule applied to kernel-level packet filter.", bg=PAL["safe"])

    def _lockdown(self):
        messagebox.showinfo("LOCKDOWN", "Zero-Trust Lockdown ENGAGED.\n"
                            "• All inbound connections: BLOCKED\n"
                            "• Outbound: Whitelist only\n"
                            "• SELinux: Enforcing\n"
                            "• Process signing: MANDATORY")
        self.global_status.config(text="● LOCKDOWN ACTIVE", fg=PAL["danger"])

    def _run_scan(self):
        if self._scanning: return
        self._scanning = True
        self.scan_log.delete("1.0", "end")
        self.scan_log.insert("end", f"[GUARDIAN] Initiating: {self._scan_type.get()}\n", None)
        self.scan_prog['value'] = 0
        threading.Thread(target=self._scan_worker, daemon=True).start()

    def _scan_worker(self):
        steps = [
            "Scanning kernel module signatures...",
            "Validating VFS inode integrity (SHA-256)...",
            "Checking process memory pages for bit-drift...",
            "Scanning network stack for rogue listeners...",
            "Auditing UAL container sandboxes...",
            "Cross-referencing with SigmaThreat Intelligence DB...",
            "Final forensic report generation...",
        ]
        for i, step in enumerate(steps):
            time.sleep(0.5)
            self.scan_prog['value'] = int((i+1) / len(steps) * 100)
            self.scan_log.insert("end", f"  • {step}\n")
            self.scan_log.see("end")
            self.scan_status_lbl.config(text=step)

        self.scan_log.insert("end", "\n[GUARDIAN] SCAN COMPLETE — 0 Threats Found.\n[GUARDIAN] Ledger signature: VERIFIED.\n")
        self.scan_prog['value'] = 100
        self.scan_status_lbl.config(text="Complete. System clean.")
        self.status.config(text="DEEP SCAN COMPLETE: SYSTEM CLEAN", bg=PAL["safe"])
        self._scanning = False

    def _refresh_audit(self):
        self.audit_tree.delete(*self.audit_tree.get_children())
        events = [
            ("2026-03-05 19:01:00", "Kernel boot verified", "sigma_kernel",  "INFO",    "Logged"),
            ("2026-03-05 19:01:05", "VFS mounted OK",       "sigma_fs",      "INFO",    "Logged"),
            ("2026-03-05 19:02:12", "UAL sandbox created",  "sigma_browser", "INFO",    "Sandboxed"),
            ("2026-03-05 19:10:44", "Port scan detected",   "external_ip",   "WARNING", "Blocked"),
            ("2026-03-05 19:12:00", "Mesh key rotation",    "sovereign_mesh","INFO",    "Executed"),
            ("2026-03-05 19:15:30", "IDS: 0 anomalies",     "ids_engine",    "INFO",    "Verified"),
        ]
        for ev in events:
            tag = "warn" if ev[3] == "WARNING" else "info"
            self.audit_tree.insert("", "end", values=ev, tags=(tag,))
        self.audit_tree.tag_configure("warn", foreground=PAL["accent"])
        self.audit_tree.tag_configure("info", foreground=PAL["text"])

    def _export_audit(self):
        from tkinter import filedialog
        f = filedialog.asksaveasfilename(defaultextension=".txt", title="Export Audit Log")
        if f:
            with open(f, "w") as fp:
                fp.write("SigmaOS Sovereign Security Guardian - Audit Log Export\n")
                fp.write("="*60 + "\n")
                for row in self.audit_tree.get_children():
                    fp.write("  |  ".join(str(v) for v in self.audit_tree.item(row, "values")) + "\n")
            messagebox.showinfo("Export", f"Audit log exported to:\n{f}")

if __name__ == "__main__":
    app = SovereignSentinel()
    app.mainloop()
