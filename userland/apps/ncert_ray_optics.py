"""
SigmaOS NCERT Ray Optics Bench v1.0
===================================
Visual Mirror/Lens Formula Simulator for Class 12 Physics
100% Native Python | Standard Library | Sovereign Aesthetics
"""
import tkinter as tk
from tkinter import ttk

PAL = {
    "bg": "#0A0B10",
    "optics": "#121420",
    "ray": "#00D26A",
    "object": "#FF3B30",
    "image": "#007AFF",
    "text": "#E2E8F0",
    "accent": "#F59E0B"
}

class OpticsBench(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • Ray Optics Virtual Bench")
        self.geometry("1000x800")
        self.configure(bg=PAL["bg"])
        
        self.mode = tk.StringVar(value="Concave Mirror")
        self.focal_len = 20.0
        self.obj_dist = 40.0 # 'u'
        
        self._build_ui()

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg=PAL["optics"], height=70)
        hdr.pack(fill="x")
        tk.Label(hdr, text="🔭 VIRTUAL OPTICS BENCH", font=("Segoe UI Bold", 18), fg=PAL["accent"], bg=PAL["optics"]).pack(pady=15)

        # Control Panel
        ctrl = tk.Frame(self, bg=PAL["bg"], padx=40, pady=20)
        ctrl.pack(fill="x")

        tk.Label(ctrl, text="Selection:", fg=PAL["text"], bg=PAL["bg"]).grid(row=0, column=0, padx=10)
        mode_cb = ttk.Combobox(ctrl, textvariable=self.mode, values=["Concave Mirror", "Convex Mirror", "Convex Lens", "Concave Lens"], state="readonly")
        mode_cb.grid(row=0, column=1, padx=10)
        mode_cb.bind("<<ComboboxSelected>>", lambda e: self._update())

        tk.Label(ctrl, text="Focal Length (f):", fg=PAL["text"], bg=PAL["bg"]).grid(row=0, column=2, padx=10)
        self.f_scale = tk.Scale(ctrl, from_=10, to=50, orient="horizontal", bg=PAL["bg"], fg="white", highlightthickness=0, command=lambda x: self._update())
        self.f_scale.set(20)
        self.f_scale.grid(row=0, column=3, padx=10)

        tk.Label(ctrl, text="Object Distance (u):", fg=PAL["text"], bg=PAL["bg"]).grid(row=0, column=4, padx=10)
        self.u_scale = tk.Scale(ctrl, from_=5, to=150, orient="horizontal", bg=PAL["bg"], fg="white", highlightthickness=0, command=lambda x: self._update())
        self.u_scale.set(40)
        self.u_scale.grid(row=0, column=5, padx=10)

        # Output Display
        self.out_lbl = tk.Label(self, text="IMAGE DATA: Wait...", font=("Consolas", 11), fg=PAL["ray"], bg=PAL["bg"])
        self.out_lbl.pack(pady=10)

        # Bench View
        self.canvas = tk.Canvas(self, bg="#050508", highlightthickness=1, highlightbackground="#1A1C25")
        self.canvas.pack(fill="both", expand=True, padx=40, pady=20)
        
        self._update()

    def _update(self):
        mode = self.mode.get()
        f = float(self.f_scale.get())
        u = -float(self.u_scale.get()) # Cartesian sign convention
        
        # Mirror/Lens formulas
        # 1/v + 1/u = 1/f (Mirror) -> 1/v = 1/f - 1/u
        # 1/v - 1/u = 1/f (Lens) -> 1/v = 1/f + 1/u
        
        if "Mirror" in mode:
            actual_f = -f if "Concave" in mode else f
            try:
                v = 1 / ( (1/actual_f) - (1/u) )
                m = -v/u
            except ZeroDivisionError: v = float('inf'); m = 0
        else: # Lens
            actual_f = f if "Convex" in mode else -f
            try:
                v = 1 / ( (1/actual_f) + (1/u) )
                m = v/u
            except ZeroDivisionError: v = float('inf'); m = 0

        nature = "REAL" if v > 0 or ("Mirror" in mode and v < 0) else "VIRTUAL"
        # Complex logic for nature depending on device type... simplified:
        if "Mirror" in mode:
            nature = "REAL" if v < 0 else "VIRTUAL"
            pos = "In front" if v < 0 else "Behind"
        else:
            nature = "REAL" if v > 0 else "VIRTUAL"
            pos = "Other side" if v > 0 else "Same side"

        self.out_lbl.config(text=f"Image dist (v): {abs(v):.2f} units | Magnification: {m:.2f} | Nature: {nature} ({pos})")
        self._draw_bench(u, v, f, mode)

    def _draw_bench(self, u, v, f, mode):
        self.canvas.delete("all")
        cx, cy = 500, 250
        self.canvas.create_line(50, cy, 950, cy, fill="#333", dash=(4,4)) # Principal Axis
        
        # Center Line
        self.canvas.create_line(cx, cy-100, cx, cy+100, fill="white", width=2)
        self.canvas.create_text(cx, cy+120, text=mode, fill="white")

        # Object (u is already -ve, let's scale it)
        ox = cx + u * 4
        self.canvas.create_line(ox, cy, ox, cy-50, fill=PAL["object"], width=4, arrow=tk.LAST)
        self.canvas.create_text(ox, cy+20, text="OBJECT", fill=PAL["object"])

        # Image
        if abs(v) < 1000:
            scale_factor = 4 if "Lens" in mode else -4
            ix = cx + v * scale_factor
            val_h = 50 * (v/u if "Lens" in mode else -v/u)
            self.canvas.create_line(ix, cy, ix, cy+val_h, fill=PAL["image"], width=4, arrow=tk.LAST)
            self.canvas.create_text(ix, cy+70 if val_h > 0 else cy-70, text="IMAGE", fill=PAL["image"])

if __name__ == "__main__":
    OpticsBench().mainloop()
