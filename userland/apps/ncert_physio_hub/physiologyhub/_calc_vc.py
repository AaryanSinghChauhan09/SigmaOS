# Generated method: PhysiologyHub._calc_vc
import tkinter as tk
from tkinter import ttk, messagebox
import math, random

class PhysiologyHub:
    def _calc_vc(self):
        try:
            vc = int(self.tv.get()) + int(self.irv.get()) + 1000
            self.vc_res.config(text=f'VC: {vc} ml | Inspiratory Capacity: {int(self.tv.get()) + int(self.irv.get())} ml')
        except:
            pass