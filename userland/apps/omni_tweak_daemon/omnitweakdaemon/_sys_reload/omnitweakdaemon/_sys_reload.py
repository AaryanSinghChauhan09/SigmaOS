# Generated method: OmniTweakDaemon._sys_reload
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class OmniTweakDaemon:
    def _sys_reload(self):
        self.status.config(text='RELOADING DAEMON SYMLINKS...', bg=PAL['accent'], fg='white')
        self.after(500, lambda: self.status.config(text='UNIT FILES PARSED AND LOADED TO MEMORY', bg=PAL['success'], fg='black'))