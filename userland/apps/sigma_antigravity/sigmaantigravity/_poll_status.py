"""
Auto-split from userland\apps\sigma_antigravity.py — SigmaAntigravity._poll_status
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import threading, webbrowser, urllib.parse, json, os, time, sys
from typing import Dict, Any, List, Optional



class SigmaAntigravity:
    def _poll_status(self):
        self._update_history()
        self.after(5000, self._poll_status)
