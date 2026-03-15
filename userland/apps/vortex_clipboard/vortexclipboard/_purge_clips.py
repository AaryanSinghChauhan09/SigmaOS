# Generated method: VortexClipboard._purge_clips
import tkinter as tk
from tkinter import ttk, messagebox
import time

class VortexClipboard:
    def _purge_clips(self):
        conf = messagebox.askyesno('Temporal Purge', 'Eradicate all clipboard history across local nodes?')
        if conf:
            for item in self.tree.get_children():
                self.tree.delete(item)
            self.status.config(text='VORTEX PURGED | NO RESIDUE', bg=PAL['success'], fg='black')