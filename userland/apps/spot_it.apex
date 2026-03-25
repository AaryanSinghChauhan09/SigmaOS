from sigma_core.hal.native_libc import ApexLib
"""
SigmaOS — Spot It! Find It From The Group (G22)
================================================
A set of random items appears on screen. The player must find the
TARGET item (shown top-left) hidden in a crowd of distractors.
Timed challenge, progressive difficulty, score and combo system.
100% IP-safe — all drawn with canvas primitives.
"""
import tkinter as tk
from tkinter import messagebox
import random
# import time (Purged by Scrubber)
import math

PAL = {
    "bg":       "#0B0C10",
    "panel":    "#14161C",
    "card":     "#1A1C24",
    "accent":   "#00D4FF",
    "accent2":  "#7B2FFF",
    "success":  "#32D74B",
    "danger":   "#FF453A",
    "warning":  "#FFD60A",
    "text":     "#F2F2F7",
    "dim":      "#636366",
    "border":   "#2C2E36",
}

# Shapes we draw natively — each is a callable(canvas, cx, cy, size, color)
def draw_circle(c, cx, cy, s, col, outline="#FFFFFF"):
    c.create_oval(cx-s, cy-s, cx+s, cy+s, fill=col, outline=outline, width=2)

def draw_square(c, cx, cy, s, col, outline="#FFFFFF"):
    c.create_rectangle(cx-s, cy-s, cx+s, cy+s, fill=col, outline=outline, width=2)

def draw_triangle(c, cx, cy, s, col, outline="#FFFFFF"):
    pts = [cx, cy-s, cx+s, cy+s, cx-s, cy+s]
    c.create_polygon(pts, fill=col, outline=outline, width=2)

def draw_diamond(c, cx, cy, s, col, outline="#FFFFFF"):
    pts = [cx, cy-s, cx+s, cy, cx, cy+s, cx-s, cy]
    c.create_polygon(pts, fill=col, outline=outline, width=2)

def draw_star(c, cx, cy, s, col, outline="#FFFFFF"):
    pts = []
    for i in range(10):
        angle = math.pi * i / 5 - math.pi / 2
        r     = s if i % 2 == 0 else s * 0.45
        pts.extend([cx + r * math.cos(angle), cy + r * math.sin(angle)])
    c.create_polygon(pts, fill=col, outline=outline, width=2)

def draw_cross(c, cx, cy, s, col, outline="#FFFFFF"):
    t = s // 3
    c.create_rectangle(cx-t, cy-s, cx+t, cy+s, fill=col, outline=outline, width=2)
    c.create_rectangle(cx-s, cy-t, cx+s, cy+t, fill=col, outline=outline, width=2)

def draw_hexagon(c, cx, cy, s, col, outline="#FFFFFF"):
    pts = []
    for i in range(6):
        a = math.pi * i / 3
        pts.extend([cx + s * math.cos(a), cy + s * math.sin(a)])
    c.create_polygon(pts, fill=col, outline=outline, width=2)

def draw_arrow(c, cx, cy, s, col, outline="#FFFFFF"):
    pts = [cx, cy-s, cx+s, cy, cx+s//2, cy, cx+s//2, cy+s, cx-s//2, cy+s, cx-s//2, cy, cx-s, cy]
    c.create_polygon(pts, fill=col, outline=outline, width=2)

SHAPES = [
    ("circle",   draw_circle),
    ("square",   draw_square),
    ("triangle", draw_triangle),
    ("diamond",  draw_diamond),
    ("star",     draw_star),
    ("cross",    draw_cross),
    ("hexagon",  draw_hexagon),
    ("arrow",    draw_arrow),
]

COLORS = [
    "#FF6B6B", "#FF9F43", "#FFD93D", "#6BCB77",
    "#4D96FF", "#C77DFF", "#FF70AB", "#00B4D8",
    "#F4A261", "#52B788", "#FB8500", "#8338EC",
    "#3A86FF", "#FF006E", "#FFBE0B", "#8AC926",
]

LEVELS = [
    {"name": "Easy",       "count": 12, "time": 30, "bonus": 1},
    {"name": "Normal",     "count": 20, "time": 25, "bonus": 2},
    {"name": "Hard",       "count": 30, "time": 20, "bonus": 3},
    {"name": "Insane",     "count": 45, "time": 15, "bonus": 5},
    {"name": "Impossible", "count": 60, "time": 10, "bonus": 8},
]


class SpotItGame(tk.Tk):
    """
    Interactive 'Spot It!' / 'Find It!' game.
    A target shape+colour appears top-left; player clicks the matching
    item hidden among a crowd of distractors.
    """

    CANVAS_W = 860
    CANVAS_H = 460
    ITEM_SIZE = 22     # shape base radius

    def __init__(self):
        super().__init__()
        self.title("SigmaOS — Spot It! Find It From the Group")
        self.geometry("1000x720")
        self.configure(bg=PAL["bg"])
        self.resizable(True, True)

        # State
        self.level_idx   = 0
        self.score       = 0
        self.combo       = 0
        self.best_combo  = 0
        self.round_n     = 0
        self.time_left   = 0
        self.running     = False
        self.target      = None    # (shape_name, shape_fn, color)
        self.items       = []      # list of dicts placed on canvas
        self._tick_id    = None

        self._build_ui()

    # ── UI ────────────────────────────────────────────────────────────────

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg=PAL["bg"])
        hdr.pack(fill="x", padx=20, pady=(14, 4))

        tk.Label(hdr, text="🔍 SPOT IT — FIND THE TARGET!",
                 font=("Segoe UI", 20, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")

        ctrl = tk.Frame(hdr, bg=PAL["bg"])
        ctrl.pack(side="right")

        tk.Label(ctrl, text="Level:", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI", 9)).pack(side="left")
        self.level_var = tk.IntVar(value=0)
        for i, lv in enumerate(LEVELS):
            tk.Radiobutton(ctrl, text=lv["name"], variable=self.level_var, value=i,
                           bg=PAL["bg"], fg=PAL["text"], selectcolor=PAL["panel"],
                           activebackground=PAL["bg"],
                           command=lambda: None,
                           font=("Segoe UI", 8)).pack(side="left", padx=3)

        # Main layout
        main = tk.Frame(self, bg=PAL["bg"])
        main.pack(fill="both", expand=True, padx=20, pady=6)

        # Left: target preview + stats
        left = tk.Frame(main, bg=PAL["bg"], width=180)
        left.pack(side="left", fill="y")
        left.pack_propagate(False)

        # Target card
        tgt_card = tk.Frame(left, bg=PAL["card"], padx=16, pady=16)
        tgt_card.pack(fill="x", pady=(0, 12))
        tk.Label(tgt_card, text="FIND THIS", font=("Segoe UI", 9, "bold"),
                 fg=PAL["dim"], bg=PAL["card"]).pack()
        self.tgt_canvas = tk.Canvas(tgt_card, width=120, height=120,
                                    bg=PAL["panel"], highlightthickness=0)
        self.tgt_canvas.pack(pady=8)
        self.lbl_tgt_name = tk.Label(tgt_card, text="—",
                                     font=("Segoe UI", 10, "bold"),
                                     fg=PAL["accent"], bg=PAL["card"])
        self.lbl_tgt_name.pack()

        # Timer
        timer_card = tk.Frame(left, bg=PAL["card"], padx=16, pady=14)
        timer_card.pack(fill="x", pady=(0, 12))
        tk.Label(timer_card, text="TIME", font=("Segoe UI", 8, "bold"),
                 fg=PAL["dim"], bg=PAL["card"]).pack()
        self.lbl_timer = tk.Label(timer_card, text="—",
                                  font=("Segoe UI", 30, "bold"),
                                  fg=PAL["warning"], bg=PAL["card"])
        self.lbl_timer.pack()
        self.timer_bar_var = tk.DoubleVar(value=100)
        self.timer_bar = tk.Canvas(timer_card, width=140, height=8,
                                   bg=PAL["panel"], highlightthickness=0)
        self.timer_bar.pack()

        # Stats
        stats_card = tk.Frame(left, bg=PAL["card"], padx=16, pady=12)
        stats_card.pack(fill="x")
        self.lbl_score = self._mini_stat(stats_card, "Score",     "0")
        self.lbl_combo = self._mini_stat(stats_card, "Combo 🔥",  "×0")
        self.lbl_best  = self._mini_stat(stats_card, "Best Combo","×0")
        self.lbl_round = self._mini_stat(stats_card, "Round",     "0")

        # Game canvas
        self.canvas = tk.Canvas(main, width=self.CANVAS_W, height=self.CANVAS_H,
                                bg=PAL["panel"], highlightthickness=1,
                                highlightbackground=PAL["border"])
        self.canvas.pack(side="left", fill="both", expand=True, padx=(14, 0))
        self.canvas.bind("<Button-1>", self._on_click)

        # Buttons
        btn_fr = tk.Frame(self, bg=PAL["bg"])
        btn_fr.pack(pady=8)

        self.btn_play = tk.Button(
            btn_fr, text="▶  NEW ROUND",
            command=self._new_round,
            font=("Segoe UI", 12, "bold"),
            bg=PAL["accent2"], fg="white",
            relief="flat", padx=28, pady=10, cursor="hand2")
        self.btn_play.pack(side="left", padx=8)

        self.btn_reset = tk.Button(
            btn_fr, text="↺  Reset",
            command=self._reset_game,
            font=("Segoe UI", 10),
            bg=PAL["panel"], fg=PAL["dim"],
            relief="flat", padx=18, pady=10, cursor="hand2")
        self.btn_reset.pack(side="left", padx=8)

        self.status = tk.Label(self, text="Press  ▶ NEW ROUND  to start.",
                               bg=PAL["accent2"], fg="white",
                               font=("Segoe UI", 9, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

        # Initial prompt
        self.canvas.create_text(
            self.CANVAS_W // 2, self.CANVAS_H // 2,
            text="Press  ▶ NEW ROUND  to begin!",
            font=("Segoe UI", 18, "bold"),
            fill=PAL["dim"], tags="msg"
        )

    def _mini_stat(self, parent, label, value):
        row = tk.Frame(parent, bg=PAL["card"])
        row.pack(fill="x", pady=3)
        tk.Label(row, text=label, font=("Segoe UI", 8),
                 fg=PAL["dim"], bg=PAL["card"], width=10, anchor="w").pack(side="left")
        lbl = tk.Label(row, text=value, font=("Segoe UI", 11, "bold"),
                       fg=PAL["accent"], bg=PAL["card"])
        lbl.pack(side="left")
        return lbl

    # ── Game Logic ────────────────────────────────────────────────────────

    def _new_round(self):
        level = LEVELS[self.level_var.get()]
        self.round_n += 1
        self.time_left = level["time"]
        self.running   = True
        self.items     = []
        self.lbl_round.config(text=str(self.round_n))

        # Pick a target
        shape_name, shape_fn = random.choice(SHAPES)
        color   = random.choice(COLORS)
        self.target = (shape_name, shape_fn, color)

        # Draw target in preview
        self.tgt_canvas.delete("all")
        shape_fn(self.tgt_canvas, 60, 60, 30, color)
        self.lbl_tgt_name.config(text=f"{shape_name.capitalize()}")

        # Build item grid
        self._populate_canvas(level)

        # Start timer
        if self._tick_id:
            self.after_cancel(self._tick_id)
        self._tick()

        self.btn_play.config(state="disabled")
        self.status.config(text=f"Level: {level['name']} | Find the {shape_name}!", bg=PAL["panel"])

    def _populate_canvas(self, level):
        self.canvas.delete("all")
        self.items.clear()

        W, H = self.CANVAS_W, self.CANVAS_H
        n    = level["count"]
        s    = self.ITEM_SIZE

        # Generate positions
        positions = self._generate_positions(n, W, H, s)
        target_idx = random.randint(0, n - 1)

        shape_name, shape_fn, color = self.target

        for i, (x, y) in enumerate(positions):
            if i == target_idx:
                # Place the target
                chosen_shape = shape_fn
                chosen_name  = shape_name
                chosen_color = color
                is_target    = True
            else:
                # Place a distractor — different shape OR color
                while True:
                    other_name, other_fn = random.choice(SHAPES)
                    other_color = random.choice(COLORS)
                    if other_name != shape_name or other_color != color:
                        break
                chosen_shape = other_fn
                chosen_name  = other_name
                chosen_color = other_color
                is_target    = False

            # Draw item
            chosen_shape(self.canvas, x, y, s, chosen_color)

            # Invisible hit area for precise detection
            hit_id = self.canvas.create_oval(
                x - s - 4, y - s - 4, x + s + 4, y + s + 4,
                fill="", outline="", tags=f"item_{i}"
            )
            self.items.append({
                "idx":       i,
                "x":         x,
                "y":         y,
                "is_target": is_target,
                "hit_id":    hit_id,
            })

    def _generate_positions(self, n, W, H, s):
        """Generate non-overlapping random positions."""
        positions, attempts = [], 0
        min_dist = s * 2 + 6
        while len(positions) < n and attempts < n * 50:
            x = random.randint(s + 10, W - s - 10)
            y = random.randint(s + 10, H - s - 10)
            if all(math.hypot(x - px, y - py) >= min_dist for px, py in positions):
                positions.append((x, y))
            attempts += 1
        return positions

    def _on_click(self, event):
        if not self.running:
            return
        x, y = event.x, event.y

        for item in self.items:
            dx, dy = x - item["x"], y - item["y"]
            if math.hypot(dx, dy) <= self.ITEM_SIZE + 4:
                if item["is_target"]:
                    self._correct()
                else:
                    self._wrong(item["x"], item["y"])
                return

    def _correct(self):
        self.running = False
        if self._tick_id:
            self.after_cancel(self._tick_id)

        level  = LEVELS[self.level_var.get()]
        time_bonus = max(0, self.time_left)
        self.combo  += 1
        self.best_combo = max(self.best_combo, self.combo)
        pts = (10 + time_bonus * 2) * level["bonus"] * self.combo
        self.score += pts

        # Flash green success on canvas
        self.canvas.create_rectangle(0, 0, self.CANVAS_W, self.CANVAS_H,
                                     fill=PAL["success"], stipple="gray25",
                                     outline="", tags="flash")
        self.canvas.create_text(
            self.CANVAS_W // 2, self.CANVAS_H // 2,
            text=f"✅  FOUND IT!  +{pts} pts  🔥 ×{self.combo}",
            font=("Segoe UI", 22, "bold"),
            fill="white", tags="flash"
        )
        self._update_stats()
        self.status.config(text=f"✅ Correct! +{pts} pts | Combo: ×{self.combo}", bg=PAL["success"])
        self.after(1200, self._round_end)

    def _wrong(self, item_x, item_y):
        # Flash the item red
        self.canvas.create_oval(item_x - self.ITEM_SIZE - 6,
                                item_y - self.ITEM_SIZE - 6,
                                item_x + self.ITEM_SIZE + 6,
                                item_y + self.ITEM_SIZE + 6,
                                fill=PAL["danger"], outline="", tags="wrong_flash")
        self.after(300, lambda: self.canvas.delete("wrong_flash"))
        self.combo = 0
        self._update_stats()
        self.status.config(text="❌ Wrong! Keep looking…", bg=PAL["danger"])

    def _time_up(self):
        self.running = False
        self.combo = 0
        self._update_stats()
        self.canvas.create_rectangle(0, 0, self.CANVAS_W, self.CANVAS_H,
                                     fill=PAL["danger"], stipple="gray25",
                                     outline="", tags="flash")
        self.canvas.create_text(
            self.CANVAS_W // 2, self.CANVAS_H // 2,
            text="⏰  TIME'S UP!",
            font=("Segoe UI", 26, "bold"),
            fill="white", tags="flash"
        )
        self.status.config(text="⏰ Time's up! Press ▶ NEW ROUND.", bg=PAL["danger"])
        self.after(1500, self._round_end)

    def _round_end(self):
        self.canvas.delete("flash")
        self.running = False
        self.btn_play.config(state="normal")
        self.lbl_timer.config(text="—", fg=PAL["warning"])
        self.timer_bar.delete("all")

    def _tick(self):
        if not self.running:
            return
        level = LEVELS[self.level_var.get()]
        total = level["time"]

        pct  = self.time_left / total
        col  = PAL["success"] if pct > 0.5 else (PAL["warning"] if pct > 0.25 else PAL["danger"])
        self.lbl_timer.config(text=str(self.time_left), fg=col)

        # Draw timer bar
        self.timer_bar.delete("all")
        bar_w = int(140 * pct)
        self.timer_bar.create_rectangle(0, 0, 140, 8, fill=PAL["panel"], outline="")
        if bar_w > 0:
            self.timer_bar.create_rectangle(0, 0, bar_w, 8, fill=col, outline="")

        self.time_left -= 1
        if self.time_left < 0:
            self._time_up()
        else:
            self._tick_id = self.after(1000, self._tick)

    def _update_stats(self):
        self.lbl_score.config(text=str(self.score))
        self.lbl_combo.config(text=f"×{self.combo}")
        self.lbl_best.config(text=f"×{self.best_combo}")
        self.lbl_round.config(text=str(self.round_n))

    def _reset_game(self):
        self.running = False
        if self._tick_id:
            self.after_cancel(self._tick_id)
        self.score = self.combo = self.best_combo = self.round_n = 0
        self._update_stats()
        self.canvas.delete("all")
        self.tgt_canvas.delete("all")
        self.lbl_tgt_name.config(text="—")
        self.lbl_timer.config(text="—")
        self.timer_bar.delete("all")
        self.canvas.create_text(
            self.CANVAS_W // 2, self.CANVAS_H // 2,
            text="Press  ▶ NEW ROUND  to begin!",
            font=("Segoe UI", 18, "bold"),
            fill=PAL["dim"]
        )
        self.btn_play.config(state="normal")
        self.status.config(text="Reset. Pick a level and press ▶ NEW ROUND.", bg=PAL["accent2"])


if __name__ == "__main__":
    app = SpotItGame()
    app.mainloop()
