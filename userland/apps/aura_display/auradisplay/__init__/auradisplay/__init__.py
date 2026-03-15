# Generated method: AuraDisplay.__init__
import tkinter as tk
from tkinter import ttk, messagebox

class AuraDisplay:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Aura Display')
        self.geometry('700x550')
        self.configure(bg=PAL['bg'])
        self.circadian_sync = tk.BooleanVar(value=True)
        self.blue_filter = tk.IntVar(value=75)
        self.brightness = tk.IntVar(value=60)
        self._setup_styles()
        self._build_ui()