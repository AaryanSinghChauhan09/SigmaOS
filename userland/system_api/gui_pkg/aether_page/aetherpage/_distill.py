# Generated method: AetherPage._distill
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_LOGO, FONT_MONO

class AetherPage:
    def _distill(self):
        distillator = self.kernel.registry.get('neural_distillator')
        if distillator:
            res = distillator.distill_from_mirrors()
            self.gui._notify('AI MESH', f'Distillation: {res}', 'INFO')
        else:
            self.gui._notify('AI MESH', 'Distillator Offline.', 'ERR')