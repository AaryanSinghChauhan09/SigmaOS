# Generated method: SigmaQuickSettings._toggle_dev
import tkinter as tk
from tkinter import ttk

class SigmaQuickSettings:
    def _toggle_dev(self):
        self.kernel.cfg.DEVELOPER_MODE = self.dev_var.get()
        print(f'[INFO] QuickSettings: Developer mode set to {self.kernel.cfg.DEVELOPER_MODE}')