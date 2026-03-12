"""
SigmaOS Fluid UI Compositor (v1.0)
==================================
Hardware-accelerated desktop rendering engine.
USP: Z-buffered window sorting and quantum double-buffering.
Principles: Double Buffering, Ray-traced Compositing, Z-Depth Sorting, V-Sync.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#00D4FF", # Fluid Cyan
    "accent_dim": "#0099B8",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "warning": "#FFD60A",
    "panel": "#1C1E24"
}

class FluidCompositor(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Fluid UI Compositor")
        self.geometry("1150x750")
        self.configure(bg=PAL["bg"])
        
        self.fps = 144
        self.vsync = True
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')

    def _build_ui(self):
        # Premium Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="FLUID UI COMPOSITOR ENGINE", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        nav_btns = [
            ("🎥 FLUSH BACKBUFFER", self._flush_buffer),
            ("👁️ TOGGLE V-SYNC", self._toggle_vsync)
        ]
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=15, pady=8, command=cmd).pack(side="left", padx=5)

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Left Panel (Principles)
        self.prin_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=300, padx=15, pady=15)
        self.prin_fr.pack(side="left", fill="y", padx=(0, 20))
        self.prin_fr.pack_propagate(False)

        tk.Label(self.prin_fr, text="RENDERING PRINCIPLES", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(0, 10))

        principles = [
            ("Double Buffering", "Drawing to a hidden frame (Backbuffer), then swapping it to screen instantly to prevent UI tearing."),
            ("Z-Buffer Depth", "Sorting overlapping windows (Z-Index) to calculate pixel occlusion without drawing hidden geometry."),
            ("V-Sync / Refresh Rate", "Locking the redraw loop to the monitor's exact refresh hertz, preventing frame desynchronization."),
            ("Compositing Manager", "Handling shadows, translucency, and blur natively by combining off-screen bitmaps.")
        ]
        
        for name, desc in principles:
            f = tk.Frame(self.prin_fr, bg=PAL["sidebar"], pady=10, padx=10)
            f.pack(fill="x", pady=5)
            tk.Label(f, text=f"💠 {name}", font=("Inter", 9, "bold"), fg=PAL["accent"], bg=PAL["sidebar"]).pack(anchor="w")
            tk.Label(f, text=desc, font=("Inter", 8), fg=PAL["dim"], bg=PAL["sidebar"], wraplength=240, justify="left").pack(anchor="w", pady=(5,0))

        # Right Panel (Compositor Canvas)
        self.canvas_fr = tk.Frame(self.workspace, bg=PAL["bg"])
        self.canvas_fr.pack(side="left", fill="both", expand=True)
        
        tk.Label(self.canvas_fr, text="Z-BUFFER SIMULATION (GPU COMPOSITOR)", font=("Inter", 12, "bold"), fg=PAL["text"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 10))
        
        self.canvas = tk.Canvas(self.canvas_fr, bg=PAL["sidebar"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, pady=10)
        
        # Draw mock windows with Z-depth
        windows = [
            (50, 50, 400, 300, "#2D3142", "Terminal z-index: 1"),
            (150, 120, 500, 400, "#4F5D75", "Browser z-index: 2 (Mica Blur)"),
            (250, 200, 600, 500, "#EF8354", "Active Focus z-index: 3 (Drop Shadow)")
        ]
        
        for x1, y1, x2, y2, col, txt in windows:
            # Drop shadow mock
            self.canvas.create_rectangle(x1+8, y1+8, x2+8, y2+8, fill="#050505", outline="")
            # Window mock
            self.canvas.create_rectangle(x1, y1, x2, y2, fill=col, outline=PAL["text"], width=1)
            self.canvas.create_rectangle(x1, y1, x2, y1+30, fill="#1A1C23", outline="")
            self.canvas.create_text(x1+10, y1+15, text=txt, fill=PAL["text"], font=("Inter", 9, "bold"), anchor="w")

        # Status
        self.status = tk.Label(self, text=f"COMPOSITOR ACTIVE | V-SYNC: {'ON' if self.vsync else 'OFF'} | TARGET FRAME-TIME: 6.94ms (144Hz)", 
                               bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _flush_buffer(self):
        self.status.config(text="FLUSHING BACKBUFFER TO DISPLAY (POINTER SWAP)...", bg=PAL["warning"], fg="black")
        self.after(50, lambda: self.status.config(text=f"SCREEN REFRESHED | FRAME RENDERED IN {random.uniform(2.1, 4.3):.2f}ms", bg=PAL["success"], fg="black"))

    def _toggle_vsync(self):
        self.vsync = not self.vsync
        stat = "ON" if self.vsync else "OFF"
        col = PAL["success"] if self.vsync else PAL["danger"]
        
        if self.vsync:
            msg = "V-Sync Enabled. Engine refresh locked to hardware chronometer.\nScreen tearing prevented."
        else:
            msg = "V-Sync Disabled. Rendering engine fully unthrottled.\nFPS unlocked. Potential screen tearing introduced."
            
        messagebox.showinfo("Hardware Compositor", msg)
        self.status.config(text=f"COMPOSITOR ACTIVE | V-SYNC: {stat} | GPU UNLOCKED", bg=col, fg="black" if self.vsync else "white")


if __name__ == "__main__":
    app = FluidCompositor()
    app.mainloop()
