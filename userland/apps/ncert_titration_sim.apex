"""
SigmaOS NCERT Titration Simulator v1.0
======================================
Interactive Acid-Base Titration Lab for Classes 11-12
100% Native Python | Standard Library | Sovereign Aesthetics
"""
import tkinter as tk
from tkinter import messagebox
import random

PAL = {
    "bg": "#05060A",
    "panel": "#0E111A",
    "acid": "#FF3B30",
    "base": "#007AFF",
    "indicator_a": "#FFFFFF", # Colorless
    "indicator_b": "#FF8AD8", # Pink (Phenolphthalein)
    "text": "#E2E8F0",
    "accent": "#6C63FF"
}

class TitrationSim(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • Virtual Titration Lab")
        self.geometry("800x700")
        self.configure(bg=PAL["bg"])
        
        self.acid_conc = 0.1
        self.base_conc = _r(random.uniform(0.05, 0.15), 3) # Target to find
        self.vol_in_flask = 20.0
        self.vol_added = 0.0
        self.is_done = False
        
        self.vol_lbl: tk.Label = tk.Label()
        self.canvas: tk.Canvas = tk.Canvas()
        
        self._build_ui()

    def _build_ui(self):
        # Layout
        side = tk.Frame(self, bg=PAL["panel"], width=250)
        side.pack(side="left", fill="y")
        side.pack_propagate(False)
        
        tk.Label(side, text="TITRATION CONTROL", font=("Segoe UI Bold", 12), fg=PAL["accent"], bg=PAL["panel"]).pack(pady=20)
        
        tk.Label(side, text=f"Flask: 20ml Base (Unknown M)", fg=PAL["text"], bg=PAL["panel"]).pack(pady=5)
        tk.Label(side, text=f"Burette: 0.1M HCl (Acid)", fg=PAL["text"], bg=PAL["panel"]).pack(pady=5)
        
        self.vol_lbl = tk.Label(side, text="Volume Added: 0.00 ml", font=("Consolas", 12), fg=PAL["base"], bg=PAL["panel"])
        self.vol_lbl.pack(pady=30)
        
        tk.Button(side, text="ADD DROP (0.1 ml)", command=self._add_drop, bg="#1A1E30", fg="white", relief="flat", padx=20, pady=10).pack(fill="x", padx=20, pady=5)
        tk.Button(side, text="FAST POUR (1.0 ml)", command=self._pour, bg="#1A1E30", fg="white", relief="flat", padx=20, pady=10).pack(fill="x", padx=20, pady=5)
        tk.Button(side, text="RESET LAB", command=self._reset, bg=PAL["acid"], fg="white", relief="flat", padx=20, pady=10).pack(fill="x", padx=20, pady=40)

        # Visual Area
        self.canvas = tk.Canvas(self, bg=PAL["bg"], highlightthickness=0)
        self.canvas.pack(side="right", fill="both", expand=True)
        self._draw_flask()

    def _draw_flask(self):
        self.canvas.delete("liquid")
        # Burette
        self.canvas.create_rectangle(380, 50, 420, 300, outline="white", width=2)
        # Burette Liquid
        high = 250 - (self.vol_added * 5)
        if high > 50:
            self.canvas.create_rectangle(382, high, 418, 298, fill=PAL["acid"], tags="liquid")
        
        # Flask
        self.canvas.create_polygon(350, 600, 450, 600, 420, 500, 380, 500, outline="white", fill="", width=2)
        
        # Flask Liquid (Color change logic)
        color = PAL["indicator_b"] # Starts Basic (Pink)
        endpoint = (self.base_conc * self.vol_in_flask) / self.acid_conc
        
        if self.vol_added >= endpoint:
            color = "#F0F0F0" # Neutral/Acidic (Colorless/White)
            if not self.is_done:
                self.is_done = True
                messagebox.showinfo("Endpoint!", f"Reaction Complete!\nVolume used: {self.vol_added:.2f} ml\nCalculated Base Molarity: {self.base_conc}")

        self.canvas.create_polygon(360, 598, 440, 598, 430, 550, 370, 550, fill=color, tags="liquid")

    def _add_drop(self):
        if self.is_done: return
        self.vol_added += 0.1
        self._update_all()

    def _pour(self):
        if self.is_done: return
        self.vol_added += 1.0
        self._update_all()

    def _update_all(self):
        self.vol_lbl.config(text=f"Volume Added: {self.vol_added:.2f} ml")
        self._draw_flask()

    def _reset(self):
        self.vol_added = 0.0
        self.is_done = False
        self.base_conc = _r(random.uniform(0.05, 0.15), 3)
        self._update_all()

def _r(x, d=4):
    return float(("{:." + str(d) + "f}").format(x))

if __name__ == "__main__":
    TitrationSim().mainloop()
