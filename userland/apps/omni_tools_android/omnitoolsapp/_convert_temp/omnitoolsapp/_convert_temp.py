# Generated method: OmniToolsApp._convert_temp
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _convert_temp(self) -> None:
        try:
            v = float(self.temp_in.get())
            unit = self.temp_unit.get()
            mapping = {'C→F': v * 9 / 5 + 32, 'F→C': (v - 32) * 5 / 9, 'C→K': v + 273.15, 'K→C': v - 273.15}
            self.temp_out.config(text=f'Result: {fmt(mapping[unit])} ({unit})')
        except (ValueError, KeyError):
            self.temp_out.config(text='Error: invalid input')