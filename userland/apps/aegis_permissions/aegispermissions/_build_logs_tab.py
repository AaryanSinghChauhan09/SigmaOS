# Generated method: AegisPermissions._build_logs_tab
import tkinter as tk
from tkinter import ttk, messagebox

class AegisPermissions:
    def _build_logs_tab(self):
        log_text = tk.Text(self.tab_logs, bg=PAL['sidebar'], fg=PAL['warning'], font=('Consolas', 10), relief='flat')
        log_text.pack(fill='both', expand=True)
        log_text.insert(tk.END, '[SYS-CLK 14:02:00] pdf_forge.py requested Network Access -> DENIED (Aegis Rule 4)\n')
        log_text.insert(tk.END, '[SYS-CLK 14:02:05] omni_search.py initiated RAM Map -> OK (Token Verified)\n')
        log_text.insert(tk.END, '[SYS-CLK 14:05:12] pulse_browser requested GPS -> DENIED (Temporal Block Active)\n')
        log_text.config(state=tk.DISABLED)