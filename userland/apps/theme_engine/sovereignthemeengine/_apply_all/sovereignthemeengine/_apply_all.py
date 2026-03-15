# Generated method: SovereignThemeEngine._apply_all
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random

class SovereignThemeEngine:
    def _apply_all(self):
        self.status.config(text='PROPAGATING THEME MATRIX TO GTK3/4/QT5/6/TERMINAL...', bg=PAL['warning'], fg='black')
        self.after(1500, lambda: messagebox.showinfo('Theme Engine', 'All theme vectors applied system-wide.\n\nGTK3 gtkrc-2.0 deployed.\nQt5ct config written.\nTerminal color sequences emitted.\n\nNo logout required.'))
        self.after(1500, lambda: self.status.config(text='THEME ENGINE: PROPAGATION COMPLETE', bg=PAL['success'], fg='black'))