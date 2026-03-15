# Generated method: AetherNetMapper._log
import tkinter as tk
from tkinter import ttk, messagebox
import socket
import threading
import random

class AetherNetMapper:
    def _log(self, msg, color=PAL['success']):
        self.terminal.config(state=tk.NORMAL)
        self.terminal.insert(tk.END, f'>>> {msg}\n')
        self.terminal.see(tk.END)
        self.terminal.config(state=tk.DISABLED)