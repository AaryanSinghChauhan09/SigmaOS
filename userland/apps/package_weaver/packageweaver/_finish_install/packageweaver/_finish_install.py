# Generated method: PackageWeaver._finish_install
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class PackageWeaver:
    def _finish_install(self, item, val):
        val[3] = 'Installed'
        self.tree.item(item, values=val)
        self.status.config(text=f'{val[0]} INSTALLED AND SANDBOXED UNDER ZERO-TRUST.', bg=PAL['success'], fg='black')