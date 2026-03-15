# Generated method: OmniToolsApp._calc_bmi
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _calc_bmi(self) -> None:
        try:
            w = float(self.bmi_weight.get())
            h = float(self.bmi_height.get()) / 100
            bmi = w / h ** 2
            cat = 'Underweight' if bmi < 18.5 else 'Normal' if bmi < 25 else 'Overweight' if bmi < 30 else 'Obese'
            self.bmi_result.config(text=f'BMI: {fmt(bmi)}  ({cat})', fg=PAL['success'])
        except Exception:
            self.bmi_result.config(text='Error: check inputs', fg=PAL['danger'])