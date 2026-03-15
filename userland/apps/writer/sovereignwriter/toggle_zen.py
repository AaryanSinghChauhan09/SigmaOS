"""
Auto-split from userland\apps\writer.py — SovereignWriter.toggle_zen
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import os



class SovereignWriter:
    def toggle_zen(self):
        self.is_zen = not self.is_zen
        if self.is_zen:
            self.sidebar.pack_forget()
            self.ai_side.pack_forget()
            self.toolbar.pack_forget()
            self.editor.config(padx=150, pady=100, font=('Inter', 18))
        else:
            self.sidebar.pack(side='left', fill='y', before=self.editor_fr)
            self.ai_side.pack(side='right', fill='y', after=self.editor_fr)
            self.toolbar.pack(side='top', fill='x', before=self.main_fr)
            self.editor.config(padx=80, pady=80, font=('Inter', 13))
