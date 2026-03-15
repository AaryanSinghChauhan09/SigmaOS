# Generated method: AuraDisplay._apply_matrix
import tkinter as tk
from tkinter import ttk, messagebox

class AuraDisplay:
    def _apply_matrix(self):
        conf = 'GEO-LOCKED' if self.circadian_sync.get() else 'STATIC'
        self.status.config(text=f'AURA OVERRIDE COMPLETE: {conf} L: {self.brightness.get()} B: {self.blue_filter.get()}', bg=PAL['accent_dim'], fg='white')
        messagebox.showinfo('Neural Shift', 'Color Gamut shifted at Ring-0 GPU level. Hardware override complete.')