# Generated method: TitrationSim._add_drop
import tkinter as tk
from tkinter import messagebox
import random

class TitrationSim:
    def _add_drop(self):
        if self.is_done:
            return
        self.vol_added += 0.1
        self._update_all()