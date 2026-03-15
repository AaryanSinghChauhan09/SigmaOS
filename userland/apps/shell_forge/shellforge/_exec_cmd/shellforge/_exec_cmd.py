# Generated method: ShellForge._exec_cmd
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ShellForge:
    def _exec_cmd(self, event):
        cmd = self.cmd_entry.get().strip()
        if not cmd:
            return
        self.cmd_history.append(cmd)
        self.cmd_entry.delete(0, tk.END)
        self.term_output.config(state=tk.NORMAL)
        self.term_output.insert(tk.END, f'\nsovereign@apex ❯ {cmd}\n')
        if cmd.startswith('ls'):
            self.term_output.insert(tk.END, '  📁 sigma_core  📁 userland  📁 kernel  📄 README.md  📄 sigma_cli.py\n', 'success')
        elif cmd.startswith('pwd'):
            self.term_output.insert(tk.END, '/home/sigmauser/SigmaOS\n')
        elif cmd.startswith('htop'):
            self.term_output.insert(tk.END, '[htop redirected to Nexus Monitor]\n', 'dim')
        elif cmd.startswith('git'):
            self.term_output.insert(tk.END, f'On branch master. 0 files modified.\n', 'success')
        elif cmd.startswith('echo'):
            self.term_output.insert(tk.END, cmd.replace('echo', '').strip() + '\n')
        elif cmd == 'exit':
            self.term_output.insert(tk.END, 'Session terminated.\n', 'danger')
        else:
            self.term_output.insert(tk.END, f"sigma: command '{cmd}' executed via Neural Resolver.\n", 'dim')
        self.term_output.tag_config('success', foreground=PAL['success'])
        self.term_output.tag_config('dim', foreground=PAL['dim'])
        self.term_output.tag_config('danger', foreground=PAL['danger'])
        self.term_output.see(tk.END)
        self.term_output.config(state=tk.DISABLED)