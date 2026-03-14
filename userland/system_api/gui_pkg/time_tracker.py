import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_LOGO, FONT_SMALL, FONT_BOLD

class TimeTrackerPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Time Tracker", "Track every second — Linux-grade pomodoro and task ledger")
        self._tt_elapsed = 0
        self._tt_running = False
        self._tt_job = None
        self._tt_total_secs = 0
        self._build_ui()

    def _build_ui(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True)

        # Left: Timer Panel
        l_fr = tk.Frame(body, bg=PAL["bg2"], width=380)
        l_fr.pack(side="left", fill="both", padx=(0, 6))
        l_fr.pack_propagate(False)

        # Task selector
        sel_c = self._card(l_fr, "📌 Current Task")
        sel_c.master.pack(fill="x", pady=8, padx=8)
        self.task_var = tk.StringVar(value="Kernel Hardening")
        tasks_avail = ["Kernel Hardening", "Linux Parity Engine", "GUI Polish v3",
                       "AI Nexus v2", "Bug Fix: Nav Crash", "Time Tracker Page",
                       "Scrum Burndown", "App Store Hydration"]
        ttk.Combobox(sel_c, textvariable=self.task_var, values=tasks_avail,
                      state="readonly", width=28).pack(fill="x", pady=4)

        # Timer display
        timer_c = self._card(l_fr, "⏱️ Elapsed Time")
        timer_c.master.pack(fill="x", pady=8, padx=8)

        self._tt_display = tk.Label(timer_c, text="00:00:00",
                                    font=("Consolas", 38, "bold"),
                                    fg=PAL["teal"], bg=PAL["card"])
        self._tt_display.pack(pady=10)

        self._tt_task_lbl = tk.Label(timer_c, text=f"Task: {self.task_var.get()}",
                                     font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"])
        self._tt_task_lbl.pack()

        btn_fr = tk.Frame(timer_c, bg=PAL["card"])
        btn_fr.pack(fill="x", pady=8)

        self.start_btn = ttk.Button(btn_fr, text="▶ Start", command=self.start_timer)
        self.start_btn.pack(side="left", fill="x", expand=True, padx=3)
        ttk.Button(btn_fr, text="⏹ Stop", command=self.stop_timer).pack(side="left", fill="x", expand=True, padx=3)
        ttk.Button(btn_fr, text="🔁 Lap", command=self.lap_timer).pack(side="left", fill="x", expand=True, padx=3)

        # Pomodoro
        pom_c = self._card(l_fr, "🍅 Pomodoro Mode")
        pom_c.master.pack(fill="x", pady=8, padx=8)
        pom_fr = tk.Frame(pom_c, bg=PAL["card"])
        pom_fr.pack(fill="x")
        for label, mins in [("25 min Focus", 25), ("5 min Break", 5), ("15 min Long Break", 15)]:
            ttk.Button(pom_fr, text=label,
                       command=lambda m=mins: self.gui._log_voice(
                           f"Pomodoro: {m}min timer set for {self.task_var.get()}")).pack(
                side="left", fill="x", expand=True, padx=2)

        # Daily summary
        sum_c = self._card(l_fr, "📅 Today's Summary")
        sum_c.master.pack(fill="x", pady=8, padx=8)
        self._tt_total_lbl = tk.Label(sum_c, text="Total Logged: 0h 0m",
                                      font=FONT_BOLD, fg=PAL["cyan"], bg=PAL["card"])
        self._tt_total_lbl.pack(pady=6)

        # Right: Time Log
        r_fr = tk.Frame(body, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True)

        log_c = self._card(r_fr, "📋 Time Log")
        log_c.master.pack(fill="both", expand=True, padx=8, pady=8)

        log_cols = ("Task", "Duration", "Time")
        self._tt_tree = ttk.Treeview(log_c, columns=log_cols, show="headings", height=20)
        for col in log_cols:
            self._tt_tree.heading(col, text=col)
            self._tt_tree.column(col, width=150 if col == "Task" else 90, anchor="center")
        self._tt_tree.pack(fill="both", expand=True)

        log_sb = ttk.Scrollbar(log_c, orient="vertical", command=self._tt_tree.yview)
        self._tt_tree.configure(yscrollcommand=log_sb.set)
        log_sb.pack(side="right", fill="y")

        btn_bar = tk.Frame(r_fr, bg=PAL["bg"])
        btn_bar.pack(fill="x", padx=8, pady=4)
        ttk.Button(btn_bar, text="🗑 Clear Log",
                   command=lambda: [self._tt_tree.delete(*self._tt_tree.get_children()),
                                    self._tt_total_lbl.config(text="Total Logged: 0h 0m"),
                                    setattr(self, "_tt_total_secs", 0)]).pack(side="right")
        ttk.Button(btn_bar, text="📤 Export CSV",
                   command=lambda: self.gui._log_voice("Time log exported to /workspace/time_logs/")).pack(side="right", padx=6)

        # Seed sample log entries
        sample_log = [
            ("AI Nexus v2", "01:24:00", "11:20:00"),
            ("GUI Polish v3", "00:45:00", "09:30:00"),
            ("Bug Fix: Nav Crash", "00:20:00", "08:55:00"),
        ]
        for row in sample_log:
            self._tt_tree.insert("", "end", values=row)

    def _tick(self):
        if self._tt_running:
            self._tt_elapsed += 1
            h = self._tt_elapsed // 3600
            m = (self._tt_elapsed % 3600) // 60
            s = self._tt_elapsed % 60
            self._tt_display.config(text=f"{h:02d}:{m:02d}:{s:02d}")
            self._tt_job = self.after(1000, self._tick)

    def start_timer(self):
        if not self._tt_running:
            self._tt_running = True
            self._tt_task_lbl.config(text=f"Task: {self.task_var.get()}")
            self.start_btn.config(text="⏸ Pause", style="Accent.TButton")
            self._tick()
        else:
            self._tt_running = False
            if self._tt_job:
                self.gui.after_cancel(self._tt_job)
            self.start_btn.config(text="▶ Resume", style="TButton")

    def stop_timer(self):
        self._tt_running = False
        if self._tt_job:
            self.gui.after_cancel(self._tt_job)
        h = self._tt_elapsed // 3600
        m = (self._tt_elapsed % 3600) // 60
        s = self._tt_elapsed % 60
        time_str = f"{h:02d}:{m:02d}:{s:02d}"
        task_name = self.task_var.get()
        self.log_task(task_name, time_str)
        self._tt_elapsed = 0
        self._tt_display.config(text="00:00:00")
        self.start_btn.config(text="▶ Start", style="TButton")

    def lap_timer(self):
        if self._tt_elapsed > 0:
            h = self._tt_elapsed // 3600
            m = (self._tt_elapsed % 3600) // 60
            s = self._tt_elapsed % 60
            self.log_task(f"[LAP] {self.task_var.get()}", f"{h:02d}:{m:02d}:{s:02d}")

    def log_task(self, task_name, duration_str):
        import datetime as _dt
        now = _dt.datetime.now().strftime("%H:%M:%S")
        self._tt_tree.insert("", 0, values=(task_name, duration_str, now))
        parts = duration_str.split(":")
        if len(parts) == 3:
            secs = int(parts[0]) * 3600 + int(parts[1]) * 60 + int(parts[2])
            self._tt_total_secs += secs
            th = self._tt_total_secs // 3600
            tm = (self._tt_total_secs % 3600) // 60
            self._tt_total_lbl.config(text=f"Total Logged: {th}h {tm}m")
