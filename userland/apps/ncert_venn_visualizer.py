"""
SigmaOS NCERT Venn Diagram Visualizer v1.0
Set Theory (Union, Intersection) for Classes 11-12
100% stdlib/tkinter
"""
import tkinter as tk

class VennVisualizer(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • Venn Diagram Visualizer")
        self.geometry("800x600")
        self.configure(bg="#0D0F18")
        
        self.set_a = tk.StringVar(value="1, 2, 3, 4")
        self.set_b = tk.StringVar(value="3, 4, 5, 6")
        
        self._build_ui()
        self._draw()

    def _build_ui(self):
        ctrl = tk.Frame(self, bg="#11142A", pady=20)
        ctrl.pack(fill="x")
        
        tk.Label(ctrl, text="Set A:", fg="white", bg="#11142A").pack(side="left", padx=10)
        tk.Entry(ctrl, textvariable=self.set_a, width=20).pack(side="left", padx=5)
        
        tk.Label(ctrl, text="Set B:", fg="white", bg="#11142A").pack(side="left", padx=10)
        tk.Entry(ctrl, textvariable=self.set_b, width=20).pack(side="left", padx=5)
        
        tk.Button(ctrl, text="UPDATE", command=self._draw, bg="#6C63FF", fg="white", relief="flat").pack(side="left", padx=20)

        self.canvas = tk.Canvas(self, bg="#0D0F18", highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, padx=40, pady=40)

        self.info = tk.Label(self, text="", fg="#E8E8F0", bg="#0D0F18", font=("Segoe UI", 11), pady=20)
        self.info.pack()

    def _draw(self):
        self.canvas.delete("all")
        try:
            a = set(x.strip() for x in self.set_a.get().split(",") if x.strip())
            b = set(x.strip() for x in self.set_b.get().split(",") if x.strip())
        except: return

        common = a & b
        only_a = a - b
        only_b = b - a

        # Draw Circles (Simplified)
        self.canvas.create_oval(150, 100, 450, 400, outline="#3B82F6", width=3)
        self.canvas.create_oval(350, 100, 650, 400, outline="#EC4899", width=3)
        
        # Labels
        self.canvas.create_text(300, 250, text="\n".join(list(only_a)), fill="#3B82F6", font=("Segoe UI Bold", 10))
        self.canvas.create_text(500, 250, text="\n".join(list(only_b)), fill="#EC4899", font=("Segoe UI Bold", 10))
        self.canvas.create_text(400, 250, text="\n".join(list(common)), fill="white", font=("Segoe UI Bold", 10))
        
        self.canvas.create_text(250, 80, text="Set A", fill="#3B82F6", font=("Segoe UI Bold", 12))
        self.canvas.create_text(550, 80, text="Set B", fill="#EC4899", font=("Segoe UI Bold", 12))

        self.info.config(text=f"A ∪ B: {{{', '.join(sorted(list(a|b)))}}} | A ∩ B: {{{', '.join(sorted(list(common)))}}}")

if __name__ == "__main__":
    VennVisualizer().mainloop()
