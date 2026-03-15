# Generated method: SovereignCodeForge.save_file
import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional

class SovereignCodeForge:
    def save_file(self):
        if not self.current_file:
            f = filedialog.asksaveasfilename(defaultextension='.py')
            if not f:
                return
            self.current_file = f
        try:
            with open(self.current_file, 'w', encoding='utf-8') as f:
                f.write(self.txt.get('1.0', 'end'))
            self._unsaved = False
            self.status.config(text=f'SAVED: {os.path.basename(self.current_file)}')
        except Exception as e:
            messagebox.showerror('Save Error', str(e))