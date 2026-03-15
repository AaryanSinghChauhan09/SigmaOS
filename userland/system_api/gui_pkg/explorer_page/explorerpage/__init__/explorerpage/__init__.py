# Generated method: ExplorerPage.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'SOVEREIGN EXPLORER', 'Distributed FS & Silo Orchestration')
        self.current_path = tk.StringVar(value=gui.kernel._root)
        self.build()