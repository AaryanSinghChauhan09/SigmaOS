# Generated method: SystemProfiler._update_telemetry
import tkinter as tk
from tkinter import ttk, messagebox
import platform
import random

class SystemProfiler:
    def _update_telemetry(self):
        self.cpu_usage = random.randint(15, 85)
        self.ram_usage = random.randint(30, 92)
        self.cpu_panel.val_lbl.config(text=f'{self.cpu_usage}%')
        self.cpu_panel.pbar['value'] = self.cpu_usage
        self.ram_panel.val_lbl.config(text=f'{self.ram_usage}%')
        self.ram_panel.pbar['value'] = self.ram_usage
        if self.cpu_usage > 75:
            self.cpu_panel.val_lbl.config(fg=PAL['danger'])
            self.status.config(text='WARNING: HIGH CPU THERMALS DETECTED', bg=PAL['danger'])
        else:
            self.cpu_panel.val_lbl.config(fg=PAL['success'])
            self.status.config(text='TELEMETRY FEED ACTIVE | NOMINAL OPERATION', bg=PAL['accent_dim'])
        self.after(2000, self._update_telemetry)