# Generated method: SpotItGame._correct
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

class SpotItGame:
    def _correct(self):
        self.running = False
        if self._tick_id:
            self.after_cancel(self._tick_id)
        level = LEVELS[self.level_var.get()]
        time_bonus = max(0, self.time_left)
        self.combo += 1
        self.best_combo = max(self.best_combo, self.combo)
        pts = (10 + time_bonus * 2) * level['bonus'] * self.combo
        self.score += pts
        self.canvas.create_rectangle(0, 0, self.CANVAS_W, self.CANVAS_H, fill=PAL['success'], stipple='gray25', outline='', tags='flash')
        self.canvas.create_text(self.CANVAS_W // 2, self.CANVAS_H // 2, text=f'✅  FOUND IT!  +{pts} pts  🔥 ×{self.combo}', font=('Segoe UI', 22, 'bold'), fill='white', tags='flash')
        self._update_stats()
        self.status.config(text=f'✅ Correct! +{pts} pts | Combo: ×{self.combo}', bg=PAL['success'])
        self.after(1200, self._round_end)