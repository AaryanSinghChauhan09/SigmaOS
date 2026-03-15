"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge.stop_exec
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def stop_exec(self):
        if self._proc and self._proc.poll() is None:
            self._proc.terminate()
            self.term.insert('end', '\n[RUNTIME] Execution terminated by user.\n', 'warn')
        else:
            self.status.config(text='No running process to stop.')
