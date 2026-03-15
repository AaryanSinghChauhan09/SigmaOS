# Generated method: SigmaQuickSettings._apply_mode
import tkinter as tk
from tkinter import ttk

class SigmaQuickSettings:
    def _apply_mode(self, event=None):
        mode = self.mode_var.get()
        self.kernel.cfg.apply_mode(mode)
        print(f'[INFO] QuickSettings: Mode switched to {mode}')