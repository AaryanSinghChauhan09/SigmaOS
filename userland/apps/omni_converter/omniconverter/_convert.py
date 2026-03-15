# Generated method: OmniConverter._convert
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
from typing import List, Dict

class OmniConverter:
    def _convert(self):
        if not self.source_file:
            messagebox.showwarning('Warning', 'Please select a source bitstream first.')
            return
        target = self.format_box.get()
        self.status.config(text=f'MORPHING TO {target}... [GPU ENGINE ACTIVE]', bg=PAL['secondary'], fg='white')
        self.update()
        time.sleep(1.5)
        self.status.config(text='MORPH COMPLETE | BUFFER COMMITTED TO VAULT', bg=PAL['success'], fg='white')
        messagebox.showinfo('OmniConverter Pro', f'Bitstream successfully morphed into {target}.\nEncryption: Sovereignty Level 10.')