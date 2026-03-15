# Generated method: WatchTheCup._resolve
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _resolve(self, chosen):
        self.phase = 'result'
        cfg = SPEEDS[self.difficulty]
        win = chosen == self.coin_pos
        if win:
            self.streak += 1
            self.best_streak = max(self.best_streak, self.streak)
            pts = 10 * cfg['bonus'] * (1 + self.streak // 3)
            self.score += pts
            self.lbl_instr.config(text=f'✅  Correct! +{pts} points  🔥×{self.streak}', fg=PAL['success'])
            self.status.config(text=f'✅ Correct! Streak: {self.streak}', bg=PAL['success'])
        else:
            self.streak = 0
            self.lbl_instr.config(text=f'❌  Wrong!  The coin was under cup {self.coin_pos + 1}.', fg=PAL['danger'])
            self.status.config(text=f'❌ Wrong! It was cup {self.coin_pos + 1}.', bg=PAL['danger'])
        self._draw_scene(show_coin=True, result_cup=chosen)
        self._update_stats()
        self.after(2000, self._round_done)