# Generated method: IdentityVisualizer._draw
import tkinter as tk

class IdentityVisualizer:
    def _draw(self):
        if not hasattr(self, 'canvas') or not self.canvas.winfo_exists():
            return
        self.canvas.delete('all')
        a, b = (self.a.get(), self.b.get())
        ox, oy = (50, 50)
        self.canvas.create_rectangle(ox, oy, ox + a, oy + a, fill='#3B82F6', outline='white')
        self.canvas.create_text(ox + a / 2, oy + a / 2, text=f'a²\n({a}x{a})', fill='white')
        self.canvas.create_rectangle(ox + a, oy, ox + a + b, oy + a, fill='#22C55E', outline='white')
        self.canvas.create_text(ox + a + b / 2, oy + a / 2, text=f'ab\n({a}x{b})', fill='white')
        self.canvas.create_rectangle(ox, oy + a, ox + a, oy + a + b, fill='#22C55E', outline='white')
        self.canvas.create_text(ox + a / 2, oy + a + b / 2, text=f'ab\n({b}x{a})', fill='white')
        self.canvas.create_rectangle(ox + a, oy + a, ox + a + b, oy + a + b, fill='#EC4899', outline='white')
        self.canvas.create_text(ox + a + b / 2, oy + a + b / 2, text=f'b²\n({b}x{b})', fill='white')
        total = (a + b) ** 2
        if hasattr(self, 'label') and self.label.winfo_exists():
            self.label.config(text=f'(a+b)² = a² + 2ab + b²  =>  ({a}+{b})² = {a}² + 2({a}*{b}) + {b}² = {total}')