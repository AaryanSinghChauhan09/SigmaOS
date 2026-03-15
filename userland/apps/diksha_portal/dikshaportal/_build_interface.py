# Generated method: DikshaPortal._build_interface
import tkinter as tk
from tkinter import ttk, messagebox
import json

class DikshaPortal:
    def _build_interface(self):
        head = tk.Frame(self, bg=self.styles['accent'], height=100)
        head.pack(fill='x')
        tk.Label(head, text='DIKSHA DIGITAL LEARNING', font=('Segoe UI', 20, 'bold'), fg='white', bg=self.styles['accent']).pack(pady=25)
        main = tk.Frame(self, bg=self.styles['bg'], padx=40, pady=40)
        main.pack(fill='both', expand=True)
        cat_fr = tk.Frame(main, bg=self.styles['bg'])
        cat_fr.pack(fill='x', pady=20)
        cats = [('Digital Textbooks', 'Access 1-12 NCERT Library'), ('Smart QR Linker', 'Scan simulation codes'), ('Teacher Hub', 'Resources & Lesson Plans'), ('Student Progress', 'Performance Analytics')]
        for name, desc in cats:
            c = tk.Frame(main, bg=self.styles['card'], padx=20, pady=20, highlightthickness=1, highlightbackground='#312E81')
            c.pack(fill='x', pady=10)
            tk.Label(c, text=name, font=('Segoe UI Bold', 14), fg=self.styles['accent'], bg=self.styles['card']).pack(anchor='w')
            tk.Label(c, text=desc, font=('Segoe UI', 10), fg='#94A3B8', bg=self.styles['card']).pack(anchor='w')
            tk.Button(c, text='OPEN MODULE', bg=self.styles['accent'], fg='white', relief='flat', padx=15, command=lambda n=name: self._launch_feature(n)).pack(side='right', pady=(-40, 0))