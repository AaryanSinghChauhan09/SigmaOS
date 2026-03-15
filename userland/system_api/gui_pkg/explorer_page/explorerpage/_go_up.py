# Generated method: ExplorerPage._go_up
import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage:
    def _go_up(self):
        new = os.path.dirname(self.current_path.get())
        self.current_path.set(new)
        self._load_dir()