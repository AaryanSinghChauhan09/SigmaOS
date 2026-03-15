# Generated method: ExplorerPage._refresh_silos
import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage:
    def _refresh_silos(self):
        for w in self.silo_list.winfo_children():
            w.destroy()
        for sid, data in self.kernel.silo_fs.active_silos.items():
            tk.Label(self.silo_list, text=f"?? {data['app']}", fg=PAL['cyan'], bg=PAL['card']).pack(anchor='w')