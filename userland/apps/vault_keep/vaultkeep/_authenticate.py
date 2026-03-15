# Generated method: VaultKeep._authenticate
import tkinter as tk
from tkinter import ttk, messagebox
import random

class VaultKeep:
    def _authenticate(self):
        if not self.auth_state:
            self.status.config(text='VERIFYING NEURAL IMPRINT...', bg=PAL['warning'], fg='black')
            self.after(1500, self._grant_auth)
        else:
            self.status.config(text='VAULT LOCKED | MEMORY SHREEDED', bg=PAL['danger'], fg='white')
            self.auth_state = False