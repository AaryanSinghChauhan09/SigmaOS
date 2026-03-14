"""
SigmaOS Sovereign Zenith Arcade (v4.0 Apex)
===========================================
A premium collection of board games and strategic simulations.
USP: Neural-latency game logic & Zero-Telemetery Multiplayer Arcade.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional

# Decouple via absolute path injection
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..")))

try:
    from sigma_core.ui.fluid_design import PALETTE as FLUID_PAL, ICONS # type: ignore
    from userland.apps.chess import SovereignStrategist # type: ignore
    from userland.apps.ludo import LudoApp # type: ignore
except ImportError:
    FLUID_PAL = None
    SovereignStrategist = None
    LudoApp = None

PAL = {
    "bg": FLUID_PAL["background"] if FLUID_PAL else "#0B0C0F",
    "sidebar": FLUID_PAL["surface"] if FLUID_PAL else "#16181D",
    "accent": FLUID_PAL["primary"] if FLUID_PAL else "#5E5CE6",
    "text": FLUID_PAL["text_primary"] if FLUID_PAL else "#E8E8E8",
    "dim": FLUID_PAL["text_secondary"] if FLUID_PAL else "#8E8E93",
    "p1": "#FF3B30", "p2": "#007AFF"
}

class SovereignArcade(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Zenith Arcade v4.0 Apex")
        self.geometry("1100x850")
        self.configure(bg=PAL["bg"])
        
        # UI Attributes (Pre-hydrated for linter health)
        self.tabs: Any = None
        self.status_lbl: Any = None
        
        self.xo_board = [""] * 9
        self.xo_turn = "X"
        self.xo_btns: List[tk.Button] = []
        self.xo_status: Any = None
        
        self.nexus_p1 = 0
        self.nexus_p2 = 0
        self.nexus_turn = 1
        self.nx_lbl: Any = None
        self.nx_score: Any = None
        self.nx_canv: Any = None
        self.nx_lines: Dict[Any, Any] = {}
        self.nx_boxes: set = set()
        
        self.bl_active = False
        self.bl_canv: Any = None
        self.bricks: List[Any] = []
        self.paddle: Any = None
        self.ball: Any = None
        self.bl_vx, self.bl_vy = 3, -3
        
        self._setup_styles()
        self._build_ui()
        
        # USP: Dynamic Aesthetic Sync (Interactive)
        if self.kernel and hasattr(self.kernel, "bus"):
             self.kernel.bus.subscribe("governor.vibe_switch", self._on_vibe_switch)

    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure("TNotebook", background=PAL["bg"], borderwidth=0)
        s.configure("TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["text"], padding=[20, 10])
        s.map("TNotebook.Tab", background=[("selected", PAL["accent"])])

    def _build_ui(self):
        head = tk.Frame(self, bg=PAL["bg"], padx=30, pady=20)
        head.pack(fill="x")
        tk.Label(head, text="ZENITH ARCADE", font=("Inter", 24, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        self.status_lbl = tk.Label(head, text="SYSTEM STATUS: PROTECTED", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["bg"])
        self.status_lbl.pack(side="right", pady=12)

        self.tabs = ttk.Notebook(self)
        self.tabs.pack(fill="both", expand=True, padx=25, pady=10)

        # Tab: Tic-Tac-Toe
        xo_fr = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
        self.tabs.add(xo_fr, text=f" {ICONS.get('board_hub', '⭕')} XO ")
        self._init_xo(xo_fr)

        # Tab: Nexus
        nx_fr = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
        self.tabs.add(nx_fr, text=f" {ICONS.get('mesh', '🕸️')} DOTS ")
        self._init_nexus(nx_fr)

        # Tab: Blocks
        bl_fr = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
        self.tabs.add(bl_fr, text=f" {ICONS.get('fabric', '🧱')} VOID ")
        self._init_blocks(bl_fr)
        
        # Tab: Proximity Launchers
        if SovereignStrategist:
            ch_fr = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
            self.tabs.add(ch_fr, text=f" {ICONS.get('ncert', '♟️')} CHESS ")
            tk.Button(ch_fr, text="LAUNCH SOVEREIGN CHESS", bg=PAL["accent"], fg="white", 
                      command=lambda: SovereignStrategist().mainloop() if SovereignStrategist else None, font=("Inter Bold", 10), padx=20, pady=10).pack(expand=True) # type: ignore
            
        if LudoApp:
            ld_fr = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
            self.tabs.add(ld_fr, text=f" {ICONS.get('board_hub', '🎲')} LUDO ")
            tk.Button(ld_fr, text="LAUNCH DETERMINISTIC LUDO", bg=PAL["accent"], fg="white", 
                      command=lambda: LudoApp().mainloop() if LudoApp else None, font=("Inter Bold", 10), padx=20, pady=10).pack(expand=True) # type: ignore

    def _init_xo(self, parent):
        self.xo_status = tk.Label(parent, text="X's STRATEGIC VECTOR", font=("Inter", 12, "bold"), fg=PAL["text"], bg=PAL["bg"])
        self.xo_status.pack(pady=20)
        grid = tk.Frame(parent, bg=PAL["bg"])
        grid.pack()
        for i in range(9):
            btn = tk.Button(grid, text="", font=("Inter", 32, "bold"), width=4, height=1, bg=PAL["sidebar"], fg="white", command=lambda idx=i: self._xo_move(idx)) # type: ignore
            btn.grid(row=i//3, column=i%3, padx=5, pady=5)
            self.xo_btns.append(btn)

    def _xo_move(self, idx):
        if self.xo_board[idx] == "":
            self.xo_board[idx] = self.xo_turn
            self.xo_btns[idx].config(text=self.xo_turn, fg=PAL["accent"] if self.xo_turn=="X" else "#32D74B")
            if self._check_xo():
                self.xo_status.config(text=f"WINNER: {self.xo_turn}", fg="#32D74B")
            else:
                self.xo_turn = "O" if self.xo_turn == "X" else "X"
                self.xo_status.config(text=f"{self.xo_turn}'s STRATEGIC VECTOR")

    def _check_xo(self):
        wins = [(0,1,2),(3,4,5),(6,7,8),(0,3,6),(1,4,7),(2,5,8),(0,4,8),(2,4,6)]
        return any(self.xo_board[w[0]] == self.xo_board[w[1]] == self.xo_board[w[2]] != "" for w in wins)

    def _init_nexus(self, parent):
        inf = tk.Frame(parent, bg=PAL["bg"]); inf.pack(fill="x", padx=50)
        self.nx_lbl = tk.Label(inf, text="RED'S TURN", font=("Inter", 12, "bold"), fg=PAL["p1"], bg=PAL["bg"])
        self.nx_lbl.pack(side="left")
        self.nx_score = tk.Label(inf, text="RED: 0 | BLUE: 0", font=("Inter", 10), fg=PAL["dim"], bg=PAL["bg"])
        self.nx_score.pack(side="right")
        self.nx_canv = tk.Canvas(parent, width=450, height=450, bg="#000")
        self.nx_canv.pack(pady=20)
        self.nx_canv.bind("<Button-1>", self._nexus_click)

    def _nexus_click(self, e):
        x, y = e.x, e.y
        best_d = 25
        best_l = None
        
        for r in range(5):
            for c in range(5):
                if c < 4: # Horizontal
                    lx, ly = 70+c*100+50, 70+r*100
                    d = ((x-lx)**2 + (y-ly)**2)**0.5
                    if d < best_d: best_d, best_l = d, ((c, r), (c+1, r))
                if r < 4: # Vertical
                    lx, ly = 70+c*100, 70+r*100+50
                    d = ((x-lx)**2 + (y-ly)**2)**0.5
                    if d < best_d: best_d, best_l = d, ((c, r), (c, r+1))
                    
        if best_l and best_l not in self.nx_lines:
            p1, p2 = best_l
            color = PAL["p1"] if self.nexus_turn == 1 else PAL["p2"]
            self.nx_lines[best_l] = self.nx_canv.create_line(70+p1[0]*100, 70+p1[1]*100, 70+p2[0]*100, 70+p2[1]*100, fill=color, width=5)
            
            # Box check
            found = False
            for r in range(4):
                for c in range(4):
                    if (c,r) not in self.nx_boxes:
                        edges = [((c,r),(c+1,r)), ((c,r+1),(c+1,r+1)), ((c,r),(c,r+1)), ((c+1,r),(c+1,r+1))]
                        if all(e in self.nx_lines for e in edges):
                            self.nx_canv.create_rectangle(70+c*100+10, 70+r*100+10, 70+(c+1)*100-10, 70+(r+1)*100-10, fill=color, stipple="gray25", outline="")
                            self.nx_boxes.add((c,r))
                            if self.nexus_turn == 1: self.nexus_p1 += 1
                            else: self.nexus_p2 += 1
                            found = True
            
            if not found:
                self.nexus_turn = 3 - self.nexus_turn
                self.nx_lbl.config(text="RED'S TURN" if self.nexus_turn == 1 else "BLUE'S TURN", fg=PAL["p1"] if self.nexus_turn == 1 else PAL["p2"])
            self.nx_score.config(text=f"RED: {self.nexus_p1} | BLUE: {self.nexus_p2}")

    def _init_blocks(self, parent):
        self.bl_canv = tk.Canvas(parent, width=400, height=500, bg="#000")
        self.bl_canv.pack()
        self.paddle = self.bl_canv.create_rectangle(160, 480, 240, 490, fill=PAL["accent"], outline="")
        self.ball = self.bl_canv.create_oval(195, 470, 205, 480, fill="white", outline="")
        
        # USP: Interactive Fluid Control
        self.bl_canv.bind("<Motion>", self._on_bl_mouse)
        tk.Button(parent, text="LAUNCH NEURAL SPHERE", command=self._bl_start).pack(pady=20)

    def _on_bl_mouse(self, event):
        if self.bl_active:
             x = max(40, min(360, event.x))
             self.bl_canv.coords(self.paddle, x-40, 480, x+40, 490)

    def _bl_start(self):
        if not self.bl_active: 
            self.bl_active = True
            self.bricks = []
            for r in range(5):
                for c in range(8):
                    color = PAL["accent"] if r % 2 == 0 else "#32D74B"
                    b = self.bl_canv.create_rectangle(5+c*50, 40+r*20, 45+c*50, 55+r*20, fill=color, outline="")
                    self.bricks.append(b)
            self._bl_loop()

    def _bl_loop(self):
        if not self.bl_active: return
        self.bl_canv.move(self.ball, self.bl_vx, self.bl_vy)
        bx1, by1, bx2, by2 = self.bl_canv.coords(self.ball)
        
        # Walls
        if bx1 <= 0 or bx2 >= 400: self.bl_vx *= -1
        if by1 <= 0: self.bl_vy *= -1
        if by2 >= 500:
            self.bl_active = False
            messagebox.showinfo("Arcade", "VOID REACHED.")
            self.bl_canv.coords(self.ball, 195, 470, 205, 480)
            return

        # Paddle Collision
        px1, py1, px2, py2 = self.bl_canv.coords(self.paddle)
        if by2 >= py1 and px1 <= (bx1+bx2)/2 <= px2:
            self.bl_vy *= -1
            
        # Brick Collision
        for b in list(self.bricks):
            bbox = self.bl_canv.coords(b)
            if bbox[0] <= (bx1+bx2)/2 <= bbox[2] and bbox[1] <= (by1+by2)/2 <= bbox[3]:
                self.bl_canv.delete(b)
                self.bricks.remove(b)
                self.bl_vy *= -1
                break
        
        if not self.bricks:
            self.bl_active = False
            messagebox.showinfo("Arcade", "MATRIX CLEARED.")

        self.after(16, self._bl_loop)

    def _on_vibe_switch(self, payload: Dict[str, Any]):
        vibe = payload.get("vibe", "STANDARD")
        # Shift Sidebar and Background colors based on vibe
        vibe_colors = {
            "APEX": "#FFD700", "GAMING": "#FF00FF", "ZEN": "#E0E0E0", "STANDARD": PAL["accent"]
        }
        color = vibe_colors.get(vibe, PAL["accent"])
        if self.status_lbl:
            self.status_lbl.config(fg=color)
        print(f"[ARCADE] Aesthetic alignment with {vibe} complete.")

if __name__ == "__main__":
    SovereignArcade().mainloop()
