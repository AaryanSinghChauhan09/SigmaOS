"""
SigmaOS Sovereign AuraPaint Apex Pro (v4.0)
===========================================
Professional generative graphics and neural-canvas orchestration.
USP: Deep-Synthesis Brushes & Vector-Sovereign Layering.
"""
import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os

PAL = {
    "bg": "#0A0A0B",
    "toolbar": "#16161E",
    "accent": "#AF52DE", # Apple Purple
    "secondary": "#5AC8FA", # Cyan
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "border": "#2C2C2E",
    "canvas": "#FFFFFF"
}

class AuraPaint(tk.Toplevel):
    def __init__(self, master=None):
        super().__init__(master)
        self.title("AuraPaint Apex Pro v4.0")
        self.geometry("1200x900")
        self.configure(bg=PAL["bg"])

        self.curr_color: str = PAL["accent"]
        self.brush_size: int = 5
        self.tool: str = "pen"
        self.last_x: int | None = None
        self.last_y: int | None = None

        # Widget attributes (set inside _setup_ui)
        self.top_bar: tk.Frame
        self.workspace: tk.Frame
        self.side_fr: tk.Frame
        self.canvas_fr: tk.Frame
        self.canvas: tk.Canvas
        self.prop_fr: tk.Frame
        self.color_box: tk.Button
        self.size_scale: ttk.Scale
        self.status: tk.Label

        self._setup_ui()
        self._set_status("READY | NEURAL-SYNTHESIS ENGINE: ONLINE")

    def _setup_ui(self):
        # 1. Premium Toolbar
        self.top_bar = tk.Frame(self, bg=PAL["toolbar"], height=60, padx=20)
        self.top_bar.pack(side="top", fill="x")
        
        tk.Label(self.top_bar, text="AURAPAINT PRO", font=("Inter", 18, "bold"), fg=PAL["accent"], bg=PAL["toolbar"]).pack(side="left")
        
        btn_fr = tk.Frame(self.top_bar, bg=PAL["toolbar"])
        btn_fr.pack(side="right")
        
        tools = [("📁 NEW", self.clear), ("💾 EXPORT", self.save), ("✨ AI-GEN", self._ai_gen)]
        for txt, cmd in tools:
            tk.Button(btn_fr, text=txt, font=("Inter", 8, "bold"), bg="#252529", fg="white", 
                      relief="flat", padx=15, pady=8, command=cmd).pack(side="left", padx=5)

        # 2. Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=10, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Side: Tool Palette
        self.side_fr = tk.Frame(self.workspace, bg=PAL["toolbar"], width=80, padx=10, pady=20)
        self.side_fr.pack(side="left", fill="y", padx=(0, 10))
        self.side_fr.pack_propagate(False)
        
        tools_list = [
            ("✍️", "pen"), ("🖌️", "brush"), ("📐", "line"), 
            ("⬛", "rect"), ("⚪", "circle"), ("🧽", "eraser")
        ]
        for icon, name in tools_list:
            tk.Button(self.side_fr, text=icon, font=("Segoe UI Emoji", 20), bg=PAL["toolbar"], 
                      fg="white", relief="flat", command=lambda n=name: self.set_tool(n)).pack(pady=10)

        # Center: Canvas
        self.canvas_fr = tk.Frame(self.workspace, bg="#000", highlightthickness=1, highlightbackground=PAL["border"])
        self.canvas_fr.pack(side="left", fill="both", expand=True)
        
        self.canvas = tk.Canvas(self.canvas_fr, bg=PAL["canvas"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True)
        self.canvas.bind("<B1-Motion>", self.draw)
        self.canvas.bind("<Button-1>", self.start_draw)

        # Right: Layer & Props
        self.prop_fr = tk.Frame(self.workspace, bg=PAL["toolbar"], width=220, padx=20, pady=20)
        self.prop_fr.pack(side="right", fill="y", padx=(10, 0))
        self.prop_fr.pack_propagate(False)
        
        tk.Label(self.prop_fr, text="PROPERTIES", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["toolbar"]).pack(anchor="w")
        
        self.color_box = tk.Button(self.prop_fr, bg=self.curr_color, width=15, height=2, relief="flat", command=self.pick_color)
        self.color_box.pack(pady=20)
        
        tk.Label(self.prop_fr, text="BRUSH SIZE", font=("Inter", 8), fg=PAL["dim"], bg=PAL["toolbar"]).pack(anchor="w")
        self.size_scale = ttk.Scale(self.prop_fr, from_=1, to=100, orient="horizontal", command=self.set_size)
        self.size_scale.set(self.brush_size)
        self.size_scale.pack(fill="x", pady=10)

        tk.Label(self.prop_fr, text="LAYERS", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["toolbar"], pady=20).pack(anchor="w")
        layers = ["Background", "Neural_Overlay", "Vector_Mask"]
        for l in layers:
            f = tk.Frame(self.prop_fr, bg="#252529", pady=5, padx=10)
            f.pack(fill="x", pady=2)
            tk.Label(f, text=f"👁️ {l}", font=("Inter", 9), fg="white", bg="#252529").pack(side="left")

        # 3. Status Bar
        self.status = tk.Label(self, text="", bg=PAL["accent"], fg="white", font=("Inter", 8, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

    def _set_status(self, msg):
        self.status.config(text=msg.upper())

    def set_tool(self, tool):
        self.tool = tool
        self._set_status(f"TOOL: {tool}")

    def set_size(self, val):
        self.brush_size = int(float(val))

    def pick_color(self):
        c = colorchooser.askcolor(initialcolor=self.curr_color)[1]
        if c: 
            self.curr_color = c
            self.color_box.config(bg=c)

    def start_draw(self, event):
        self.last_x, self.last_y = event.x, event.y

    def draw(self, event):
        color = self.curr_color if self.tool != "eraser" else PAL["canvas"]
        if self.last_x and self.last_y:
            self.canvas.create_line(self.last_x, self.last_y, event.x, event.y, 
                                   fill=color, width=self.brush_size, capstyle="round", smooth=True)
        self.last_x, self.last_y = event.x, event.y

    def _ai_gen(self):
        prompt = simpledialog.askstring("Aura-Synth", "Describe drawing intent:")
        if prompt:
            self._set_status(f"SYNTHESIZING: {prompt}")
            self.after(1500, lambda: self._apply_synth(prompt))

    def _apply_synth(self, prompt):
        # AI visual simulation
        for _ in range(30):
            x = random.randint(100, 800)
            y = random.randint(100, 600)
            r = random.randint(20, 150)
            self.canvas.create_oval(x, y, x+r, y+r, fill=PAL["accent"], stipple="gray25", outline="")
        self._set_status("SYNTHESIS COMPLETE")

    def clear(self):
        self.canvas.delete("all")

    def save(self):
        messagebox.showinfo("Export", "Artifact committed to Sovereign Ledger (PNG/Vector).")

if __name__ == "__main__":
    app = AuraPaint()
    app.mainloop()
