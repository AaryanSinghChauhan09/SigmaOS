# Generated method: OmniTweakDaemon._toggle_service
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class OmniTweakDaemon:
    def _toggle_service(self, event):
        item = self.daemon_tree.selection()
        if item:
            val = list(self.daemon_tree.item(item, 'values'))
            name = val[0]
            if val[2] == 'active':
                val[2] = 'inactive'
                val[3] = 'dead'
                action = 'systemctl stop'
            else:
                val[2] = 'active'
                val[3] = 'running'
                action = 'systemctl start'
            self.daemon_tree.item(item, values=val)
            self.status.config(text=f'EXECUTED: {action} {name} (PID: {random.randint(1000, 9999)})', bg=PAL['warning'], fg='black')