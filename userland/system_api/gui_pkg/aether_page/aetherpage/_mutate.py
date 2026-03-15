# Generated method: AetherPage._mutate
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_LOGO, FONT_MONO

class AetherPage:
    def _mutate(self):
        new_id = self.kernel.mutate_kernel_state()
        self.gui._notify('AETHER', f'Kernel layout mutated: {new_id}', 'OK')
        self.gui._show_page('aether')