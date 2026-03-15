# Generated method: ExcelHub._run_clean
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import time

class ExcelHub:
    def _run_clean(self):
        self._add_log('CLEANER', 'PURGING DUPLICATE SHIMS...', PAL['dim'])
        self.after(800, lambda: self._add_log('CLEANER', 'DEDUPLICATION RATIO: 1.8x.', PAL['success']))