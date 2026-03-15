# Generated method: OmniPurge._complete_analysis
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class OmniPurge:
    def _complete_analysis(self, val):
        self.mass_lbl.config(text=f'{val:.2f} GB')
        self.mass_lbl.config(fg=PAL['accent'])
        self.status.config(text='ANALYSIS COMPLETE | READY FOR PURGE', bg=PAL['success'], fg='black')
        self._log(f'>>> ANALYSIS: {val:.2f} GB of non-essential neural mass mapped.')