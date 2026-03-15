"""
Auto-split from userland\apps\ncert_master_lab.py — NCERTMasterLab._show_res
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback, json, time
from typing import Dict, Any, List, Optional



class NCERTMasterLab:
    def _show_res(self, name, res):
        self._out.delete('1.0', 'end')
        self._out.insert('end', f'▶ LAB LOG: {name.upper()}\n', 'title')
        self._out.insert('end', f'OUTPUT: {res}\n\n')
        if self.engine and hasattr(self.engine, 'xp'):
            xp = getattr(self.engine, 'xp', 0)
            self._out.insert('end', f'◈ XP EARNED: +150 | TOTAL: {xp}\n', 'badge')
            if self._status_lbl:
                self._status_lbl.config(text=f'[XP: {xp} | COMPLETED: {self._completed_count}]')
