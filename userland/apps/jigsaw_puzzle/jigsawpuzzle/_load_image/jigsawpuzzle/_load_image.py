# Generated method: JigsawPuzzle._load_image
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _load_image(self):
        path = filedialog.askopenfilename(title='Select an image', filetypes=[('Images', '*.png *.jpg *.jpeg *.bmp *.gif *.webp'), ('All files', '*.*')])
        if not path:
            return
        if not PIL_AVAILABLE:
            messagebox.showinfo('Pillow not installed', 'Install Pillow (pip install Pillow) to use custom images.\nRunning in demo mode with colored tiles instead.')
            return
        try:
            self.pil_img = Image.open(path).convert('RGB')
            self.img_path = path
            self._build_image_tiles()
            self._render_tiles()
            self._shuffle()
            self.status.config(text=f'Image loaded: {os.path.basename(path)}  |  Shuffle and solve!', bg=PAL['success'])
        except Exception as e:
            messagebox.showerror('Error', f'Could not load image:\n{e}')