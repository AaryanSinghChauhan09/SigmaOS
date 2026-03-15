"""
Auto-split from userland\apps\spot_it.py — SpotItGame._round_end
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _round_end(self):
        self.canvas.delete('flash')
        self.running = False
        self.btn_play.config(state='normal')
        self.lbl_timer.config(text='—', fg=PAL['warning'])
        self.timer_bar.delete('all')
