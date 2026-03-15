# Generated method: ShoppingWizardPage._coupons
import tkinter as tk
from tkinter import ttk
import random
import webbrowser
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_TITLE

class ShoppingWizardPage:
    def _coupons(self):
        engine = self.kernel.registry.get('buyhatke')
        if engine:
            cs = engine.find_coupons('Global')
            self._notify('Coupon Discovery', f"Verified Coupons Found: {', '.join(cs)}", 'OK')