# Generated method: AetherPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_LOGO, FONT_MONO

class AetherPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'SOVEREIGN AETHER', 'Hyper-Dynamic Kernel Mutation & Federated AI Mesh')
        self.build()