"""
Auto-split from userland\apps\shell_game.py — WatchTheCup._begin_shuffle
"""

import tkinter as tk
from tkinter import messagebox
import random
import time



class WatchTheCup:
    def _begin_shuffle(self, cfg):
        self.phase = 'shuffling'
        self._draw_scene(show_coin=False)
        self.lbl_instr.config(text='🔀  Shuffling…', fg=PAL['dim'])
        self._do_swaps(cfg['swaps'], cfg['delay'], cfg)
