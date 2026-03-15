# Generated method: ForensicVault._log
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import os, hashlib, time, threading

class ForensicVault:
    def _log(self, msg, level='INFO'):
        ts = time.strftime('%H:%M:%S')
        self.console.insert(tk.END, f'[{ts}] [{level}] {msg}\n')
        self.console.see(tk.END)