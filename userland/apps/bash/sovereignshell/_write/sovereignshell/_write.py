# Generated method: SovereignShell._write
import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random

class SovereignShell:
    def _write(self, text, color=None):
        tag = f'tag_{random.randint(0, 99999)}'
        if color:
            self.terminal.tag_config(tag, foreground=color)
            self.terminal.insert(tk.END, text, tag)
        else:
            self.terminal.insert(tk.END, text)
        self.terminal.see(tk.END)