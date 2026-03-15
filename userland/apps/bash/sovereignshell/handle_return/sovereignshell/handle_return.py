# Generated method: SovereignShell.handle_return
import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random

class SovereignShell:
    def handle_return(self, event):
        cmd = self.terminal.get('input_start', 'end-1c').strip()
        self._write('\n')
        if cmd:
            self.history.append(cmd)
            self.history_idx = -1
            self._execute(cmd)
        self._insert_prompt()
        return 'break'