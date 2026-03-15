# Generated method: SpotItGame._reset_game
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

class SpotItGame:
    def _reset_game(self):
        self.running = False
        if self._tick_id:
            self.after_cancel(self._tick_id)
        self.score = self.combo = self.best_combo = self.round_n = 0
        self._update_stats()
        self.canvas.delete('all')
        self.tgt_canvas.delete('all')
        self.lbl_tgt_name.config(text='—')
        self.lbl_timer.config(text='—')
        self.timer_bar.delete('all')
        self.canvas.create_text(self.CANVAS_W // 2, self.CANVAS_H // 2, text='Press  ▶ NEW ROUND  to begin!', font=('Segoe UI', 18, 'bold'), fill=PAL['dim'])
        self.btn_play.config(state='normal')
        self.status.config(text='Reset. Pick a level and press ▶ NEW ROUND.', bg=PAL['accent2'])