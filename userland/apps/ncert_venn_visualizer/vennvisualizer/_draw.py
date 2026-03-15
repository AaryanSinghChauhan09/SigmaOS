# Generated method: VennVisualizer._draw
import tkinter as tk

class VennVisualizer:
    def _draw(self):
        self.canvas.delete('all')
        try:
            a = set((x.strip() for x in self.set_a.get().split(',') if x.strip()))
            b = set((x.strip() for x in self.set_b.get().split(',') if x.strip()))
        except:
            return
        common = a & b
        only_a = a - b
        only_b = b - a
        self.canvas.create_oval(150, 100, 450, 400, outline='#3B82F6', width=3)
        self.canvas.create_oval(350, 100, 650, 400, outline='#EC4899', width=3)
        self.canvas.create_text(300, 250, text='\n'.join(list(only_a)), fill='#3B82F6', font=('Segoe UI Bold', 10))
        self.canvas.create_text(500, 250, text='\n'.join(list(only_b)), fill='#EC4899', font=('Segoe UI Bold', 10))
        self.canvas.create_text(400, 250, text='\n'.join(list(common)), fill='white', font=('Segoe UI Bold', 10))
        self.canvas.create_text(250, 80, text='Set A', fill='#3B82F6', font=('Segoe UI Bold', 12))
        self.canvas.create_text(550, 80, text='Set B', fill='#EC4899', font=('Segoe UI Bold', 12))
        self.info.config(text=f"A ∪ B: {{{', '.join(sorted(list(a | b)))}}} | A ∩ B: {{{', '.join(sorted(list(common)))}}}")