"""
SigmaOS NCERT Math Identity Visualizer v1.0
Visualizing (a+b)^2 = a^2 + b^2 + 2ab
100% stdlib/tkinter
"""
import tkinter as tk

class IdentityVisualizer(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • Math Identity Visualizer")
        self.geometry("800x700")
        self.configure(bg="#0D0F18")
        
        self.a = tk.IntVar(value=100)
        self.b = tk.IntVar(value=50)
        
        self._build_ui()
        self._draw()

    def _build_ui(self):
        ctrl = tk.Frame(self, bg="#13162A", pady=20)
        ctrl.pack(fill="x")
        
        tk.Label(ctrl, text="Length a:", fg="white", bg="#13162A").pack(side="left", padx=10)
        tk.Scale(ctrl, from_=20, to=200, variable=self.a, orient="horizontal", command=lambda x: self._draw(), bg="#13162A", fg="white", highlightthickness=0).pack(side="left", padx=10)
        
        tk.Label(ctrl, text="Length b:", fg="white", bg="#13162A").pack(side="left", padx=10)
        tk.Scale(ctrl, from_=20, to=200, variable=self.b, orient="horizontal", command=lambda x: self._draw(), bg="#13162A", fg="white", highlightthickness=0).pack(side="left", padx=10)

        self.canvas = tk.Canvas(self, bg="#0D0F18", highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, padx=50, pady=50)
        
        self.label = tk.Label(self, text="", font=("Segoe UI Bold", 14), fg="#6C63FF", bg="#0D0F18", pady=20)
        self.label.pack()

    def _draw(self):
        self.canvas.delete("all")
        a, b = self.a.get(), self.b.get()
        ox, oy = 50, 50
        
        # a^2
        self.canvas.create_rectangle(ox, oy, ox+a, oy+a, fill="#3B82F6", outline="white")
        self.canvas.create_text(ox+a/2, oy+a/2, text=f"a²\n({a}x{a})", fill="white")
        
        # ab (right)
        self.canvas.create_rectangle(ox+a, oy, ox+a+b, oy+a, fill="#22C55E", outline="white")
        self.canvas.create_text(ox+a+b/2, oy+a/2, text=f"ab\n({a}x{b})", fill="white")
        
        # ab (bottom)
        self.canvas.create_rectangle(ox, oy+a, ox+a, oy+a+b, fill="#22C55E", outline="white")
        self.canvas.create_text(ox+a/2, oy+a+b/2, text=f"ab\n({b}x{a})", fill="white")
        
        # b^2
        self.canvas.create_rectangle(ox+a, oy+a, ox+a+b, oy+a+b, fill="#EC4899", outline="white")
        self.canvas.create_text(ox+a+b/2, oy+a+b/2, text=f"b²\n({b}x{b})", fill="white")
        
        total = (a+b)**2
        self.label.config(text=f"(a+b)² = a² + 2ab + b²  =>  ({a}+{b})² = {a}² + 2({a}*{b}) + {b}² = {total}")

if __name__ == "__main__":
    IdentityVisualizer().mainloop()
