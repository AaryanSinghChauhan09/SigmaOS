"""
SigmaOS NCERT Graphing Utility v1.0
Scientific Plotting Tool for Classes 9-12
100% stdlib/tkinter
"""
import tkinter as tk
from tkinter import ttk
import math

class NCERTGrapher(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • NCERT Dynamic Grapher")
        self.geometry("1000x700")
        self.configure(bg="#0B0D17")
        
        self.func_var = tk.StringVar(value="sin(x)")
        self.range_var = tk.DoubleVar(value=10.0)
        
        # Initialize attributes for linter
        self.canvas = tk.Canvas()
        
        self._build_ui()
        self.plot()

    def _build_ui(self):
        hdr = tk.Frame(self, bg="#11142A", height=60)
        hdr.pack(fill="x")
        tk.Label(hdr, text="📈 SCIENTIFIC GRAPHING UTILITY", fg="#3B82F6", bg="#11142A", font=("Segoe UI Bold", 14)).pack(pady=15)

        ctrl = tk.Frame(self, bg="#11142A", pady=10)
        ctrl.pack(fill="x")
        
        tk.Label(ctrl, text="f(x) = ", fg="white", bg="#11142A", font=("Consolas", 12)).pack(side="left", padx=(20, 0))
        ent = tk.Entry(ctrl, textvariable=self.func_var, bg="#1A1E30", fg="white", font=("Consolas", 12), width=30, relief="flat", insertbackground="white")
        ent.pack(side="left", padx=10)
        ent.bind("<Return>", lambda e: self.plot())

        tk.Label(ctrl, text="Range ±", fg="white", bg="#11142A").pack(side="left", padx=10)
        tk.Scale(ctrl, from_=1, to=100, variable=self.range_var, orient="horizontal", command=lambda x: self.plot(), bg="#11142A", fg="white", highlightthickness=0).pack(side="left", padx=5)

        tk.Button(ctrl, text="PLOT GRAPH", command=self.plot, bg="#6C63FF", fg="white", relief="flat", padx=15).pack(side="left", padx=20)

        self.canvas.destroy()
        self.canvas = tk.Canvas(self, bg="#00050A", highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, padx=20, pady=20)

    def plot(self):
        if not hasattr(self, 'canvas') or not self.canvas.winfo_exists(): return
        self.canvas.delete("all")
        w = self.canvas.winfo_width()
        h = self.canvas.winfo_height()
        if w < 100: w, h = 960, 500 # Fallback for initial load
        
        cx, cy = w/2, h/2
        self.canvas.create_line(0, cy, w, cy, fill="#252840") # X axis
        self.canvas.create_line(cx, 0, cx, h, fill="#252840") # Y axis
        
        rng = self.range_var.get()
        scale_x = cx / rng
        scale_y = cy / (rng/2) # Aspect ratio tweak
        
        expr = self.func_var.get().replace("^", "**")
        points = []
        
        # Safe eval environment
        safe_dict = {
            "x": 0, "sin": math.sin, "cos": math.cos, "tan": math.tan,
            "exp": math.exp, "log": math.log, "sqrt": math.sqrt, "pi": math.pi, "e": math.e
        }
        
        step = rng / 200
        for i in range(-200, 201):
            x = i * step
            safe_dict["x"] = x
            try:
                y = eval(expr, {"__builtins__": None}, safe_dict)
                px = cx + x * scale_x
                py = cy - y * scale_y
                if 0 <= px <= w and 0 <= py <= h:
                    points.append((px, py))
            except:
                continue
                
        if len(points) > 1:
            self.canvas.create_line(points, fill="#00D26A", width=2, smooth=True)
            
        # Draw labels
        self.canvas.create_text(w-20, cy+15, text="x", fill="white")
        self.canvas.create_text(cx+15, 20, text="y", fill="white")

if __name__ == "__main__":
    NCERTGrapher().mainloop()
