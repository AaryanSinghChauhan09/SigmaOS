# Generated method: OmniToolsApp._m_to_ft
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _m_to_ft(self) -> None:
        try:
            self.feet_entry.delete(0, tk.END)
            self.feet_entry.insert(0, fmt(float(self.meter_entry.get()) * 3.28084))
        except ValueError:
            messagebox.showinfo('Converter', 'Enter a valid number.')