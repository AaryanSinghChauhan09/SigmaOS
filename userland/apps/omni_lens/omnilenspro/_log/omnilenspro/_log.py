# Generated method: OmniLensPro._log
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import time
import threading

class OmniLensPro:
    def _log(self, msg):
        self.res_text.config(state=tk.NORMAL)
        self.res_text.insert(tk.END, f'{msg}\n')
        self.res_text.see(tk.END)
        self.res_text.config(state=tk.DISABLED)