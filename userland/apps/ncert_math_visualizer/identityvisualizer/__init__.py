# Generated method: IdentityVisualizer.__init__
import tkinter as tk

class IdentityVisualizer:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • Math Identity Visualizer')
        self.geometry('800x700')
        self.configure(bg='#0D0F18')
        self.a = tk.IntVar(value=100)
        self.b = tk.IntVar(value=50)
        self.canvas = tk.Canvas()
        self.label = tk.Label()
        self._build_ui()
        self._draw()