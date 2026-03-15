"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._apply_ac
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _apply_ac(self):
        sel = self.ac_popup.get('active')
        if sel:
            self.txt.insert('insert', sel)
            self.ac_popup.place_forget()
