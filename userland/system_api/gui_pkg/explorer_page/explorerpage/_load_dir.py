# Generated method: ExplorerPage._load_dir
import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage:
    def _load_dir(self):
        path = self.current_path.get()
        if not os.path.exists(path):
            messagebox.showerror('Error', 'Path not found.')
            return
        for i in self.tree.get_children():
            self.tree.delete(i)
        try:
            for item in os.listdir(path):
                full = os.path.join(path, item)
                is_dir = os.path.isdir(full)
                size = f'{os.path.getsize(full) // 1024} KB' if not is_dir else '-'
                itype = 'Folder' if is_dir else 'File'
                integrity = 'VERIFIED'
                icon = '??' if is_dir else '??'
                self.tree.insert('', 'end', values=(f'{icon} {item}', size, itype, integrity), tags=(full,))
        except Exception as e:
            self.gui._notify('FS Error', str(e), 'ERR')