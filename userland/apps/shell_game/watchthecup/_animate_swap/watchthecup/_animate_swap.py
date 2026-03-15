# Generated method: WatchTheCup._animate_swap
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _animate_swap(self, a, b, delay, callback, step=0, steps=8):
        """Smoothly interpolate cup positions."""
        if self._anim_after:
            self.after_cancel(self._anim_after)
        ax0, bx0 = (list(self.CUP_XS)[a], list(self.CUP_XS)[b])
        xa_start = self.cup_xs[a]
        xb_start = self.cup_xs[b]

        def tick(s):
            frac = s / steps
            self.cup_xs[a] = int(xa_start + (xb_start - xa_start) * frac)
            self.cup_xs[b] = int(xb_start + (xa_start - xb_start) * frac)
            self._draw_scene(show_coin=False)
            if s < steps:
                self._anim_after = self.after(delay // steps, lambda: tick(s + 1))
            else:
                self.cup_xs[a] = xb_start
                self.cup_xs[b] = xa_start
                if self.coin_pos == a:
                    self.coin_pos = b
                elif self.coin_pos == b:
                    self.coin_pos = a
                self._draw_scene(show_coin=False)
                self._anim_after = self.after(max(10, delay // 4), callback)
        tick(0)