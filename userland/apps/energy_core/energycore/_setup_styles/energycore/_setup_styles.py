# Generated method: EnergyCore._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
import time
import random
from userland.system_api.sigma_std import SigmaSys

class EnergyCore:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Energy.Horizontal.TProgressbar', background=PAL['accent'], troughcolor=PAL['sidebar'], borderwidth=0)