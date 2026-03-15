"""
Auto-split from userland\apps\energy_core.py — EnergyCore._hibernate
"""

import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
import time
import random
from userland.system_api.sigma_std import SigmaSys



class EnergyCore:
    def _hibernate(self):
        res = messagebox.askyesno('Deep Sleep', 'Engage neural hibernation protocols? All state vectors will be frozen to NVMe.')
        if res:
            self.status.config(text='HIBERNATION ENGAGED...', bg=PAL['danger'], fg='white')
