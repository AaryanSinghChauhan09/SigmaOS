# Generated method: ArcadePage.refresh_game_grid
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL

class ArcadePage:
    def refresh_game_grid(self):
        is_child = self.controller._is_child_mode()
        for child in self.grid_inner.winfo_children():
            child.destroy()
        engine = self.controller.kernel.registry.get('games')
        if not engine:
            return
        query = self.game_query.get().lower()
        cat_filter = self.cat_filter.get()
        metadata = engine.get_catalog_metadata()
        guardian = self.controller.kernel.registry.get('guardian')
        if guardian and guardian.is_child_mode():
            filtered = [g for g in metadata if (query in g['name'].lower() or query in g['id'].lower()) and (cat_filter == 'All' or cat_filter in g['category']) and guardian.check_access(g.get('age_rating', 'G'))]
        else:
            filtered = [g for g in metadata if (query in g['name'].lower() or query in g['id'].lower()) and (cat_filter == 'All' or cat_filter in g['category'])]
        for i, g in enumerate(filtered):
            r, c = divmod(i, 4)
            card = self.controller._card(self.grid_inner, f"{g['icon']} {g['name']}")
            card.master.grid(row=r, column=c, padx=8, pady=8, sticky='nsew')
            sub_head = f"Fun Level: {g['id']}" if is_child else f"ENGINE: {g['id']} v{g['version']}"
            tk.Label(card, text=sub_head, font=('Segoe UI', 7, 'bold'), fg=PAL['accent'], bg=PAL['card']).pack(anchor='w')
            tk.Label(card, text=g['desc'], font=('Segoe UI', 8), fg=PAL['dim'], bg=PAL['card'], wraplength=180, justify='left').pack(anchor='w', pady=5)
            status_txt = 'FUN READY!' if is_child else f"READY TO PLAY | {g['size_kb']} KB"
            tk.Label(card, text=status_txt, font=('Segoe UI', 7), fg=PAL['teal'], bg=PAL['card']).pack(anchor='w')

            def _play(gid=g['id'], name=g['name']):
                load_msg = f'Getting {name} ready...' if is_child else f'Hydrating {name}...'
                self.controller._morphic_island(load_msg, PAL['teal'])
                engine.play_game(gid)
                app_id = f'sigma.game.{gid.lower()}'
                self.controller._launch_app(app_id)
            btn_play_text = '✨ START FUN' if is_child else '🎮 PLAY NATIVE'
            ttk.Button(card, text=btn_play_text, command=_play).pack(fill='x', pady=10)