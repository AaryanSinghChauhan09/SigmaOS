# Generated method: SpotItGame._update_stats
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

class SpotItGame:
    def _update_stats(self):
        self.lbl_score.config(text=str(self.score))
        self.lbl_combo.config(text=f'×{self.combo}')
        self.lbl_best.config(text=f'×{self.best_combo}')
        self.lbl_round.config(text=str(self.round_n))