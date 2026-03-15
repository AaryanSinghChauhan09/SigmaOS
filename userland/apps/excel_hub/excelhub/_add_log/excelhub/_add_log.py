# Generated method: ExcelHub._add_log
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import time

class ExcelHub:
    def _add_log(self, author, msg, color='#E8E8E8'):
        if hasattr(self, 'log'):
            self.log.insert('end', f'[{author}] {msg}\n')
            self.log.see('end')