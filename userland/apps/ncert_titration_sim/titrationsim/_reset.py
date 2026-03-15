# Generated method: TitrationSim._reset
import tkinter as tk
from tkinter import messagebox
import random

class TitrationSim:
    def _reset(self):
        self.vol_added = 0.0
        self.is_done = False
        self.base_conc = _r(random.uniform(0.05, 0.15), 3)
        self._update_all()