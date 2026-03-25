from sigma_core.hal.native_libc import ApexLib
"""
SigmaOS — Shell Game (G23): Watch the Cup!
==========================================
Classic cup-and-ball (thimblerig) game.
A coin is hidden under one of 3 cups, cups shuffle, user picks which cup hides the coin.
Multiple difficulty levels, speed tiers, score tracking.
100% IP-safe — no copyrighted assets.
"""
import tkinter as tk
from tkinter import messagebox
import random
# import time (Purged by Scrubber)

PAL = {
    "bg":       "#0E0E12",
    "panel":    "#16181F",
    "card":     "#1E2028",
    "accent":   "#FFD60A",   # Gold — coin colour
    "accent2":  "#7B2FFF",
    "cup":      "#8B5E3C",   # Cup brown
    "cup_hi":   "#C4854F",
    "success":  "#32D74B",
    "danger":   "#FF453A",
    "text":     "#F2F2F7",
    "dim":      "#636366",
    "border":   "#2C2E36",
}

SPEEDS = {
    "Beginner":    {"swaps": 3,  "delay": 900, "bonus": 1},
    "Normal":      {"swaps": 5,  "delay": 600, "bonus": 2},
    "Hard":        {"swaps": 8,  "delay": 350, "bonus": 4},
    "Impossible":  {"swaps": 14, "delay": 160, "bonus": 8},
}


class WatchTheCup(tk.Tk):
    """
    Interactive shell/cup game with animated shuffling, score system,
    streak tracking, and four difficulty levels.
    """

    CUP_W, CUP_H = 110, 140    # cup rectangle dimensions
    CUP_RIM      = 18           # rim height
    COIN_R       = 22           # coin radius
    CUP_Y        = 200          # cup top Y on canvas
    CUP_XS       = [160, 380, 600]   # cup centre-X positions

    def __init__(self):
        super().__init__()
        self.title("SigmaOS — Watch the Cup!")
        self.geometry("900x680")
        self.configure(bg=PAL["bg"])
        self.resizable(False, False)

        # Game state
        self.difficulty  = "Normal"
        self.coin_pos    = 1          # index 0-2
        self.cup_xs      = list(self.CUP_XS)
        self.score       = 0
        self.streak      = 0
        self.best_streak = 0
        self.round_n     = 0
        self.phase       = "idle"    # idle | reveal | shuffling | guess | result
        self.coin_shown  = True
        self._anim_after = None

        self._build_ui()
        self._draw_scene(show_coin=True)

    # ── UI ────────────────────────────────────────────────────────────────

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg=PAL["bg"], height=60)
        hdr.pack(fill="x", padx=24, pady=(14, 0))
        hdr.pack_propagate(False)

        tk.Label(hdr, text="🎩 WATCH THE CUP!",
                 font=("Segoe UI", 22, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")

        # Difficulty selector
        diff_fr = tk.Frame(hdr, bg=PAL["bg"])
        diff_fr.pack(side="right")
        tk.Label(diff_fr, text="Difficulty:",
                 fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI", 9)).pack(side="left", padx=(0, 6))
        self.diff_var = tk.StringVar(value=self.difficulty)
        for d in SPEEDS:
            col = PAL["accent"] if d == self.difficulty else PAL["dim"]
            tk.Radiobutton(diff_fr, text=d, variable=self.diff_var, value=d,
                           bg=PAL["bg"], fg=col, selectcolor=PAL["panel"],
                           activebackground=PAL["bg"],
                           command=self._on_diff_change,
                           font=("Segoe UI", 8)).pack(side="left", padx=3)

        # Canvas
        self.canvas = tk.Canvas(self, width=760, height=400,
                                bg=PAL["panel"], highlightthickness=0)
        self.canvas.pack(pady=(10, 4))

        # Instruction label
        self.lbl_instr = tk.Label(self, text="Press  ▶ PLAY  to start a round.",
                                  font=("Segoe UI", 13),
                                  fg=PAL["text"], bg=PAL["bg"])
        self.lbl_instr.pack(pady=4)

        # Stats bar
        stats_fr = tk.Frame(self, bg=PAL["card"])
        stats_fr.pack(fill="x", padx=24, pady=(4, 0))

        self.lbl_score  = self._stat(stats_fr, "Score",       "0")
        self.lbl_streak = self._stat(stats_fr, "Streak 🔥",   "0")
        self.lbl_best   = self._stat(stats_fr, "Best Streak", "0")
        self.lbl_round  = self._stat(stats_fr, "Round",       "0")

        # Buttons
        btn_fr = tk.Frame(self, bg=PAL["bg"])
        btn_fr.pack(pady=10)

        self.btn_play = tk.Button(
            btn_fr, text="▶  PLAY",
            command=self._start_round,
            font=("Segoe UI", 12, "bold"),
            bg=PAL["accent2"], fg="white",
            relief="flat", padx=28, pady=10, cursor="hand2")
        self.btn_play.pack(side="left", padx=8)

        self.btn_reset = tk.Button(
            btn_fr, text="↺  Reset Score",
            command=self._reset,
            font=("Segoe UI", 10),
            bg=PAL["panel"], fg=PAL["dim"],
            relief="flat", padx=18, pady=10, cursor="hand2")
        self.btn_reset.pack(side="left", padx=8)

        # Status strip
        self.status = tk.Label(self, text="Ready. Pick a difficulty and press PLAY.",
                               bg=PAL["accent2"], fg="white",
                               font=("Segoe UI", 9, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

    def _stat(self, parent, label, value):
        fr = tk.Frame(parent, bg=PAL["card"], padx=20, pady=10)
        fr.pack(side="left", expand=True, fill="x")
        tk.Label(fr, text=label, font=("Segoe UI", 8),
                 fg=PAL["dim"], bg=PAL["card"]).pack()
        lbl = tk.Label(fr, text=value, font=("Segoe UI", 16, "bold"),
                       fg=PAL["accent"], bg=PAL["card"])
        lbl.pack()
        return lbl

    # ── Drawing ───────────────────────────────────────────────────────────

    def _draw_scene(self, show_coin=False, highlight=None, result_cup=None):
        c = self.canvas
        c.delete("all")
        W, H = 760, 400

        # Table surface
        c.create_rectangle(0, 340, W, 400, fill="#1A1209", outline="")
        c.create_rectangle(0, 340, W, 346, fill="#2E1F0A", outline="")

        for i, cx in enumerate(self.cup_xs):
            cy = self.CUP_Y

            # Coin (behind cup, only if shown or cup lifted)
            if show_coin and i == self.coin_pos:
                self._draw_coin(c, cx, cy + self.CUP_H - 8)

            # Cup shadow
            c.create_oval(cx - 52, 338, cx + 52, 352,
                          fill="#0A0A0A", outline="")

            # Cup body
            col = PAL["cup_hi"] if i == highlight else PAL["cup"]
            if result_cup is not None and i == result_cup:
                col = PAL["success"] if i == self.coin_pos else PAL["danger"]

            # Trapezoid-like cup via polygon
            pts = [
                cx - 42, cy + self.CUP_H,   # bottom-left
                cx + 42, cy + self.CUP_H,   # bottom-right
                cx + 30, cy,                 # top-right
                cx - 30, cy,                 # top-left
            ]
            c.create_polygon(pts, fill=col, outline="#5A3A1A", width=2)

            # Rim
            c.create_rectangle(cx - 36, cy - self.CUP_RIM,
                                cx + 36, cy,
                                fill="#5A3A1A", outline="#3A2010", width=1)
            # Knob
            c.create_oval(cx - 10, cy - self.CUP_RIM - 14,
                          cx + 10, cy - self.CUP_RIM + 2,
                          fill="#3A2010", outline="#1A0E05")

            # Number label
            c.create_text(cx, cy + self.CUP_H // 2 + 10,
                          text=str(i + 1), fill="#FFD69E",
                          font=("Segoe UI", 16, "bold"))

        # If result: show lifted cup
        if result_cup is not None:
            self._draw_coin(c, self.cup_xs[self.coin_pos],
                            self.CUP_Y + self.CUP_H - 8)

    def _draw_coin(self, c, x, y):
        r = self.COIN_R
        c.create_oval(x - r, y - r, x + r, y + r,
                      fill="#D4AF37", outline="#FFD700", width=3)
        c.create_oval(x - r + 6, y - r + 6, x + r - 6, y + r - 6,
                      fill="#E8C840", outline="")
        c.create_text(x, y, text="₿", fill="#8B6914",
                      font=("Segoe UI", 14, "bold"))

    # ── Game Flow ────────────────────────────────────────────────────────

    def _start_round(self):
        if self.phase in ("reveal", "shuffling"):
            return

        self.round_n    += 1
        self.coin_pos    = random.randint(0, 2)
        self.cup_xs      = list(self.CUP_XS)
        self.phase       = "reveal"
        self.coin_shown  = True

        self.btn_play.config(state="disabled")
        self.lbl_instr.config(
            text="👀  Watch carefully — remember the coin!",
            fg=PAL["accent"])

        self._draw_scene(show_coin=True)
        self.lbl_round.config(text=str(self.round_n))

        cfg = SPEEDS[self.difficulty]
        # After reveal delay → begin shuffling
        self.after(1200, lambda: self._begin_shuffle(cfg))

    def _begin_shuffle(self, cfg):
        self.phase = "shuffling"
        self._draw_scene(show_coin=False)
        self.lbl_instr.config(text="🔀  Shuffling…", fg=PAL["dim"])
        self._do_swaps(cfg["swaps"], cfg["delay"], cfg)

    def _do_swaps(self, remaining, delay, cfg):
        if remaining <= 0:
            self.phase = "guess"
            self.lbl_instr.config(
                text="🖱  Click a cup — where's the coin?",
                fg=PAL["accent"])
            self._draw_scene(show_coin=False)
            self.canvas.bind("<Button-1>", self._on_canvas_click)
            return

        # Pick two different cups to swap
        a, b = random.sample([0, 1, 2], 2)
        self._animate_swap(a, b, delay, lambda: self._do_swaps(remaining - 1, delay, cfg))

    def _animate_swap(self, a, b, delay, callback, step=0, steps=8):
        """Smoothly interpolate cup positions."""
        if self._anim_after:
            self.after_cancel(self._anim_after)

        ax0, bx0 = list(self.CUP_XS)[a], list(self.CUP_XS)[b]
        # Take live positions
        xa_start = self.cup_xs[a]
        xb_start = self.cup_xs[b]

        def tick(s):
            frac = s / steps
            self.cup_xs[a] = int(xa_start + (xb_start - xa_start) * frac)
            self.cup_xs[b] = int(xb_start + (xa_start - xb_start) * frac)
            self._draw_scene(show_coin=False)
            if s < steps:
                self._anim_after = self.after(delay // steps, lambda: tick(s + 1))
            else:
                # Commit final positions
                self.cup_xs[a] = xb_start
                self.cup_xs[b] = xa_start
                # Also update which cup has the coin
                if self.coin_pos == a:
                    self.coin_pos = b
                elif self.coin_pos == b:
                    self.coin_pos = a
                self._draw_scene(show_coin=False)
                self._anim_after = self.after(max(10, delay // 4), callback)

        tick(0)

    def _on_canvas_click(self, event):
        if self.phase != "guess":
            return
        self.canvas.unbind("<Button-1>")
        chosen = self._cup_at(event.x, event.y)
        if chosen is None:
            self.canvas.bind("<Button-1>", self._on_canvas_click)
            return
        self._resolve(chosen)

    def _cup_at(self, x, y):
        """Return cup index under click, or None."""
        for i, cx in enumerate(self.cup_xs):
            cy = self.CUP_Y
            if (cx - 45 <= x <= cx + 45) and (cy <= y <= cy + self.CUP_H):
                return i
        return None

    def _resolve(self, chosen):
        self.phase = "result"
        cfg   = SPEEDS[self.difficulty]
        win   = (chosen == self.coin_pos)

        if win:
            self.streak += 1
            self.best_streak = max(self.best_streak, self.streak)
            pts = 10 * cfg["bonus"] * (1 + self.streak // 3)
            self.score += pts
            self.lbl_instr.config(
                text=f"✅  Correct! +{pts} points  🔥×{self.streak}",
                fg=PAL["success"])
            self.status.config(text=f"✅ Correct! Streak: {self.streak}",
                               bg=PAL["success"])
        else:
            self.streak = 0
            self.lbl_instr.config(
                text=f"❌  Wrong!  The coin was under cup {self.coin_pos + 1}.",
                fg=PAL["danger"])
            self.status.config(
                text=f"❌ Wrong! It was cup {self.coin_pos + 1}.",
                bg=PAL["danger"])

        # Draw result
        self._draw_scene(show_coin=True, result_cup=chosen)
        self._update_stats()

        self.after(2000, self._round_done)

    def _round_done(self):
        self.phase = "idle"
        self.btn_play.config(state="normal")
        self.lbl_instr.config(
            text="Press  ▶ PLAY  for the next round.",
            fg=PAL["text"])
        self.status.config(
            text="Ready for next round.",
            bg=PAL["accent2"])
        self._draw_scene(show_coin=False)

    # ── Utilities ─────────────────────────────────────────────────────────

    def _update_stats(self):
        self.lbl_score.config(text=str(self.score))
        self.lbl_streak.config(text=str(self.streak))
        self.lbl_best.config(text=str(self.best_streak))
        self.lbl_round.config(text=str(self.round_n))

    def _on_diff_change(self):
        self.difficulty = self.diff_var.get()
        self.status.config(
            text=f"Difficulty set to: {self.difficulty}",
            bg=PAL["accent2"])

    def _reset(self):
        if self.phase in ("reveal", "shuffling"):
            return
        self.score = self.streak = self.best_streak = self.round_n = 0
        self._update_stats()
        self.phase = "idle"
        self.cup_xs = list(self.CUP_XS)
        self._draw_scene(show_coin=True)
        self.lbl_instr.config(text="Score reset. Press ▶ PLAY.", fg=PAL["text"])
        self.status.config(text="Score reset.", bg=PAL["accent2"])


if __name__ == "__main__":
    app = WatchTheCup()
    app.mainloop()
