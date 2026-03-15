# Generated method: WatchTheCup._update_stats
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _update_stats(self):
        self.lbl_score.config(text=str(self.score))
        self.lbl_streak.config(text=str(self.streak))
        self.lbl_best.config(text=str(self.best_streak))
        self.lbl_round.config(text=str(self.round_n))