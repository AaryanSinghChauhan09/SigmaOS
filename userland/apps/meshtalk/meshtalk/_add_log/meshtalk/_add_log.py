# Generated method: MeshTalk._add_log
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time

class MeshTalk:
    def _add_log(self, author, msg, color='#E8E8E8'):
        self.chat_log.config(state='normal')
        ts = time.strftime('%H:%M')
        self.chat_log.insert(tk.END, f'[{ts}] ', 'dim')
        self.chat_log.insert(tk.END, f'{author}: ', author)
        self.chat_log.insert(tk.END, f'{msg}\n\n')
        self.chat_log.tag_configure(author, foreground=color, font=('Inter Bold', 11))
        self.chat_log.tag_configure('dim', foreground=PAL['dim'], font=('Inter', 9))
        self.chat_log.config(state='disabled')
        self.chat_log.see(tk.END)