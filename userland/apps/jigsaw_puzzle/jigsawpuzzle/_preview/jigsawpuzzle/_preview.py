# Generated method: JigsawPuzzle._preview
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _preview(self):
        win = tk.Toplevel(self)
        win.title('Original Image Preview')
        win.configure(bg=PAL['bg'])
        win.geometry('550x580')
        tk.Label(win, text='ORIGINAL IMAGE', font=('Segoe UI', 12, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(pady=(16, 4))
        canvas = tk.Canvas(win, bg=PAL['panel'], highlightthickness=0)
        canvas.pack(fill='both', expand=True, padx=20, pady=10)
        if PIL_AVAILABLE and self.pil_img:
            preview = self.pil_img.resize((500, 500))
            photo = ImageTk.PhotoImage(preview)
            canvas.create_image(250, 250, image=photo, anchor='center')
            win._photo = photo
        else:
            n = self.grid_n
            ts = 100
            pad = 3
            colors = [t['color'] for t in sorted(self.tiles, key=lambda x: x['correct'])]
            for i, color in enumerate(colors):
                row, col = divmod(i, n)
                x1 = 10 + col * (ts + pad)
                y1 = 10 + row * (ts + pad)
                canvas.create_rectangle(x1, y1, x1 + ts, y1 + ts, fill=color, outline=PAL['border'])
                canvas.create_text(x1 + ts // 2, y1 + ts // 2, text=str(i + 1), font=('Segoe UI', 12, 'bold'), fill='white')
        tk.Button(win, text='Close', command=win.destroy, bg=PAL['panel'], fg=PAL['text'], relief='flat', padx=20, pady=6).pack(pady=10)