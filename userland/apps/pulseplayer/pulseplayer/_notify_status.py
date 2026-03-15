"""
Auto-split from userland\apps\pulseplayer.py — PulsePlayer._notify_status
"""

import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict



class PulsePlayer:
    def _notify_status(self, msg):
        old = self.status.cget('text')
        self.status.config(text=msg.upper(), bg=PAL['success'])
        self.after(3000, lambda: self.status.config(text=old, bg=PAL['accent']))
