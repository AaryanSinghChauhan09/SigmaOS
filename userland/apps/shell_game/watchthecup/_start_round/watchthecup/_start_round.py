# Generated method: WatchTheCup._start_round
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _start_round(self):
        if self.phase in ('reveal', 'shuffling'):
            return
        self.round_n += 1
        self.coin_pos = random.randint(0, 2)
        self.cup_xs = list(self.CUP_XS)
        self.phase = 'reveal'
        self.coin_shown = True
        self.btn_play.config(state='disabled')
        self.lbl_instr.config(text='👀  Watch carefully — remember the coin!', fg=PAL['accent'])
        self._draw_scene(show_coin=True)
        self.lbl_round.config(text=str(self.round_n))
        cfg = SPEEDS[self.difficulty]
        self.after(1200, lambda: self._begin_shuffle(cfg))