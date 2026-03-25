from sigma_core.hal.native_libc import ApexLib
"""
SigmaOS Sovereign Zenith Arcade (v4.0 Apex)
===========================================
A premium collection of board games and strategic simulations.
USP: Neural-latency game logic & Zero-Telemetery Multiplayer Arcade.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import random
# import time (Purged by Scrubber)

PAL = {
    "bg": "#0B0C0F",
    "sidebar": "#16181D",
    "accent": "#5E5CE6", # Deep Purple
    "text": "#E8E8E8",
    "dim": "#8E8E93",
    "success": "#32D74B",
    "danger": "#FF3B30",
    "p1": "#FF3B30",
    "p2": "#007AFF"
}

class SovereignArcade(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Sovereign Zenith Arcade v4.0")
        self.geometry("1000x850")
        self.configure(bg=PAL["bg"])
        
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Arcade.TNotebook", background=PAL["bg"], borderwidth=0)
        style.configure("Arcade.TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["text"], 
                        padding=[20, 10], font=("Inter", 9, "bold"))
        style.map("Arcade.TNotebook.Tab", background=[("selected", PAL["accent"])])

    def _build_ui(self):
        # Header
        head = tk.Frame(self, bg=PAL["bg"], padx=30, pady=20)
        head.pack(fill="x")
        tk.Label(head, text="ZENITH ARCADE", font=("Inter", 24, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        self.status_lbl = tk.Label(head, text="SYSTEM STATUS: PROTECTED | MESH IDLE", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["bg"])
        self.status_lbl.pack(side="right", pady=12)

        # Tabs
        self.tabs = ttk.Notebook(self, style="Arcade.TNotebook")
        self.tabs.pack(fill="both", expand=True, padx=25, pady=10)

        # 1. Quantum Tic-Tac-Toe
        xo_tab = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
        self.tabs.add(xo_tab, text=" ⭕ QUANTUM TIC-TAC-TOE ")
        self._init_xo(xo_tab)

        # 2. Nexus Connector
        nexus_tab = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
        self.tabs.add(nexus_tab, text=" 🕸️ NEXUS CONNECTOR ")
        self._init_nexus(nexus_tab)

        # 3. Aether Climber
        aether_tab = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
        self.tabs.add(aether_tab, text=" 🚀 AETHER CLIMBER ")
        self._init_aether(aether_tab)

        # 4. Sigma Blocks
        blocks_tab = tk.Frame(self.tabs, bg=PAL["bg"], pady=30)
        self.tabs.add(blocks_tab, text=" 🟥 SIGMA BLOCKS ")
        self._init_blocks(blocks_tab)

    # --- Quantum Tic-Tac-Toe ---
    def _init_xo(self, parent):
        self.xo_board = [""] * 9
        self.xo_turn = "X"
        self.xo_btns = []
        
        self.xo_status = tk.Label(parent, text="X's STRATEGIC VECTOR", font=("Inter", 12, "bold"), fg=PAL["text"], bg=PAL["bg"])
        self.xo_status.pack(pady=20)

        grid = tk.Frame(parent, bg=PAL["bg"])
        grid.pack()
        
        for i in range(9):
            btn = tk.Button(grid, text="", font=("Inter", 32, "bold"), width=4, height=1, 
                            bg=PAL["sidebar"], fg="white", relief="flat", highlightthickness=1,
                            highlightbackground=PAL["accent"], command=lambda idx=i: self._xo_move(idx))
            btn.grid(row=i//3, column=i%3, padx=5, pady=5)
            self.xo_btns.append(btn)
        
        tk.Button(parent, text="RESET MATRIX", font=("Inter", 9, "bold"), bg=PAL["danger"], 
                  fg="white", relief="flat", padx=30, pady=10, command=self._xo_reset).pack(pady=30)

    def _xo_move(self, idx):
        if self.xo_board[idx] == "":
            self.xo_board[idx] = self.xo_turn
            color = PAL["accent"] if self.xo_turn == "X" else PAL["success"]
            self.xo_btns[idx].config(text=self.xo_turn, fg=color)
            
            if self._check_xo():
                self.xo_status.config(text=f"WINNER: {self.xo_turn} | VECTOR SECURED", fg=PAL["success"])
                for b in self.xo_btns: b.config(state="disabled")
            else:
                self.xo_turn = "O" if self.xo_turn == "X" else "X"
                self.xo_status.config(text=f"{self.xo_turn}'s STRATEGIC VECTOR")

    def _check_xo(self):
        wins = [(0,1,2),(3,4,5),(6,7,8),(0,3,6),(1,4,7),(2,5,8),(0,4,8),(2,4,6)]
        for w in wins:
            if self.xo_board[w[0]] == self.xo_board[w[1]] == self.xo_board[w[2]] != "":
                return True
        return False

    def _xo_reset(self):
        self.xo_board = [""] * 9
        self.xo_turn = "X"
        for b in self.xo_btns: b.config(text="", state="normal", fg="white")
        self.xo_status.config(text="X's STRATEGIC VECTOR", fg=PAL["text"])

    # --- Nexus Connector (Dots & Boxes Premium) ---
    def _init_nexus(self, parent):
        self.nexus_p1 = 0
        self.nexus_p2 = 0
        self.nexus_turn = 1
        
        inf = tk.Frame(parent, bg=PAL["bg"])
        inf.pack(fill="x", padx=50)
        self.nx_lbl = tk.Label(inf, text="RED'S TURN", font=("Inter", 12, "bold"), fg=PAL["p1"], bg=PAL["bg"])
        self.nx_lbl.pack(side="left")
        self.nx_score = tk.Label(inf, text="RED: 0 | BLUE: 0", font=("Inter", 10), fg=PAL["dim"], bg=PAL["bg"])
        self.nx_score.pack(side="right")

        self.nx_canv = tk.Canvas(parent, width=450, height=450, bg="#000", highlightthickness=1, highlightbackground=PAL["accent"])
        self.nx_canv.pack(pady=20)
        
        self.nx_lines = {}
        self.nx_boxes = set()
        
        # Grid setup
        for r in range(5):
            for c in range(5):
                self.nx_canv.create_oval(70+c*100-4, 70+r*100-4, 70+c*100+4, 70+r*100+4, fill="white")
        
        self.nx_canv.bind("<Button-1>", self._nexus_click)

    def _nexus_click(self, e):
        # Simplified logic for finding nearest line
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

    # --- Sigma Blocks (Breakout Clone) ---
    def _init_blocks(self, parent):
        self.bl_active = False
        self.bl_canv = tk.Canvas(parent, width=400, height=500, bg="#000", highlightthickness=1, highlightbackground=PAL["accent"])
        self.bl_canv.pack()
        
        self.paddle = self.bl_canv.create_rectangle(160, 480, 240, 490, fill=PAL["accent"], outline="")
        self.ball = self.bl_canv.create_oval(195, 470, 205, 480, fill="white", outline="")
        self.bl_vx, self.bl_vy = 3, -3
        
        self.bricks = []
        for r in range(5):
            for c in range(8):
                color = PAL["accent"] if r % 2 == 0 else PAL["success"]
                b = self.bl_canv.create_rectangle(5+c*50, 40+r*20, 45+c*50, 55+r*20, fill=color, outline="")
                self.bricks.append(b)
                
        self.bl_canv.bind("<Motion>", self._bl_move_paddle)
        tk.Button(parent, text="LAUNCH NEURAL SPHERE", font=("Inter", 9, "bold"), bg=PAL["accent"], 
                  fg="white", relief="flat", padx=30, pady=10, command=self._bl_start).pack(pady=20)

    def _bl_move_paddle(self, e):
        x = e.x
        if 40 <= x <= 360:
            self.bl_canv.coords(self.paddle, x-40, 480, x+40, 490)

    def _bl_start(self):
        if not self.bl_active:
            self.bl_active = True
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
            messagebox.showinfo("Zenith Arcade", "VOID REACHED. REBOOTING SPHERE.")
            self.bl_canv.coords(self.ball, 195, 470, 205, 480)
            return

        # Paddle
        px1, py1, px2, py2 = self.bl_canv.coords(self.paddle)
        if by2 >= py1 and px1 <= (bx1+bx2)/2 <= px2:
            self.bl_vy *= -1
            
        # Bricks
        for b in self.bricks[:]:
            bbox = self.bl_canv.coords(b)
            if bbox[0] <= (bx1+bx2)/2 <= bbox[2] and bbox[1] <= (by1+by2)/2 <= bbox[3]:
                self.bl_canv.delete(b)
                self.bricks.remove(b)
                self.bl_vy *= -1
                break
        
        if not self.bricks:
            self.bl_active = False
            messagebox.showinfo("Zenith Arcade", "MATRIX CLEARED. YOU ARE THE ZENITH.")

        self.after(16, self._bl_loop)

if __name__ == "__main__":
    app = SovereignArcade()
    app.mainloop()
