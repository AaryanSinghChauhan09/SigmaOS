# Generated method: VennVisualizer.__init__
import tkinter as tk

class VennVisualizer:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • Venn Diagram Visualizer')
        self.geometry('800x600')
        self.configure(bg='#0D0F18')
        self.set_a = tk.StringVar(value='1, 2, 3, 4')
        self.set_b = tk.StringVar(value='3, 4, 5, 6')
        self._build_ui()
        self._draw()