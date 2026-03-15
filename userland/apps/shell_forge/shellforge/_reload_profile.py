"""
Auto-split from userland\apps\shell_forge.py — ShellForge._reload_profile
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class ShellForge:
    def _reload_profile(self):
        self.status.config(text='RELOADING SHELL PROFILE (source ~/.zshrc)...', bg=PAL['warning'], fg='black')
        self.after(800, lambda: self.status.config(text='PROFILE RELOADED | ALL ALIASES & PLUGINS ACTIVE', bg=PAL['success'], fg='black'))
