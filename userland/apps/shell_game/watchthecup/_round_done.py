"""
Auto-split from userland\apps\shell_game.py — WatchTheCup._round_done
"""

import tkinter as tk
from tkinter import messagebox
import random
import time



class WatchTheCup:
    def _round_done(self):
        self.phase = 'idle'
        self.btn_play.config(state='normal')
        self.lbl_instr.config(text='Press  ▶ PLAY  for the next round.', fg=PAL['text'])
        self.status.config(text='Ready for next round.', bg=PAL['accent2'])
        self._draw_scene(show_coin=False)
