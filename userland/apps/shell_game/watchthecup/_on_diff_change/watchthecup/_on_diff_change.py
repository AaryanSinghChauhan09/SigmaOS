# Generated method: WatchTheCup._on_diff_change
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _on_diff_change(self):
        self.difficulty = self.diff_var.get()
        self.status.config(text=f'Difficulty set to: {self.difficulty}', bg=PAL['accent2'])