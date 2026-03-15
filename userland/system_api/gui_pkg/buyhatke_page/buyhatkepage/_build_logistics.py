# Generated method: BuyhatkePage._build_logistics
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage:
    def _build_logistics(self, parent):
        tk.Label(parent, text='EDI Shipment Tracking', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg']).pack(pady=10)