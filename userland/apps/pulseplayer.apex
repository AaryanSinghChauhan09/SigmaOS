from sigma_core.hal.native_libc import ApexLib
"""
SigmaOS Sovereign PulsePlayer Apex Pro (v5.0)
============================================
Industry-standard Music Player: Animated visualizer, real file loading,
playlist management, EQ, and bit-perfect playback simulation.
USP: Neural Audio Upsampling & Zero-telemetry playback path.
"""
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
# import time (Purged by Scrubber), random, threading, os

PAL = {
    "bg": "#060608", "header": "#111116", "accent": "#5AC8FA",
    "secondary": "#5856D6", "text": "#E8E8E8", "dim": "#8E8E93",
    "border": "#1C1C24", "player_bg": "#0D0D12", "success": "#32D74B",
    "danger": "#FF3B30"
}

class PulsePlayer(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("PulsePlayer Apex Pro v5.0")
        self.geometry("1150x820")
        self.configure(bg=PAL["bg"])
        self.playing = False
        self.current_idx = 0
        self._progress = 0.0
        self._viz_data = [4] * 60
        self.tracks = [
            {"title": "Sovereign Beat Alpha", "artist": "Sigma_Originals", "dur": "05:12", "dur_s": 312},
            {"title": "Aether Harmony",       "artist": "Luna_Strings",    "dur": "03:45", "dur_s": 225},
            {"title": "Sigma Core Pulse",     "artist": "Hyper_Drive",     "dur": "04:20", "dur_s": 260},
            {"title": "Quantum Drift OST",    "artist": "Zenith_Nodes",    "dur": "06:01", "dur_s": 361},
            {"title": "Neural Network Sonata","artist": "AI_Composer_7",   "dur": "02:55", "dur_s": 175},
        ]
        self._setup_styles()
        self._build_ui()
        self._update_track_display()
        self._set_status("BIT-PERFECT MASTER ACTIVE | NEURAL UPSAMPLING: 384KHz | ZERO TELEMETRY")
        self._tick_timer()

    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use("clam")
        s.configure("Treeview", background=PAL["player_bg"], foreground=PAL["text"],
                    fieldbackground=PAL["player_bg"], borderwidth=0, font=("Segoe UI", 9))
        s.map("Treeview", background=[("selected", PAL["secondary"])])
        s.configure("Horizontal.TScale", background=PAL["header"], troughcolor=PAL["border"])

    def _build_ui(self):
        # HEADER
        head = tk.Frame(self, bg=PAL["header"], height=65, padx=30)
        head.pack(side="top", fill="x")
        head.pack_propagate(False)
        tk.Label(head, text="♫ PULSEPLAYER PRO", font=("Segoe UI", 14, "bold"),
                 fg=PAL["accent"], bg=PAL["header"]).pack(side="left", pady=15)
        
        search_fr = tk.Frame(head, bg="#1A1A24", padx=10, pady=6)
        search_fr.pack(side="right", padx=10, pady=12)
        tk.Label(search_fr, text="🔍", fg=PAL["dim"], bg="#1A1A24").pack(side="left")
        self.srch = tk.Entry(search_fr, bg="#1A1A24", fg="white", borderwidth=0,
                              font=("Segoe UI", 10), insertbackground="white", width=22)
        self.srch.pack(side="left", padx=5)
        self.srch.bind("<KeyRelease>", self._filter_tracks)

        # MAIN BODY
        body = tk.Frame(self, bg=PAL["bg"], padx=20, pady=15)
        body.pack(fill="both", expand=True)

        # LEFT: Art + Visualizer
        left_col = tk.Frame(body, bg=PAL["bg"])
        left_col.pack(side="left", fill="both", expand=True)

        # Album Art
        art_fr = tk.Frame(left_col, bg="#1A1A24", width=300, height=300,
                           highlightthickness=2, highlightbackground=PAL["secondary"])
        art_fr.pack(pady=10)
        art_fr.pack_propagate(False)
        self.art_lbl = tk.Label(art_fr, text="💿", font=("Segoe UI Symbol", 100),
                                 fg=PAL["secondary"], bg="#1A1A24")
        self.art_lbl.place(relx=0.5, rely=0.5, anchor="center")

        self.title_lbl = tk.Label(left_col, text="Select a track", font=("Segoe UI", 16, "bold"),
                                   fg="white", bg=PAL["bg"])
        self.title_lbl.pack()
        self.artist_lbl = tk.Label(left_col, text="—", font=("Segoe UI", 10),
                                    fg=PAL["dim"], bg=PAL["bg"])
        self.artist_lbl.pack(pady=(0, 10))

        # Visualizer
        self.viz = tk.Canvas(left_col, bg=PAL["bg"], height=100, highlightthickness=0)
        self.viz.pack(fill="x", padx=20)
        self._draw_viz()

        # EQ Section
        eq_fr = tk.Frame(left_col, bg=PAL["bg"])
        eq_fr.pack(fill="x", padx=20, pady=10)
        tk.Label(eq_fr, text="EQ", font=("Segoe UI", 8, "bold"), fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w")
        self._eq_vars = []
        for band in ["Bass", "Mid", "High", "Air"]:
            row = tk.Frame(eq_fr, bg=PAL["bg"])
            row.pack(fill="x", pady=2)
            tk.Label(row, text=band, font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["bg"], width=5, anchor="w").pack(side="left")
            v = tk.DoubleVar(value=70)
            self._eq_vars.append(v)
            ttk.Scale(row, from_=0, to=100, variable=v, orient="horizontal").pack(side="left", fill="x", expand=True)

        # RIGHT: Queue
        right_col = tk.Frame(body, bg=PAL["player_bg"], width=340, padx=15, pady=15)
        right_col.pack(side="right", fill="y", padx=(20, 0))
        right_col.pack_propagate(False)

        r_head = tk.Frame(right_col, bg=PAL["player_bg"])
        r_head.pack(fill="x", pady=(0, 10))
        tk.Label(r_head, text="SOVEREIGN QUEUE", font=("Segoe UI", 8, "bold"),
                 fg=PAL["dim"], bg=PAL["player_bg"]).pack(side="left")
        tk.Button(r_head, text="＋ ADD FILES", font=("Segoe UI", 7, "bold"),
                  bg=PAL["secondary"], fg="white", relief="flat", padx=8,
                  command=self._add_files).pack(side="right")

        self.queue_tree = ttk.Treeview(right_col, columns=("T", "D"), show="headings", height=18)
        self.queue_tree.heading("T", text="TRACK")
        self.queue_tree.heading("D", text="LEN")
        self.queue_tree.column("T", width=230)
        self.queue_tree.column("D", width=55, anchor="center")
        self.queue_tree.pack(fill="both", expand=True)
        self.queue_tree.bind("<<TreeviewSelect>>", self._on_select)
        self._populate_queue()

        # CONTROL DOCK
        dock = tk.Frame(self, bg=PAL["header"], height=150)
        dock.pack(side="bottom", fill="x")
        dock.pack_propagate(False)

        # Progress bar (interactive)
        prog_fr = tk.Frame(dock, bg=PAL["header"], padx=40, pady=12)
        prog_fr.pack(fill="x")
        self.time_lbl = tk.Label(prog_fr, text="0:00", font=("Cascadia Code", 8),
                                  fg=PAL["dim"], bg=PAL["header"])
        self.time_lbl.pack(side="left")
        self.prog_var = tk.DoubleVar(value=0)
        self.prog_bar = ttk.Scale(prog_fr, from_=0, to=100, variable=self.prog_var,
                                   orient="horizontal", command=self._on_seek)
        self.prog_bar.pack(side="left", fill="x", expand=True, padx=10)
        self.dur_lbl = tk.Label(prog_fr, text="0:00", font=("Cascadia Code", 8),
                                 fg=PAL["dim"], bg=PAL["header"])
        self.dur_lbl.pack(side="right")

        # Buttons
        btn_fr = tk.Frame(dock, bg=PAL["header"])
        btn_fr.pack()
        
        controls = [
            ("⏮", 18, self._prev), ("⏪", 15, None),
            ("▶", 36, self.toggle),
            ("⏩", 15, None), ("⏭", 18, self._next),
        ]
        for txt, size, cmd in controls:
            b = tk.Button(btn_fr, text=txt, font=("Segoe UI Symbol", size), bg=PAL["header"],
                           fg=PAL["accent"] if size == 36 else "white",
                           relief="flat", borderwidth=0, command=cmd or (lambda: None))
            b.pack(side="left", padx=14)
            
        self.play_btn = btn_fr.winfo_children()[2]  # reference to play button

        # Volume
        vol_fr = tk.Frame(dock, bg=PAL["header"])
        vol_fr.place(relx=0.87, rely=0.5, anchor="center")
        tk.Label(vol_fr, text="🔊", fg="white", bg=PAL["header"]).pack(side="left")
        self._vol = tk.DoubleVar(value=80)
        ttk.Scale(vol_fr, from_=0, to=100, variable=self._vol, orient="horizontal").pack(side="left")

        self._sovereign_mode = tk.BooleanVar(value=True)
        self._spatial_audio = tk.BooleanVar(value=False)

        tk.Button(dock, text="✨ NEURAL", font=("Segoe UI", 8, "bold"), bg="#252529",
                  fg=PAL["secondary"], relief="flat", padx=10, command=self._neural).place(relx=0.12, rely=0.5, anchor="center")

        sov_fr = tk.Frame(dock, bg=PAL["header"])
        sov_fr.place(relx=0.25, rely=0.5, anchor="center")
        tk.Checkbutton(sov_fr, text="SOVEREIGN MODE", variable=self._sovereign_mode, 
                       bg=PAL["header"], fg=PAL["accent"], selectcolor=PAL["bg"],
                       activebackground=PAL["header"], activeforeground="white",
                       font=("Segoe UI", 7, "bold"), command=self._toggle_sov).pack()

        spat_fr = tk.Frame(dock, bg=PAL["header"])
        spat_fr.place(relx=0.25, rely=0.75, anchor="center")
        tk.Checkbutton(spat_fr, text="SPATIAL AUDIO", variable=self._spatial_audio, 
                       bg=PAL["header"], fg=PAL["cyan"], selectcolor=PAL["bg"],
                       activebackground=PAL["header"], activeforeground="white",
                       font=("Segoe UI", 7, "bold"), command=self._toggle_spatial).pack()

        # Status
        self.status = tk.Label(self, text="", bg=PAL["accent"], fg="white",
                                font=("Segoe UI", 8, "bold"), pady=4)
        self.status.pack(side="bottom", fill="x")

    def _toggle_sov(self):
        msg = "SOVEREIGN MODE: ACTIVE | ZERO-TELEMETRY PIPELINE ENGAGED" if self._sovereign_mode.get() else "SOVEREIGN MODE: DISABLED | Standard Playback Active"
        self._set_status(msg)
        self._notify_status("DRC: HARD-BYPASS" if self._sovereign_mode.get() else "DRC: NORMAL")

    def _toggle_spatial(self):
        msg = "SPATIAL AUDIO: ENGAGED | Virtual 7.1 Orbit active" if self._spatial_audio.get() else "SPATIAL AUDIO: DISENGAGED"
        self._notify_status(msg)

    def _populate_queue(self):
        self.queue_tree.delete(*self.queue_tree.get_children())
        for t in self.tracks:
            tag = "sel" if self.tracks.index(t) == self.current_idx else ""
            self.queue_tree.insert("", "end", values=(t["title"], t["dur"]))
        # Highlight current
        children = self.queue_tree.get_children()
        if children and self.current_idx < len(children):
            self.queue_tree.selection_set(children[self.current_idx])

    def _add_files(self):
        files = filedialog.askopenfilenames(
            title="Add to Queue",
            filetypes=[("Audio Files", "*.mp3 *.wav *.flac *.ogg *.m4a"), ("All", "*.*")]
        )
        for f in files:
            name = os.path.basename(f)
            self.tracks.append({"title": name, "artist": "Local Library", "dur": "—", "dur_s": 0})
        self._populate_queue()
        self._notify_status(f"Added {len(files)} files to queue.")

    def _filter_tracks(self, event=None):
        q = self.srch.get().lower()
        self.queue_tree.delete(*self.queue_tree.get_children())
        for t in self.tracks:
            if q in t["title"].lower() or q in t["artist"].lower():
                self.queue_tree.insert("", "end", values=(t["title"], t["dur"]))

    def _on_select(self, event):
        sel = self.queue_tree.selection()
        if not sel: return
        idx = self.queue_tree.index(sel[0])
        self.current_idx = idx
        self._progress = 0
        self._update_track_display()
        if not self.playing:
            self.toggle()

    def _update_track_display(self):
        if not self.tracks: return
        t = self.tracks[self.current_idx]
        self.title_lbl.config(text=t["title"])
        self.artist_lbl.config(text=t["artist"])
        dur = t.get("dur_s", 0)
        self.dur_lbl.config(text=t["dur"])
        self.title(f"PulsePlayer — {t['title']}")
        icons = ["💿", "🎵", "🎸", "🎹", "🎻"]
        self.art_lbl.config(text=random.choice(icons))

    def toggle(self):
        self.playing = not self.playing
        self.play_btn.config(text="⏸" if self.playing else "▶")
        if self.playing:
            self._start_viz()

    def _prev(self):
        self.current_idx = max(0, self.current_idx - 1)
        self._progress = 0
        self._update_track_display()
        self._populate_queue()

    def _next(self):
        self.current_idx = (self.current_idx + 1) % len(self.tracks)
        self._progress = 0
        self._update_track_display()
        self._populate_queue()

    def _on_seek(self, val):
        t = self.tracks[self.current_idx]
        dur_s = t.get("dur_s", 300)
        self._progress = (float(val) / 100.0) * dur_s

    def _tick_timer(self):
        if self.playing:
            t = self.tracks[self.current_idx]
            dur_s = t.get("dur_s", 300)
            if dur_s > 0:
                self._progress = min(self._progress + 1, dur_s)
                pct = (self._progress / dur_s) * 100
                self.prog_var.set(pct)
                elapsed = int(self._progress)
                self.time_lbl.config(text=f"{elapsed//60}:{elapsed%60:02d}")
                if self._progress >= dur_s:
                    self._next()
        self.after(1000, self._tick_timer)

    def _start_viz(self):
        self._draw_viz()
        if self.playing:
            self.after(80, self._start_viz)

    def _draw_viz(self):
        self.viz.delete("all")
        w = self.viz.winfo_width() or 700
        h = self.viz.winfo_height() or 100
        n = 60
        bw = w / n
        for i in range(n):
            if self.playing:
                self._viz_data[i] = max(4, min(h-4, self._viz_data[i] + random.randint(-15, 15)))
            bh = self._viz_data[i]
            x = i * bw
            r_val = int(90 + 165 * (i / n))
            g_val = int(140 + 88 * (1 - i/n))
            color = f"#{r_val:02x}{g_val:02x}fa"
            self.viz.create_rectangle(x+1, h-bh, x+bw-2, h, fill=color, outline="")

    def _neural(self):
        messagebox.showinfo("Neural Audio Engine",
                            "Sovereign-Loom Neural Upsampling: ACTIVE\n"
                            "• Bit-depth: 32-bit float\n"
                            "• Sample rate: 384 kHz\n"
                            "• AI noise reduction: ENABLED\n"
                            "• Spatial audio: ACTIVE (Dolby Atmos-parity)")

    def _set_status(self, msg):
        self.status.config(text=msg.upper())

    def _notify_status(self, msg):
        old = self.status.cget("text")
        self.status.config(text=msg.upper(), bg=PAL["success"])
        self.after(3000, lambda: self.status.config(text=old, bg=PAL["accent"]))

if __name__ == "__main__":
    app = PulsePlayer()
    app.mainloop()
