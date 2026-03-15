"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._tree_load
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _tree_load(self, event):
        sel = self._tree.selection()
        if not sel:
            return
        vals = self._tree.item(sel[0], 'values')
        if vals and os.path.isfile(vals[0]):
            self._open_path(vals[0])
