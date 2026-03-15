# Generated method: ExplorerPage._on_double_click
import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage:
    def _on_double_click(self, event):
        item = self.tree.selection()[0]
        full_path = self.tree.item(item, 'tags')[0]
        if os.path.isdir(full_path):
            self.current_path.set(full_path)
            self._load_dir()
        else:
            os.startfile(full_path)