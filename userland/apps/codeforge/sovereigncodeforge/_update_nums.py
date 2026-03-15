"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._update_nums
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _update_nums(self):
        cnt = int(self.txt.index('end-1c').split('.')[0])
        self.num_bar.config(state='normal')
        self.num_bar.delete('1.0', 'end')
        self.num_bar.insert('1.0', '\n'.join((str(i) for i in range(1, cnt + 1))))
        self.num_bar.config(state='disabled')
