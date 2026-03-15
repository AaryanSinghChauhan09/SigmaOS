# Generated method: TitrationSim._draw_flask
import tkinter as tk
from tkinter import messagebox
import random

class TitrationSim:
    def _draw_flask(self):
        self.canvas.delete('liquid')
        self.canvas.create_rectangle(380, 50, 420, 300, outline='white', width=2)
        high = 250 - self.vol_added * 5
        if high > 50:
            self.canvas.create_rectangle(382, high, 418, 298, fill=PAL['acid'], tags='liquid')
        self.canvas.create_polygon(350, 600, 450, 600, 420, 500, 380, 500, outline='white', fill='', width=2)
        color = PAL['indicator_b']
        endpoint = self.base_conc * self.vol_in_flask / self.acid_conc
        if self.vol_added >= endpoint:
            color = '#F0F0F0'
            if not self.is_done:
                self.is_done = True
                messagebox.showinfo('Endpoint!', f'Reaction Complete!\nVolume used: {self.vol_added:.2f} ml\nCalculated Base Molarity: {self.base_conc}')
        self.canvas.create_polygon(360, 598, 440, 598, 430, 550, 370, 550, fill=color, tags='liquid')