# Generated method: VaultKeep._grant_auth
import tkinter as tk
from tkinter import ttk, messagebox
import random

class VaultKeep:
    def _grant_auth(self):
        self.auth_state = True
        self.status.config(text='BIOMETRICS CONFIRMED | VAULT UNLOCKED (30s TEMPORAL PASS)', bg=PAL['success'], fg='black')