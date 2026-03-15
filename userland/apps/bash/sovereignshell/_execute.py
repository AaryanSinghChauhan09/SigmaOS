"""
Auto-split from userland\apps\bash.py — SovereignShell._execute
"""

import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random



class SovereignShell:
    def _execute(self, cmd):
        parts = cmd.split()
        base = parts[0]
        if base == 'clear':
            self.terminal.delete('1.0', tk.END)
        elif base == 'ls':
            try:
                files = os.listdir(self.curr_dir)
                self._write('\n'.join(files) + '\n', PAL['text'])
            except Exception as e:
                self._write(f'ls: {e}\n', PAL['error'])
        elif base == 'cd':
            target = parts[1] if len(parts) > 1 else os.path.expanduser('~')
            try:
                os.chdir(target)
                self.curr_dir = os.getcwd()
            except Exception as e:
                self._write(f'cd: {e}\n', PAL['error'])
        elif base == 'whoami':
            self._write('sovereign_user\n', PAL['success'])
        elif base == 'help':
            self._write('Sovereign Shell Built-ins:\n', PAL['warning'])
            self._write('ls, cd, clear, whoami, htop, help, exit, neofetch\n')
        elif base == 'neofetch':
            self._write('   .---.     OS: SigmaOS Apex Pro 4.0\n', PAL['accent'])
            self._write('  /     \\    Kernel: Sovereign-Loom 5.2\n', PAL['accent'])
            self._write(' | () () |   Shell: SovereignShell 3.0\n', PAL['accent'])
            self._write('  \\  ^  /    CPU: Neural-Cores x128\n', PAL['accent'])
            self._write('   |||||     RAM: 512GB Bit-Safe\n', PAL['accent'])
        else:
            try:
                out = subprocess.check_output(cmd, shell=True, stderr=subprocess.STDOUT, timeout=5).decode()
                self._write(out)
            except Exception as e:
                self._write(f'shell: {cmd}: command not found in this isolated shim.\n', PAL['error'])
