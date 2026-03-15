# Generated method: BuyhatkePage._build_coupons
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage:
    def _build_coupons(self, parent):
        tk.Label(parent, text='Auto-Coupon Discovery', font=FONT_MED, fg=PAL['teal'], bg=PAL['bg']).pack(pady=10)