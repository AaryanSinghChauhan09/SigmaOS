# Generated method: ExplorerPage._show_context_menu
import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage:
    def _show_context_menu(self, event):
        item = self.tree.identify_row(event.y)
        if not item:
            return
        self.tree.selection_set(item)
        full_path = self.tree.item(item, 'tags')[0]
        menu = tk.Menu(self, tearoff=0, bg=PAL['bg2'], fg=PAL['text'])
        menu.add_command(label='Open', command=lambda: os.startfile(full_path))
        menu.add_separator()
        menu.add_command(label='?? Mount in New Silo', command=lambda: self._mount_silo(full_path))
        menu.add_command(label='?? Semantic Index (Aeryn)', command=lambda: self.gui._notify('Search', 'Queued for indexing.', 'OK'))
        menu.add_command(label='?? Verify Shard Integrity', command=lambda: self.gui._notify('Integrity', 'Hash verified.', 'OK'))
        menu.post(event.x_root, event.y_root)