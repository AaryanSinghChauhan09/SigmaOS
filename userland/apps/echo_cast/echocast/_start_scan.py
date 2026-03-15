# Generated method: EchoCast._start_scan
import tkinter as tk
from tkinter import ttk, messagebox
import threading
import time
import random

class EchoCast:
    def _start_scan(self):
        if self.scanning:
            return
        self.scanning = True
        self.canvas.delete('all')
        self.status.config(text='BROADCASTING SUB-ETHER PING...', bg=PAL['warning'], fg='black')

        def animate_sonar(r):
            if not self.scanning:
                return
            self.canvas.delete('sonar')
            self.canvas.create_oval(250 - r, 150 - r, 250 + r, 150 + r, outline=PAL['accent'], width=2, tags='sonar')
            if r < 300:
                self.after(50, lambda: animate_sonar(r + 10))
            else:
                self.scanning = False
                self._show_nodes()
        animate_sonar(10)