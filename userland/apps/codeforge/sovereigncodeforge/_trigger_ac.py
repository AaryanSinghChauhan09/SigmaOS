"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._trigger_ac
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _trigger_ac(self):
        words = ['print(', 'import ', 'def ', 'class ', 'return ', "if __name__ == '__main__':", 'SigmaKernel', 'SovereignAINexus']
        self.ac_popup.delete(0, 'end')
        for w in words:
            self.ac_popup.insert('end', w)
        idx = self.txt.index('insert')
        bbox = self.txt.bbox(idx)
        if bbox:
            self.ac_popup.place(x=bbox[0] + 60, y=bbox[1] + 80)
