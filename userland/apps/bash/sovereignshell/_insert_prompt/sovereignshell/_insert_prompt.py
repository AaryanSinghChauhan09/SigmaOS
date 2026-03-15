# Generated method: SovereignShell._insert_prompt
import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random

class SovereignShell:
    def _insert_prompt(self):
        self._write(f'user@sigmaos', PAL['success'])
        self._write(':', PAL['text'])
        self._write(f"{self.curr_dir.replace(os.path.expanduser('~'), '~')}", PAL['prompt'])
        self._write('$ ', PAL['text'])
        self.terminal.mark_set('input_start', 'insert')