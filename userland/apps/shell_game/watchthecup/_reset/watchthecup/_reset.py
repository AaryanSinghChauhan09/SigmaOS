# Generated method: WatchTheCup._reset
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _reset(self):
        if self.phase in ('reveal', 'shuffling'):
            return
        self.score = self.streak = self.best_streak = self.round_n = 0
        self._update_stats()
        self.phase = 'idle'
        self.cup_xs = list(self.CUP_XS)
        self._draw_scene(show_coin=True)
        self.lbl_instr.config(text='Score reset. Press ▶ PLAY.', fg=PAL['text'])
        self.status.config(text='Score reset.', bg=PAL['accent2'])