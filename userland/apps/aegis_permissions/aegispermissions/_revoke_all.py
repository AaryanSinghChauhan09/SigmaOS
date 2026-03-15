# Generated method: AegisPermissions._revoke_all
import tkinter as tk
from tkinter import ttk, messagebox

class AegisPermissions:
    def _revoke_all(self):
        conf = messagebox.askyesno('Global Revoke', 'Instantiate DEFCON 1 Token Revocation?\nThis will disconnect all apps from hardware rings.')
        if conf:
            self.status.config(text='DEFCON 1: ALL TOKENS BURNED. KERNEL IS ISOLATED.', bg=PAL['danger'], fg='white')