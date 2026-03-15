# Generated method: EmailDisco._sync_threads
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time

class EmailDisco:
    def _sync_threads(self):
        self.tree.delete(*self.tree.get_children())
        items = [('Sovereign-User', 'SigmaOS v4.0 Deployment', 'URGENT'), ('Antigravity', 'Model Quota Exhaustion', 'HIGH'), ('Security Warden', 'Unauthorized Shim Blocked', 'CRITICAL'), ('Board Hub', 'New Game Assets Ready', 'LOW')]
        for it in items:
            self.tree.insert('', 'end', values=it)