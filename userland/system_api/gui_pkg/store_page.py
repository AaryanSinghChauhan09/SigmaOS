import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class StorePage(SigmaPage):
    """Native Sovereign Store — Child-Safe & Multi-Category."""
    
    def __init__(self, parent, gui):
        is_child = gui._is_child_mode()
        title = "Kiddie Toy Shop" if is_child else "Sovereign App Store"
        subtitle = "Fun & Safe Toys for Everyone!" if is_child else "Verified Zero-Trust Applications & Games"
        super().__init__(parent, gui, title, subtitle)
        self._build_ui()

    def _build_ui(self):
        is_child = self.gui._is_child_mode()
        
        # Search + Category Bar
        ctrl_fr = tk.Frame(self, bg=PAL["bg"], pady=8)
        ctrl_fr.pack(fill="x", padx=20)

        self._search_v = tk.StringVar()
        search_e = ttk.Entry(ctrl_fr, textvariable=self._search_v, width=28)
        search_e.pack(side="left", padx=(0, 10))
        tk.Label(ctrl_fr, text="🔍", fg=PAL["dim"], bg=PAL["bg"]).pack(side="left", padx=(0, 10))

        self._store_cat = tk.StringVar(value="All")
        categories = ["All", "Games", "Productivity", "Media"] if is_child else ["All", "AI", "Games", "Dev", "Security", "Productivity", "Media"]
        for cat in categories:
            b = tk.Button(ctrl_fr, text=cat, font=FONT_BOLD, bg=PAL["bg2"], fg=PAL["dim"],
                          relief="flat", padx=10, pady=4,
                          command=lambda c=cat: [self._store_cat.set(c), self._refresh_grid()])
            b.pack(side="left", padx=3)

        # App Grid
        container = tk.Frame(self, bg=PAL["bg"])
        container.pack(fill="both", expand=True, padx=20, pady=10)

        self._canvas = tk.Canvas(container, bg=PAL["bg"], highlightthickness=0)
        self._canvas.pack(side="left", fill="both", expand=True)
        sb = ttk.Scrollbar(container, orient="vertical", command=self._canvas.yview)
        sb.pack(side="right", fill="y")
        self._canvas.configure(yscrollcommand=sb.set)
        
        self._grid = tk.Frame(self._canvas, bg=PAL["bg"])
        self._canvas.create_window((0, 0), window=self._grid, anchor="nw")
        self._grid.bind("<Configure>", lambda e: self._canvas.configure(scrollregion=self._canvas.bbox("all")))

        self._all_apps = [
            ("♟ Happy Chess",           "Games", "Brain Games for Kids.",                "♟", "sigma.game.chess",        PAL["purple"], True),
            ("🎲 Fun Ludo",             "Games", "Play Ludo with Friends.",              "🎲", "sigma.game.ludo",         "#FF9F0A", True),
            ("🚪 Welcome Friend",       "Productivity", "Learn how to use SigmaOS.",    "🚪", "sigma.sys.welcome",       PAL["accent"], True),
            ("📝 Magic Writer",         "Productivity", "Write and draw stories.",      "📝", "sigma.prod.writer",       "#34C759", True),
            ("♫ Happy Musics",          "Media", "Listen to happy music.",             "♫", "sigma.media.pulseplay",   "#5AC8FA", True),
            ("🎨 Color Paint",          "Media", "Paint beautiful pictures.",            "🎨", "sigma.media.aurapaint",   "#FF6B96", True),
            ("🧬 OS Brain",             "AI",  "OS Guide.",    "🧬", "sigma.ai.nexus_ai",       PAL["cyan"], False),
            ("⚡ AI Secret",            "AI",  "AI Power.",    "⚡", "sigma.ai.antigravity",    "#3D9EFF", False),
            ("🔒 Safety Robot",         "Security", "Safety Scan.",    "🔒", "sigma.sys.sentinel",      "#FF453A", False),
        ]

        self._search_v.trace_add("write", lambda *_: self._refresh_grid())
        self._refresh_grid()

    def _refresh_grid(self):
        for w in self._grid.winfo_children(): w.destroy()
        cat_filter = self._store_cat.get()
        q = self._search_v.get().lower()
        child_active = self.gui._is_child_mode()
        
        # Filter logic
        visible = []
        for a in self._all_apps:
            if (cat_filter == "All" or a[1] == cat_filter) and \
               (not q or q in a[0].lower() or q in a[2].lower()) and \
               (not child_active or a[6]):
                visible.append(a)

        for i, (name, tag, desc, icon, aid, color, is_safe) in enumerate(visible):
            r, c = divmod(i, 3)
            card = tk.Frame(self._grid, bg=PAL["card"], width=310, height=230,
                            highlightthickness=1, highlightbackground=PAL["border"])
            card.grid(row=r, column=c, padx=10, pady=10, sticky="nsew")
            card.pack_propagate(False)

            band = tk.Frame(card, bg=color, height=4)
            band.pack(fill="x")

            head = tk.Frame(card, bg=PAL["card"], pady=8)
            head.pack(fill="x", padx=12)
            tk.Label(head, text=icon, font=("Segoe UI Symbol", 22), bg=PAL["card"], fg=color).pack(side="left")
            clean_name = name.lstrip("🧬⚡♟🎲🚪📝♫🎨🔒")
            tk.Label(head, text=clean_name, font=FONT_BOLD, bg=PAL["card"], fg="white").pack(side="left", padx=8)

            tk.Label(card, text=desc, font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"], wraplength=270, justify="left").pack(fill="x", padx=12, pady=8)

            btn = tk.Button(card, text="START", font=("Segoe UI", 9, "bold"), bg=color, fg="white", relief="flat", pady=7,
                            command=lambda a=aid: self.gui._launch_app(a))
            btn.pack(side="bottom", fill="x", padx=12, pady=10)
