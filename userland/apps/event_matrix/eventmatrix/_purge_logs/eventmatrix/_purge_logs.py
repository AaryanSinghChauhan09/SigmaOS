# Generated method: EventMatrix._purge_logs
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class EventMatrix:
    def _purge_logs(self):
        conf = messagebox.askyesno('Log Purge', 'Permanently shred all system events? This bypasses standard recycle routines.')
        if conf:
            self.tree.delete(*self.tree.get_children())
            self.status.config(text='LOG MATRIX SHREDDED | ZERO-TRUST IMMUTABILITY RESTORED', bg=PAL['danger'], fg='white')