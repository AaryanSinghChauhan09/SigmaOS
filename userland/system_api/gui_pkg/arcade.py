import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL

class ArcadePage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "SOVEREIGN ARCADE", "Zero-Telemetry Clean-Room Game Engine (64+ Logic Modules)")

        # Control Bar: Search & Categories
        ctrl = tk.Frame(self, bg=PAL["nav_bg"], pady=10)
        ctrl.pack(fill="x", padx=10)

        tk.Label(ctrl, text="🔎 SEARCH:", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["nav_bg"]).pack(side="left", padx=(20, 10))
        self.game_query = tk.StringVar()
        self.game_query.trace_add("write", lambda *args: self.refresh_game_grid())
        search_ent = tk.Entry(ctrl, textvariable=self.game_query, bg=PAL["bg"], fg=PAL["text"], insertbackground=PAL["accent"], 
                              font=FONT_SMALL, relief="flat", width=30)
        search_ent.pack(side="left", padx=5)

        tk.Label(ctrl, text="MODE:", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["nav_bg"]).pack(side="left", padx=(30, 10))
        self.cat_filter = tk.StringVar(value="All")
        cat_cb = ttk.Combobox(ctrl, textvariable=self.cat_filter, values=["All", "Board Strategy", "Puzzle / Logic", "Brain Training", "Action / Retro"], state="readonly", width=15)
        cat_cb.pack(side="left", padx=5)
        cat_cb.bind("<<ComboboxSelected>>", lambda e: self.refresh_game_grid())

        # Main Scrolling Grid
        # Check if ScrollableFrame exists in controller or if we need a standard one
        self.scroll_fr = tk.Frame(self, bg=PAL["bg"])
        self.scroll_fr.pack(fill="both", expand=True, padx=10, pady=10)
        
        self.grid_inner = tk.Frame(self.scroll_fr, bg=PAL["bg"])
        self.grid_inner.pack(fill="both", expand=True)

        self.refresh_game_grid()

    def refresh_game_grid(self):
        for child in self.grid_inner.winfo_children(): child.destroy()

        engine = self.controller.kernel.registry.get("games")
        if not engine: return

        query = self.game_query.get().lower()
        cat_filter = self.cat_filter.get()
        
        metadata = engine.get_catalog_metadata()
        
        # Guardian Filtering
        guardian = self.controller.kernel.registry.get("guardian")
        if guardian and guardian.is_child_mode():
            filtered = [g for g in metadata if (query in g["name"].lower() or query in g["id"].lower()) and 
                        (cat_filter == "All" or cat_filter in g["category"]) and
                        guardian.check_access(g.get("age_rating", "G"))]
        else:
            filtered = [g for g in metadata if (query in g["name"].lower() or query in g["id"].lower()) and 
                        (cat_filter == "All" or cat_filter in g["category"])]

        for i, g in enumerate(filtered):
            r, c = divmod(i, 4)
            card = self.controller._card(self.grid_inner, f"{g['icon']} {g['name']}")
            card.master.grid(row=r, column=c, padx=8, pady=8, sticky="nsew")
            
            tk.Label(card, text=f"ENGINE: {g['id']} v{g['version']}", font=("Segoe UI", 7, "bold"), fg=PAL["accent"], bg=PAL["card"]).pack(anchor="w")
            tk.Label(card, text=g['desc'], font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["card"], wraplength=180, justify="left").pack(anchor="w", pady=5)
            
            status_txt = f"READY TO PLAY | {g['size_kb']} KB"
            tk.Label(card, text=status_txt, font=("Segoe UI", 7), fg=PAL["teal"], bg=PAL["card"]).pack(anchor="w")

            def _play(gid=g['id'], name=g['name']):
                self.controller._morphic_island(f"Hydrating {name}...", PAL["teal"])
                engine.play_game(gid)
                app_id = f"sigma.game.{gid.lower()}"
                self.controller._launch_app(app_id)
                
            ttk.Button(card, text="🎮 PLAY NATIVE", command=_play).pack(fill="x", pady=10)
