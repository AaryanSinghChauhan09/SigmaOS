# Generated method: TitrationSim._pour
import tkinter as tk
from tkinter import messagebox
import random

class TitrationSim:
    def _pour(self):
        if self.is_done:
            return
        self.vol_added += 1.0
        self._update_all()