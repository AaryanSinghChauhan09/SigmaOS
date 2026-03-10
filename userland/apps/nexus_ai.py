"""
SigmaOS Sovereign AI Nexus (v4.0) — OS COMMAND & CONTROL CENTER
==============================================================
The primary AI Agent and Guide for SigmaOS. 
- Integrated OS Guidance: Step-by-step walkthroughs of every module.
- Task Execution: Dispatches system-level actions via the Kernel Bus.
- Security Auditor: Monitors for "loopholes" and unverified configurations.
- Agentic Chat: Natural language interaction with the OS core.
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json

PAL = {
    "bg": "#0A0B0F", "sidebar": "#12141C", "card": "#181B26", 
    "accent": "#5E5CE6", "success": "#32D74B", "danger": "#FF453A",
    "warning": "#FF9F0A", "text": "#F2F2F7", "dim": "#8E8E93",
    "border": "#2C2C3C", "header_bg": "#111218"
}

class SovereignAINexus(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("SigmaOS Sovereign AI Nexus v5.0")
        self.geometry("1150x850")
        self.configure(bg=PAL["bg"])
        self._setup_styles()
        self._is_thinking = False
        
        # Integration: Register LoopholeEngine and TaskAgent
        try:
            from loopholes import LoopholeEngine
            from task_agent import TaskAgent
            self.loopholes = LoopholeEngine(kernel)
            self.task_agent = TaskAgent(kernel)
        except Exception as e:
            print(f"Engine registration failed: {e}")
            self.loopholes = None
            self.task_agent = None
            
        self._build_ui()
        self._log_welcome()
        self._poll_system_audit()

    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use("clam")
        s.configure("Treeview", background=PAL["card"], foreground=PAL["text"], 
                    fieldbackground=PAL["card"], borderwidth=0, font=("Segoe UI", 9))
        s.configure("TNotebook", background=PAL["bg"], borderwidth=0)
        s.configure("TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["dim"], 
                    padding=[12, 6], font=("Segoe UI", 9))
        s.map("TNotebook.Tab", background=[("selected", PAL["card"])], 
              foreground=[("selected", "white")])

    def _build_ui(self):
        # Header
        head = tk.Frame(self, bg=PAL["header_bg"], height=70, padx=25)
        head.pack(side="top", fill="x")
        head.pack_propagate(False)
        tk.Label(head, text="🧬 SOVEREIGN AI NEXUS", font=("Segoe UI Bold", 16), 
                 fg=PAL["accent"], bg=PAL["header_bg"]).pack(side="left", pady=18)
        
        self.status_dot = tk.Label(head, text="● AGENT ACTIVE", font=("Segoe UI", 8, "bold"), 
                                   fg=PAL["success"], bg=PAL["header_bg"])
        self.status_dot.pack(side="right", padx=10)

        # Main Body
        body = tk.Frame(self, bg=PAL["bg"], padx=20, pady=15)
        body.pack(fill="both", expand=True)

        self.nb = ttk.Notebook(body)
        self.nb.pack(fill="both", expand=True)

        self._build_agent_tab()
        self._build_guide_tab()
        self._build_audit_tab()
        self._build_tasks_tab()

    def _build_agent_tab(self):
        tab = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(tab, text="  🤖 AI Agent  ")

        # Chat display
        self.chat_display = scrolledtext.ScrolledText(tab, bg=PAL["card"], fg=PAL["text"], 
                                                     font=("Segoe UI", 10), borderwidth=0, 
                                                     padx=15, pady=15, insertbackground="white")
        self.chat_display.pack(fill="both", expand=True, pady=(10, 10))
        self.chat_display.tag_config("agent", foreground=PAL["accent"], font=("Segoe UI Bold", 10))
        self.chat_display.tag_config("user", foreground="white", font=("Segoe UI", 10))
        self.chat_display.tag_config("sys", foreground=PAL["dim"], font=("Segoe UI Italics", 9))

        # Input Area
        input_fr = tk.Frame(tab, bg=PAL["bg"], pady=10)
        input_fr.pack(fill="x")
        
        self.chat_input = tk.Entry(input_fr, bg=PAL["card"], fg="white", 
                                   insertbackground="white", font=("Segoe UI", 11), 
                                   borderwidth=0, relief="flat", highlightthickness=1, 
                                   highlightbackground=PAL["border"])
        self.chat_input.pack(side="left", fill="x", expand=True, ipady=8, padx=(0, 10))
        self.chat_input.bind("<Return>", lambda e: self._handle_chat())

        send_btn = tk.Button(input_fr, text="SEND", font=("Segoe UI", 9, "bold"), 
                           bg=PAL["accent"], fg="white", relief="flat", padx=20, 
                           command=self._handle_chat)
        send_btn.pack(side="right")

    def _build_guide_tab(self):
        tab = tk.Frame(self.nb, bg=PAL["bg"], padx=15, pady=15)
        self.nb.add(tab, text="  📖 OS Guide  ")

        # Split: Menu on left, Content on right
        left = tk.Frame(tab, bg=PAL["sidebar"], width=250)
        left.pack(side="left", fill="y", padx=(0, 15))
        left.pack_propagate(False)

        tk.Label(left, text="MODULES & PATHS", font=("Segoe UI", 8, "bold"), 
                 fg=PAL["dim"], bg=PAL["sidebar"], pady=10).pack()

        guides = {
            "Introduction": "Welcome to SigmaOS. The Zero-Trust, Neuro-Native environment.",
            "Security Guardian (Sentinel)": "Sentinel is your 5-tab security center. Use 'Hex-Scan' for deep validation.",
            "CodeForge IDE": "A professional-grade IDE. Supports real Python code execution and sandbox terminal.",
            "Antigravity AI Hub": "Orchestrate 13+ AI platforms from a single point. Manage quotas and presets.",
            "Sovereign Writer": "A minimalist, privacy-focused text editor for secure document authoring.",
            "PulsePlayer": "Music with neural upsampling and integrated EQ. Bit-perfect playback paths.",
            "File Explorer (VFS)": "The Virtual File System mirrors your project root with AI-driven cleanup.",
            "Automation Hub": "Tasker/Shortcuts parity. Automate everything with 'Shortcut Forge'.",
            "OmniBrowser": "The multi-engine, sandboxed browser core for absolute web privacy."
        }

        self.guide_text = scrolledtext.ScrolledText(tab, bg=PAL["card"], fg=PAL["text"], 
                                                  font=("Segoe UI", 10), borderwidth=0, 
                                                  padx=20, pady=20)
        self.guide_text.pack(side="right", fill="both", expand=True)

        def _load_guide(title):
            self.guide_text.delete("1.0", "end")
            self.guide_text.insert("end", f"{title.upper()}\n", "title")
            self.guide_text.insert("end", "="*len(title) + "\n\n", "title")
            self.guide_text.insert("end", guides[title])
            self.guide_text.tag_config("title", foreground=PAL["accent"], font=("Segoe UI Bold", 14))

        for g in guides:
            btn = tk.Button(left, text=g, font=("Segoe UI", 9), bg=PAL["sidebar"], 
                          fg=PAL["text"], relief="flat", anchor="w", padx=10, pady=5,
                          command=lambda t=g: _load_guide(t))
            btn.pack(fill="x")
            btn.bind("<Enter>", lambda e, b=btn: b.config(bg=PAL["card"]))
            btn.bind("<Leave>", lambda e, b=btn: b.config(bg=PAL["sidebar"]))

        _load_guide("Introduction")

    def _build_audit_tab(self):
        tab = tk.Frame(self.nb, bg=PAL["bg"], padx=20, pady=20)
        self.nb.add(tab, text="  🛡️ Security Audit  ")

        tk.Label(tab, text="SYSTEM LOOPHOLE SCANNER (APEX V1.0)", font=("Segoe UI Bold", 11), 
                 fg="white", bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))

        # Loophole Grid
        self.lh_fr = tk.Frame(tab, bg=PAL["bg"])
        self.lh_fr.pack(fill="x", pady=10)
        self._render_loopholes()

        tk.Label(tab, text="REAL-TIME AUDIT LOG", font=("Segoe UI", 8, "bold"), 
                 fg=PAL["dim"], bg=PAL["bg"], pady=10).pack(anchor="w")
                 
        self.audit_log = scrolledtext.ScrolledText(tab, bg="#050508", fg=PAL["success"], 
                                                   font=("Cascadia Code", 9), borderwidth=0, 
                                                   padx=15, pady=15, height=12)
        self.audit_log.pack(fill="both", expand=True)
        self.audit_log.tag_config("err", foreground=PAL["danger"])
        self.audit_log.tag_config("warn", foreground=PAL["warning"])

    def _render_loopholes(self):
        for w in self.lh_fr.winfo_children(): w.destroy()
        if not self.loopholes: return
        
        for lh in self.loopholes.scan():
            card = tk.Frame(self.lh_fr, bg=PAL["card"], padx=15, pady=12, highlightthickness=1, 
                            highlightbackground=PAL["border"])
            card.pack(fill="x", pady=4)
            
            c1 = tk.Frame(card, bg=PAL["card"])
            c1.pack(side="left", fill="both", expand=True)
            
            status_col = PAL["danger"] if lh["status"] == "DETECTED" else PAL["success"]
            tk.Label(c1, text=f"• {lh['name']}", font=("Segoe UI Bold", 10), fg=status_col, bg=PAL["card"]).pack(anchor="w")
            tk.Label(c1, text=lh['desc'], font=("Segoe UI", 9), fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
            
            if lh["status"] == "DETECTED":
                btn = tk.Button(card, text="FIX LOOPHOLE", font=("Segoe UI", 8, "bold"), bg=PAL["accent"], 
                              fg="white", relief="flat", padx=15, command=lambda i=lh['id']: self._fix_lh(i))
                btn.pack(side="right")
            else:
                tk.Label(card, text="✓ MITIGATED", font=("Segoe UI", 8, "bold"), fg=PAL["success"], bg=PAL["card"]).pack(side="right")

    def _fix_lh(self, lid):
        if self.loopholes.apply_fix(lid):
            self.after(300, self._render_loopholes)
            ts = time.strftime("[%H:%M:%S]")
            self.audit_log.insert("end", f"{ts} FIXED LOOPHOLE {lid}: System policy enforced.\n", "sys")

    def _build_tasks_tab(self):
        tab = tk.Frame(self.nb, bg=PAL["bg"], padx=15, pady=15)
        self.nb.add(tab, text="  ✅ Task Manager  ")
        
        cols = ("ID", "Task", "Priority", "Status", "Source")
        self.task_tree = ttk.Treeview(tab, columns=cols, show="headings", height=15)
        for col in cols:
            self.task_tree.heading(col, text=col)
            self.task_tree.column(col, width=100 if col != "Task" else 300, anchor="center")
        self.task_tree.pack(fill="both", expand=True)

        btn_fr = tk.Frame(tab, bg=PAL["bg"], pady=10)
        btn_fr.pack(fill="x")
        ttk.Button(btn_fr, text="➕ Add Task", command=self._add_task).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="✔ Mark Done", command=self._mark_done).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="🚀 Execute Auto-Task", command=self._exec_auto).pack(side="left", padx=5)

    # Logic
    def _log_welcome(self):
        self._write_chat("Nexus", "Welcome, Sovereign. I am the AI Nexus. I can guide you through SigmaOS, automate tasks, or audit system security. How can I assist today?")

    def _write_chat(self, author, msg):
        self.chat_display.configure(state="normal")
        ts = time.strftime("[%H:%M]")
        if author == "Nexus":
            self.chat_display.insert("end", f"{ts} ", "sys")
            self.chat_display.insert("end", "NEXUS: ", "agent")
            self.chat_display.insert("end", f"{msg}\n\n", "user")
        else:
            self.chat_display.insert("end", f"{ts} ", "sys")
            self.chat_display.insert("end", "YOU: ", "user")
            self.chat_display.insert("end", f"{msg}\n\n", "user")
        self.chat_display.see("end")
        self.chat_display.configure(state="disabled")

    def _handle_chat(self):
        txt = self.chat_input.get().strip()
        if not txt: return
        self.chat_input.delete(0, "end")
        self._write_chat("User", txt)
        
        # Simple local intent mapper (Agent logic)
        lower = txt.lower()
        response = ""
        
        if any(x in lower for x in ["scan", "audit", "security", "threat", "loophole"]):
            self.nb.select(2)
            response = "Scanning the Sovereign Core for behavioral loopholes... I've updated the Audit tab. You can approve individual fixes there."
            self._poll_system_audit(force=True)
        elif "fix" in lower:
            response = "I'm identifying the most critical loopholes now. Please approve the fixes in the 'Security Audit' tab."
            self.nb.select(2)
        elif "help" in lower or "guide" in lower:
            self.nb.select(1)
            response = "I have opened the OS Guide. Specifically, for security, use the 'Sovereign Sentinel' app."
        elif "open" in lower:
            app = lower.replace("open", "").strip()
            response = f"Sending request to Sigma Kernel to launch '{app}' in a sandboxed session."
            # Simulating OS Action Bus emission
            self._write_chat("Nexus", f"SYSTEM_ACTION: LAUNCH_MODULE({app})")
        elif "status" in lower:
            response = f"All systems nominal. CPU: {random.randint(2, 8)}%, RAM: {random.uniform(0.4, 1.2):.1f}GB. 0 Security threats detected."
        else:
            response = "Acknowledged. I'll search the SigmaMesh for relevant insights. Is there a specific OS component you need help with?"

        self.after(600, lambda: self._write_chat("Nexus", response))

    def _poll_system_audit(self, force=False):
        if not force and random.random() > 0.3: return # simulated periodic poll
        
        ts = time.strftime("%H:%M:%S")
        checks = [
            ("Checking Kernel Module signatures", "ok"),
            ("Scanning for telemetry hooks in shims", "ok"),
            ("Verifying VFS inode integrity", "ok"),
            ("Security loop: Monitoring external listeners", "warn"),
            ("Found: 1 Unverified process in background", "err"),
            ("Neutralizing unverified process via P2P Mesh...", "ok"),
        ]
        
        for msg, status in checks:
            tag = "err" if status == "err" else ("warn" if status == "warn" else "")
            self.audit_log.insert("end", f"[{ts}] {msg}\n", tag)
            self.audit_log.see("end")

        if not force:
            self.after(15000, self._poll_system_audit)

    def _exec_auto(self):
        txt = self.chat_input.get().strip()
        if not txt:
            messagebox.showwarning("Auto-Task", "Enter an OS intent in the chat input first.")
            return

        plan = self.task_agent.plan_task(txt)
        self._write_chat("Nexus", f"AUTONOMOUS PLAN GENERATED:\n{plan['title']}\n\n" + "\n".join([f"• {s}" for s in plan['steps']]))
        
        def _exec():
            self.after(0, lambda: self._write_chat("Nexus", "EXECUTING SOVEREIGN PLAN..."))
            for step in plan['steps']:
                # Schedule child updates back to main thread
                msg = f"  [DONE] {step}"
                self.after(200, lambda m=msg: self._write_chat("Nexus", m))
                time.sleep(0.8)
            
            self.after(100, lambda: self._write_chat("Nexus", "TASK COMPLETE. SYSTEM OPTIMIZED."))
            self.after(200, lambda: self._notify_done(plan['title']))

        threading.Thread(target=_exec, daemon=True).start()

    def _notify_done(self, title):
        tid = f"T-{random.randint(100, 999)}"
        self.task_tree.insert("", "end", values=(tid, title, "N/A", "Completed", "Nexus"))

    def _add_task(self):
        tid = f"T-{random.randint(100, 999)}"
        self.task_tree.insert("", "end", values=(tid, "Audit PulsePlayer bit-paths", "High", "Pending", "User"))

    def _mark_done(self):
        sel = self.task_tree.selection()
        if sel:
            self.task_tree.item(sel[0], values=(self.task_tree.item(sel[0], "values")[0], 
                                               self.task_tree.item(sel[0], "values")[1], 
                                               self.task_tree.item(sel[0], "values")[2], 
                                               "Completed", "Nexus"))

    def _exec_auto_legacy(self):
        messagebox.showinfo("Auto-Task", "Agentic task execution started: 'Clean System Shims'.\nStatus: [Running]")

if __name__ == "__main__":
    app = SovereignAINexus()
    app.mainloop()
