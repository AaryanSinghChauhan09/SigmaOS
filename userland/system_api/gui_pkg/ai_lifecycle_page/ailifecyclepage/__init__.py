# Generated method: AILifecyclePage.__init__
import tkinter as tk
from tkinter import ttk
import time
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class AILifecyclePage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'AI MISSION CONTROL', 'Unified Alpha-Zero Lifecycle Engineering Studio')
        self.build()