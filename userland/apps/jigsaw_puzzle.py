"""
SigmaOS — Jigsaw Puzzle (G21)
==============================
Upload any image and it's cut into N×N tiles that get shuffled.
Drag-and-drop tiles back to their correct positions to complete the puzzle.
100% IP-safe — no 3rd-party assets.
"""
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

# Try to import PIL; if unavailable, use a built-in canvas renderer
try:
    from PIL import Image, ImageTk
    PIL_AVAILABLE = True
except ImportError:
    PIL_AVAILABLE = False

PAL = {
    "bg":        "#0B0C10",
    "panel":     "#13151A",
    "accent":    "#00D4FF",
    "accent2":   "#7B2FFF",
    "success":   "#32D74B",
    "warning":   "#FFD60A",
    "danger":    "#FF453A",
    "text":      "#F2F2F7",
    "dim":       "#636366",
    "card":      "#1C1E26",
    "border":    "#2C2E36",
}

GRID_OPTIONS = [3, 4, 5, 6]


class JigsawPuzzle(tk.Tk):
    """
    SigmaOS Jigsaw Puzzle — Interactive tile-sliding puzzle.
    When PIL is available, uses real image tiles.
    Otherwise renders a colorful numbered grid as a fallback.
    """

    def __init__(self):
        super().__init__()
        self.title("SigmaOS — Jigsaw Puzzle")
        self.geometry("1100x760")
        self.configure(bg=PAL["bg"])
        self.resizable(True, True)

        # State
        self.grid_n      = 4          # NxN
        self.tile_size   = 120
        self.tiles       = []         # list of tile dicts
        self.img_path    = None
        self.pil_img     = None
        self.start_time  = None
        self.moves       = 0
        self.solved      = False
        self.drag_data   = {"tile": None, "ox": 0, "oy": 0}
        self.tile_images = []         # keep refs alive

        self._build_ui()

    # ── UI Construction ──────────────────────────────────────────────────────

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg=PAL["bg"], pady=12)
        hdr.pack(fill="x", padx=20)

        tk.Label(hdr, text="🧩 JIGSAW PUZZLE",
                 font=("Segoe UI", 22, "bold"),
                 fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")

        ctrl = tk.Frame(hdr, bg=PAL["bg"])
        ctrl.pack(side="right")

        tk.Label(ctrl, text="Grid:", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI", 10)).pack(side="left", padx=(0, 4))

        self.grid_var = tk.IntVar(value=self.grid_n)
        for n in GRID_OPTIONS:
            tk.Radiobutton(ctrl, text=f"{n}×{n}", variable=self.grid_var,
                           value=n, bg=PAL["bg"], fg=PAL["text"],
                           selectcolor=PAL["panel"],
                           activebackground=PAL["bg"], activeforeground=PAL["accent"],
                           command=self._on_grid_change,
                           font=("Segoe UI", 9)).pack(side="left", padx=4)

        tk.Button(ctrl, text="📂 Load Image", command=self._load_image,
                  bg=PAL["accent2"], fg="white", relief="flat",
                  padx=14, pady=6, font=("Segoe UI", 9, "bold"),
                  cursor="hand2").pack(side="left", padx=(10, 0))

        tk.Button(ctrl, text="🔀 Shuffle", command=self._shuffle,
                  bg=PAL["panel"], fg=PAL["text"], relief="flat",
                  padx=14, pady=6, font=("Segoe UI", 9, "bold"),
                  cursor="hand2").pack(side="left", padx=6)

        tk.Button(ctrl, text="👁 Preview", command=self._preview,
                  bg=PAL["panel"], fg=PAL["text"], relief="flat",
                  padx=14, pady=6, font=("Segoe UI", 9, "bold"),
                  cursor="hand2").pack(side="left", padx=0)

        # Separator
        tk.Frame(self, bg=PAL["border"], height=1).pack(fill="x", padx=20)

        # Main area
        main = tk.Frame(self, bg=PAL["bg"])
        main.pack(fill="both", expand=True, padx=20, pady=14)

        # Canvas (puzzle board)
        self.canvas_frame = tk.Frame(main, bg=PAL["panel"],
                                     relief="flat", bd=2,
                                     highlightbackground=PAL["border"],
                                     highlightthickness=1)
        self.canvas_frame.pack(side="left", fill="both", expand=True)

        self.canvas = tk.Canvas(self.canvas_frame, bg=PAL["panel"],
                                highlightthickness=0, cursor="fleur")
        self.canvas.pack(fill="both", expand=True)

        # Right sidebar
        side = tk.Frame(main, bg=PAL["bg"], width=220)
        side.pack(side="right", fill="y", padx=(16, 0))
        side.pack_propagate(False)

        # Stats card
        stats = tk.Frame(side, bg=PAL["card"], padx=18, pady=18)
        stats.pack(fill="x", pady=(0, 14))

        tk.Label(stats, text="STATS", font=("Segoe UI", 9, "bold"),
                 fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")

        self.lbl_moves = self._stat_row(stats, "Moves", "0")
        self.lbl_time  = self._stat_row(stats, "Time", "00:00")
        self.lbl_grid  = self._stat_row(stats, "Grid", "4×4")
        self.lbl_tiles = self._stat_row(stats, "Tiles", "16")

        # Progress bar
        tk.Label(side, text="COMPLETION", font=("Segoe UI", 9, "bold"),
                 fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 6))

        self.progress_var = tk.DoubleVar(value=0)
        self.progress_bar = ttk.Progressbar(side, variable=self.progress_var,
                                            maximum=100, length=190)
        self.progress_bar.pack(fill="x")

        self.lbl_progress = tk.Label(side, text="0% Complete",
                                     font=("Segoe UI", 10, "bold"),
                                     fg=PAL["accent"], bg=PAL["bg"])
        self.lbl_progress.pack(pady=(6, 16))

        # How-to
        tips = tk.Frame(side, bg=PAL["card"], padx=14, pady=14)
        tips.pack(fill="x")
        tk.Label(tips, text="HOW TO PLAY", font=("Segoe UI", 9, "bold"),
                 fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w", pady=(0, 8))
        for tip in [
            "📂 Load any image",
            "🔀 Shuffle tiles",
            "🖱 Drag tiles to swap",
            "👁 Preview original",
            "🏆 Place all correctly!"
        ]:
            tk.Label(tips, text=tip, font=("Segoe UI", 8),
                     fg=PAL["text"], bg=PAL["card"],
                     wraplength=180, justify="left").pack(anchor="w", pady=1)

        # Bottom status bar
        self.status = tk.Label(self, text="Load an image to begin, or use the built-in demo.",
                               bg=PAL["accent"], fg="black",
                               font=("Segoe UI", 9, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

        # Canvas bindings
        self.canvas.bind("<ButtonPress-1>",   self._on_press)
        self.canvas.bind("<B1-Motion>",        self._on_drag)
        self.canvas.bind("<ButtonRelease-1>",  self._on_release)

        # Auto-start demo
        self._start_demo()
        self._tick_clock()

    def _stat_row(self, parent, label, value):
        row = tk.Frame(parent, bg=PAL["card"])
        row.pack(fill="x", pady=4)
        tk.Label(row, text=label, font=("Segoe UI", 8),
                 fg=PAL["dim"], bg=PAL["card"], width=8, anchor="w").pack(side="left")
        lbl = tk.Label(row, text=value, font=("Segoe UI", 11, "bold"),
                       fg=PAL["accent"], bg=PAL["card"])
        lbl.pack(side="left")
        return lbl

    # ── Demo Mode (no image needed) ─────────────────────────────────────────

    def _start_demo(self):
        self.grid_n = self.grid_var.get()
        self._build_demo_tiles()
        self._render_tiles()
        self._update_stats()

    def _build_demo_tiles(self):
        """Build colorful numbered tiles for demo mode."""
        n = self.grid_n
        total = n * n
        # Palette of vivid tile colors
        colors = [
            "#FF6B6B","#FF9F43","#FFD93D","#6BCB77","#4D96FF","#C77DFF",
            "#FF70AB","#00B4D8","#F4A261","#52B788","#FB8500","#8338EC",
            "#3A86FF","#FF006E","#FFBE0B","#8AC926","#1982C4","#6A4C93",
            "#FF595E","#FFCA3A","#6A994E","#023E8A","#E56B6F","#B5179E",
            "#480CA8","#4361EE","#4CC9F0","#F72585","#7400B8","#43AA8B",
            "#90BE6D","#F9C74F","#F9844A","#F8961E","#F3722C","#577590"
        ]

        self.tiles = []
        for i in range(total):
            row, col = divmod(i, n)
            self.tiles.append({
                "id":      i,
                "correct": i,
                "current": i,
                "color":   colors[i % len(colors)],
                "label":   str(i + 1),
                "canvas_id": None,
                "text_id":   None,
            })

        self.status.config(
            text=f"Demo mode — {n}×{n} grid. Load an image for a real jigsaw!",
            bg=PAL["accent2"])

    # ── Image loading ───────────────────────────────────────────────────────

    def _load_image(self):
        path = filedialog.askopenfilename(
            title="Select an image",
            filetypes=[("Images", "*.png *.jpg *.jpeg *.bmp *.gif *.webp"),
                       ("All files", "*.*")]
        )
        if not path:
            return
        if not PIL_AVAILABLE:
            messagebox.showinfo(
                "Pillow not installed",
                "Install Pillow (pip install Pillow) to use custom images.\n"
                "Running in demo mode with colored tiles instead."
            )
            return

        try:
            self.pil_img  = Image.open(path).convert("RGB")
            self.img_path = path
            self._build_image_tiles()
            self._render_tiles()
            self._shuffle()
            self.status.config(
                text=f"Image loaded: {os.path.basename(path)}  |  Shuffle and solve!",
                bg=PAL["success"])
        except Exception as e:
            messagebox.showerror("Error", f"Could not load image:\n{e}")

    def _build_image_tiles(self):
        if not self.pil_img:
            return
        n = self.grid_n
        ts = self.tile_size
        total = n * n
        img = self.pil_img.resize((ts * n, ts * n))
        self.tile_images = []
        self.tiles = []

        for i in range(total):
            r, c = divmod(i, n)
            box = (c * ts, r * ts, (c + 1) * ts, (r + 1) * ts)
            crop = img.crop(box)
            photo = ImageTk.PhotoImage(crop)
            self.tile_images.append(photo)
            self.tiles.append({
                "id":        i,
                "correct":   i,
                "current":   i,
                "photo":     photo,
                "canvas_id": None,
                "text_id":   None,
            })

    # ── Rendering ───────────────────────────────────────────────────────────

    def _render_tiles(self):
        self.canvas.delete("all")
        n   = self.grid_n
        ts  = self.tile_size
        pad = 4
        board_px = n * (ts + pad) + pad
        ox = max(0, (self.canvas.winfo_width()  - board_px) // 2)
        oy = max(0, (self.canvas.winfo_height() - board_px) // 2)

        for tile in self.tiles:
            pos = tile["current"]
            row, col = divmod(pos, n)
            x1 = ox + pad + col * (ts + pad)
            y1 = oy + pad + row * (ts + pad)
            x2, y2 = x1 + ts, y1 + ts

            # Color indicator: green if correct, normal if wrong
            correct = tile["current"] == tile["correct"]
            border = PAL["success"] if correct else PAL["border"]

            if "photo" in tile and tile["photo"]:
                cid = self.canvas.create_image(
                    x1, y1, anchor="nw", image=tile["photo"],
                    tags=(f"tile_{tile['id']}", "tile")
                )
                # Border overlay
                self.canvas.create_rectangle(
                    x1, y1, x2, y2,
                    outline=border, width=3 if correct else 1,
                    tags=(f"tile_{tile['id']}", "tile")
                )
                tile["canvas_id"] = cid
            else:
                # Demo colored tile
                cid = self.canvas.create_rectangle(
                    x1, y1, x2, y2,
                    fill=tile["color"], outline=border,
                    width=3 if correct else 1,
                    tags=(f"tile_{tile['id']}", "tile")
                )
                tid = self.canvas.create_text(
                    x1 + ts // 2, y1 + ts // 2,
                    text=tile["label"],
                    font=("Segoe UI", max(10, ts // 6), "bold"),
                    fill="white",
                    tags=(f"tile_{tile['id']}", "tile")
                )
                tile["canvas_id"] = cid
                tile["text_id"]   = tid

            # Store coords for hit-testing
            tile["x1"], tile["y1"] = x1, y1

    def _redraw(self):
        self.canvas.after(10, self._render_tiles)

    # ── Drag-and-Drop ───────────────────────────────────────────────────────

    def _find_tile_at(self, x, y):
        n  = self.grid_n
        ts = self.tile_size
        pad = 4
        board_px = n * (ts + pad) + pad
        ox = max(0, (self.canvas.winfo_width()  - board_px) // 2)
        oy = max(0, (self.canvas.winfo_height() - board_px) // 2)

        for tile in self.tiles:
            pos  = tile["current"]
            row, col = divmod(pos, n)
            x1 = ox + pad + col * (ts + pad)
            y1 = oy + pad + row * (ts + pad)
            if x1 <= x <= x1 + ts and y1 <= y <= y1 + ts:
                return tile
        return None

    def _find_tile_pos_at(self, x, y):
        """Return the grid position (0-indexed) at canvas coords x, y."""
        n  = self.grid_n
        ts = self.tile_size
        pad = 4
        board_px = n * (ts + pad) + pad
        ox = max(0, (self.canvas.winfo_width()  - board_px) // 2)
        oy = max(0, (self.canvas.winfo_height() - board_px) // 2)

        col = (x - ox - pad) // (ts + pad)
        row = (y - oy - pad) // (ts + pad)
        if 0 <= row < n and 0 <= col < n:
            return row * n + col
        return None

    def _on_press(self, event):
        if self.solved:
            return
        tile = self._find_tile_at(event.x, event.y)
        if tile:
            self.drag_data["tile"] = tile
            self.drag_data["start_pos"] = tile["current"]

    def _on_drag(self, event):
        pass  # Visual drag handled on release (swap-on-drop)

    def _on_release(self, event):
        if self.solved or not self.drag_data["tile"]:
            return
        src_tile = self.drag_data["tile"]
        tgt_pos  = self._find_tile_pos_at(event.x, event.y)

        if tgt_pos is not None and tgt_pos != src_tile["current"]:
            # Find tile at target position and swap
            tgt_tile = next((t for t in self.tiles if t["current"] == tgt_pos), None)
            if tgt_tile:
                src_tile["current"], tgt_tile["current"] = \
                    tgt_tile["current"], src_tile["current"]
                self.moves += 1
                self._update_stats()
                self._render_tiles()
                self._check_solved()

        self.drag_data["tile"] = None

    # ── Game Logic ──────────────────────────────────────────────────────────

    def _shuffle(self):
        if self.solved:
            self.solved = False
        positions = list(range(self.grid_n * self.grid_n))
        random.shuffle(positions)
        for tile, pos in zip(self.tiles, positions):
            tile["current"] = pos
        self.moves      = 0
        self.start_time = time.time()
        self._update_stats()
        self._render_tiles()
        self.status.config(
            text="Puzzle shuffled! Drag tiles to solve.",
            bg=PAL["accent2"])

    def _check_solved(self):
        correct = sum(1 for t in self.tiles if t["current"] == t["correct"])
        total   = len(self.tiles)
        pct     = int(correct / total * 100)
        self.progress_var.set(pct)
        self.lbl_progress.config(text=f"{pct}% Complete")

        if correct == total:
            self.solved = True
            elapsed = int(time.time() - (self.start_time or time.time()))
            m, s = divmod(elapsed, 60)
            self.status.config(
                text=f"🏆 SOLVED in {m:02}:{s:02}  |  {self.moves} moves. Congratulations!",
                bg=PAL["success"])
            messagebox.showinfo(
                "🏆 Puzzle Solved!",
                f"Congratulations!\n\nYou solved the {self.grid_n}×{self.grid_n} puzzle "
                f"in {m:02}:{s:02} using {self.moves} moves."
            )

    def _preview(self):
        win = tk.Toplevel(self)
        win.title("Original Image Preview")
        win.configure(bg=PAL["bg"])
        win.geometry("550x580")

        tk.Label(win, text="ORIGINAL IMAGE",
                 font=("Segoe UI", 12, "bold"),
                 fg=PAL["dim"], bg=PAL["bg"]).pack(pady=(16, 4))

        canvas = tk.Canvas(win, bg=PAL["panel"], highlightthickness=0)
        canvas.pack(fill="both", expand=True, padx=20, pady=10)

        if PIL_AVAILABLE and self.pil_img:
            preview = self.pil_img.resize((500, 500))
            photo   = ImageTk.PhotoImage(preview)
            canvas.create_image(250, 250, image=photo, anchor="center")
            win._photo = photo  # keep ref
        else:
            # Render the demo grid in solved order
            n   = self.grid_n
            ts  = 100
            pad = 3
            colors = [t["color"] for t in sorted(self.tiles, key=lambda x: x["correct"])]
            for i, color in enumerate(colors):
                row, col = divmod(i, n)
                x1 = 10 + col * (ts + pad)
                y1 = 10 + row * (ts + pad)
                canvas.create_rectangle(x1, y1, x1 + ts, y1 + ts,
                                        fill=color, outline=PAL["border"])
                canvas.create_text(x1 + ts // 2, y1 + ts // 2,
                                   text=str(i + 1),
                                   font=("Segoe UI", 12, "bold"), fill="white")

        tk.Button(win, text="Close", command=win.destroy,
                  bg=PAL["panel"], fg=PAL["text"], relief="flat",
                  padx=20, pady=6).pack(pady=10)

    # ── Utilities ───────────────────────────────────────────────────────────

    def _on_grid_change(self):
        self.grid_n = self.grid_var.get()
        self.tile_images = []
        if PIL_AVAILABLE and self.pil_img:
            self._build_image_tiles()
        else:
            self._build_demo_tiles()
        self._render_tiles()
        self._update_stats()

    def _update_stats(self):
        n     = self.grid_n
        total = n * n
        correct = sum(1 for t in self.tiles if t["current"] == t["correct"])
        pct   = int(correct / total * 100) if total else 0
        self.lbl_moves.config(text=str(self.moves))
        self.lbl_grid.config(text=f"{n}×{n}")
        self.lbl_tiles.config(text=str(total))
        self.progress_var.set(pct)
        self.lbl_progress.config(text=f"{pct}% Complete")

    def _tick_clock(self):
        if self.start_time and not self.solved:
            elapsed = int(time.time() - self.start_time)
            m, s = divmod(elapsed, 60)
            self.lbl_time.config(text=f"{m:02}:{s:02}")
        self.after(1000, self._tick_clock)


if __name__ == "__main__":
    app = JigsawPuzzle()
    app.mainloop()
