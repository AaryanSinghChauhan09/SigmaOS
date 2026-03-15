# Generated method: OmniSearch._launch_asset
import tkinter as tk
from tkinter import ttk, messagebox
import time

class OmniSearch:
    def _launch_asset(self, event):
        item = self.tree.selection()
        if item:
            val = self.tree.item(item, 'values')[0]
            messagebox.showinfo('Omni-Launch', f'Neural fetch complete. Launching vector:\n\n{val}')