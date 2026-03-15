# Generated method: TitrationSim._update_all
import tkinter as tk
from tkinter import messagebox
import random

class TitrationSim:
    def _update_all(self):
        self.vol_lbl.config(text=f'Volume Added: {self.vol_added:.2f} ml')
        self._draw_flask()