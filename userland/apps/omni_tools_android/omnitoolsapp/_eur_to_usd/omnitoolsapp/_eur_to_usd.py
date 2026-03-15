# Generated method: OmniToolsApp._eur_to_usd
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _eur_to_usd(self) -> None:
        try:
            self.usd_entry.delete(0, tk.END)
            self.usd_entry.insert(0, fmt(float(self.eur_entry.get()) / 0.92))
        except ValueError:
            messagebox.showinfo('Currency', 'Enter a valid amount.')