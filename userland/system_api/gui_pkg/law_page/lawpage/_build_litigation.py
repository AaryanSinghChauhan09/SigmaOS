# Generated method: LawPage._build_litigation
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class LawPage:
    def _build_litigation(self, parent):
        tk.Label(parent, text='Litigation & E-Discovery', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg']).pack(pady=10)
        log = self.gui._console(parent, height=15)
        log.pack(fill='both', expand=True, padx=20, pady=10)
        ttk.Button(parent, text='Launch Discovery Scan', command=lambda: self.gui._log(log, 'Searching encrypted silos...', 'INFO')).pack(pady=10)