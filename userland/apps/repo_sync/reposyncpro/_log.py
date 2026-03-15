# Generated method: RepoSyncPro._log
import tkinter as tk
from tkinter import ttk, messagebox
import subprocess
import threading
import os

class RepoSyncPro:
    def _log(self, msg):
        self.terminal.config(state=tk.NORMAL)
        self.terminal.insert(tk.END, f'>>> {msg}\n')
        self.terminal.see(tk.END)
        self.terminal.config(state=tk.DISABLED)