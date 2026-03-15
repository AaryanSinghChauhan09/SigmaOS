# Generated method: SovereignCodeForge._insert_tab
import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional

class SovereignCodeForge:
    def _insert_tab(self, event):
        if self.ac_popup.winfo_viewable():
            self._apply_ac()
            return 'break'
        self.txt.insert('insert', '    ')
        return 'break'