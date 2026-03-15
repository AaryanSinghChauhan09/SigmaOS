# Generated method: TerminalPage._build_ui
import tkinter as tk
from tkinter import ttk
import threading
from .base_page import SigmaPage
from .styles import PAL, FONT_MONO

class TerminalPage:
    def _build_ui(self):
        toolbar = tk.Frame(self, bg=PAL['bg'], height=32)
        toolbar.pack(fill='x', pady=(0, 10))
        for cmd_name in ['Clear', 'Sudo', 'Scripts', 'SSH']:
            b = tk.Button(toolbar, text=cmd_name, font=('Inter', 8), bg=PAL['bg3'], fg=PAL['dim'], relief='flat', bd=0, padx=10, command=lambda c=cmd_name: self._term_aux(c))
            b.pack(side='left', padx=2)
        self._term_out = self._console(self, height=25)
        self._term_out.pack(fill='both', expand=True, pady=(0, 4))
        entry_row = tk.Frame(self, bg=PAL['bg'])
        entry_row.pack(fill='x')
        self.prompt_lbl = tk.Label(entry_row, text='σ >', font=FONT_MONO, fg=PAL['cyan'], bg=PAL['bg'])
        self.prompt_lbl.pack(side='left')
        self._term_input = tk.StringVar()
        self._term_entry = ttk.Entry(entry_row, textvariable=self._term_input, font=FONT_MONO, width=80)
        self._term_entry.pack(side='left', fill='x', expand=True, padx=6)
        self._term_entry.bind('<Return>', self._term_exec)
        ttk.Button(entry_row, text='▶ Run', command=self._term_exec).pack(side='left')
        self._sudo_btn = tk.Button(entry_row, text='🛡️ SUDO', font=('Segoe UI', 7, 'bold'), bg=PAL['bg3'], fg=PAL['dim'], relief='flat', padx=5, command=self._toggle_sudo)
        self._sudo_btn.pack(side='right', padx=5)
        self._term_entry.bind('<Up>', self._term_hist_up)
        self._term_entry.bind('<Down>', self._term_hist_down)
        self._log(self._term_out, "SigmaOS Integrated REPL — type 'help' for commands\n", 'HEAD')