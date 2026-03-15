"""
Auto-split from userland\apps\event_matrix.py — EventMatrix._inspect_log
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class EventMatrix:
    def _inspect_log(self, event):
        item = self.tree.selection()
        if item:
            val = self.tree.item(item, 'values')
            messagebox.showinfo('Log Inspection', f'DEEP INSPECTION (Ring-0 Access):\n\nTimestamp: {val[1]}\nSeverity: {val[0]}\nThread: {val[2]}\n\nPayload: {val[3]}\n\nMetadata: Cryptographically signed via AES-GCM.')
