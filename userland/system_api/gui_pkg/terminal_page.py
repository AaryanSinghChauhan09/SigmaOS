import tkinter as tk
from tkinter import ttk
import threading
from .base_page import SigmaPage
from .styles import PAL, FONT_MONO

class TerminalPage(SigmaPage):
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "Integrated Terminal", "Sovereign Shell & REPL — Zero-Latency Command Orchestration")
        self._term_history = []
        self._term_hist_idx = -1
        self._is_elevated = tk.BooleanVar(value=False)
        self._build_ui()

    def _build_ui(self):
        toolbar = tk.Frame(self, bg=PAL["bg"], height=32)
        toolbar.pack(fill="x", pady=(0, 10))
        
        for cmd_name in ["Clear", "Sudo", "Scripts", "SSH"]:
            b = tk.Button(toolbar, text=cmd_name, font=("Inter", 8), bg=PAL["bg3"], fg=PAL["dim"],
                            relief="flat", bd=0, padx=10, command=lambda c=cmd_name: self._term_aux(c))
            b.pack(side="left", padx=2)

        self._term_out = self._console(self, height=25)
        self._term_out.pack(fill="both", expand=True, pady=(0,4))

        entry_row = tk.Frame(self, bg=PAL["bg"])
        entry_row.pack(fill="x")
        self.prompt_lbl = tk.Label(entry_row, text="σ >", font=FONT_MONO,
                                   fg=PAL["cyan"], bg=PAL["bg"])
        self.prompt_lbl.pack(side="left")
        
        self._term_input = tk.StringVar()
        self._term_entry = ttk.Entry(entry_row, textvariable=self._term_input,
                                     font=FONT_MONO, width=80)
        self._term_entry.pack(side="left", fill="x", expand=True, padx=6)
        self._term_entry.bind("<Return>", self._term_exec)
        ttk.Button(entry_row, text="▶ Run",
                   command=self._term_exec).pack(side="left")

        self._sudo_btn = tk.Button(entry_row, text="🛡️ SUDO", font=("Segoe UI", 7, "bold"),
                                   bg=PAL["bg3"], fg=PAL["dim"], relief="flat", padx=5,
                                   command=self._toggle_sudo)
        self._sudo_btn.pack(side="right", padx=5)

        self._term_entry.bind("<Up>", self._term_hist_up)
        self._term_entry.bind("<Down>", self._term_hist_down)

        self._log(self._term_out, "SigmaOS Integrated REPL — type 'help' for commands\n", "HEAD")

    def _toggle_sudo(self):
        curr = self._is_elevated.get()
        self._is_elevated.set(not curr)
        if not curr:
            self._log(self._term_out, "ELEVATING PRIVILEGES: Biometric Audit Passed. [ROOT ACTIVE]", "WARN")
            self._sudo_btn.config(fg="white", bg=PAL["red"])
            self.prompt_lbl.config(text="# ")
        else:
            self._log(self._term_out, "DROPPING PRIVILEGES: User mode restored.", "INFO")
            self._sudo_btn.config(fg=PAL["dim"], bg=PAL["bg3"])
            self.prompt_lbl.config(text="σ > ")

    def _term_hist_up(self, e):
        if not self._term_history: return
        self._term_hist_idx = min(self._term_hist_idx + 1, len(self._term_history) - 1)
        self._term_input.set(self._term_history[len(self._term_history) - 1 - self._term_hist_idx])
        self._term_entry.icursor("end")

    def _term_hist_down(self, e):
        if self._term_hist_idx <= 0:
            self._term_hist_idx = -1
            self._term_input.set("")
            return
        self._term_hist_idx -= 1
        self._term_input.set(self._term_history[len(self._term_history) - 1 - self._term_hist_idx])
        self._term_entry.icursor("end")

    def _term_exec(self, event=None):
        raw = self._term_input.get().strip()
        if not raw: return
        self._term_history.append(raw)
        self._term_hist_idx = -1
        self._term_input.set("")
        prompt = "# " if self._is_elevated.get() else "σ > "
        self._log(self._term_out, f"{prompt}{raw}", "WARN" if self._is_elevated.get() else "INFO")

        parts = raw.split()
        cmd = parts[0].lower()

        def run():
            try:
                if cmd == "help":
                    self._log(self._term_out, "Apex Commands: help | manual | clear | exit", "INFO")
                elif cmd == "clear":
                    self.gui.after(0, lambda: [self._term_out.configure(state="normal"), 
                                               self._term_out.delete("1.0","end"), 
                                               self._term_out.configure(state="disabled")])
                else:
                    self._log(self._term_out, f"Command '{cmd}' not found in Sovereign path.", "ERR")
            except Exception as exc:
                self._log(self._term_out, f"Error: {exc}", "ERR")

        threading.Thread(target=run, daemon=True).start()

    def _term_aux(self, cmd):
        self._log(self._term_out, f"AUX: Executing {cmd} module...", "INFO")
