"""
Auto-split from userland\apps\pulseplayer.py — PulsePlayer._set_status
"""

import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict



class PulsePlayer:
    def _set_status(self, msg):
        self.status.config(text=msg.upper())
