# Generated method: BrowserPage._build_ui
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_MED, FONT_SMALL

class BrowserPage:
    def _build_ui(self):
        guardian = self.kernel.registry.get('guardian')
        is_child = guardian and guardian.is_child_mode()
        nav = tk.Frame(self, bg=PAL['bg2'], height=50)
        nav.pack(fill='x')
        nav.pack_propagate(False)
        icon_text = '✨' if is_child else '🛡️'
        self.status_lbl = tk.Label(nav, text=icon_text, font=FONT_MED, fg=PAL['green'], bg=PAL['bg2'])
        self.status_lbl.pack(side='left', padx=10)
        self.url_e = tk.Entry(nav, bg=PAL['bg3'], fg='white', font=FONT_MED, bd=0, insertbackground='white')
        self.url_e.pack(side='left', fill='x', expand=True, padx=5, pady=10)
        initial_url = 'https://kids.search' if is_child else 'https://sigma.search'
        if self.browser and self.browser.tabs:
            initial_url = self.browser.tabs[0]['url']
        self.url_e.insert(0, initial_url)
        shield_fr = tk.Frame(nav, bg=PAL['bg2'])
        shield_fr.pack(side='left', padx=10)
        shield_icons = [('🌈', 'Rainbow Shield'), ('⭐️', 'Star Power'), ('🎈', 'Fun Guard')] if is_child else [('🎭', 'WebGL Fake Values'), ('🖼️', 'Canvas Noise'), ('📍', 'Geo-Spoof')]
        for icon, tooltip in shield_icons:
            btn = tk.Button(shield_fr, text=icon, font=('Segoe UI Symbol', 10), bg=PAL['bg2'], fg=PAL['green'], relief='flat', bd=0, command=lambda t=tooltip: self.gui._log_voice(f'Shield: {t} active.'))
            btn.pack(side='left')
        view = tk.Frame(self, bg='white')
        view.pack(fill='both', expand=True)
        main_text = 'FUN LEARNING WEB' if is_child else 'SOVEREIGN SEARCH'
        self.content_lbl = tk.Label(view, text=main_text, font=('Inter Bold', 24), fg=PAL['bg'], bg='white', wraplength=800)
        self.content_lbl.pack(pady=50)

        def _go(e=None):
            url = self.url_e.get().lower()
            if is_child:
                whitelist = ['khanacademy', 'wikipedia', 'ncert', 'diksha', 'google.com/search?q=kids', 'youtube.com/kids', 'pbskids', 'disneyplus']
                if not any((w in url for w in whitelist)):
                    self.gui._notify('Guardian', 'Site is restricted. Try Khan Academy or Wikipedia!', 'ERR')
                    self.content_lbl.config(text='🚫 SAFETY GUARD ACTIVE\nThis site is not for kids. Happy Learning!')
                    return
            if self.browser:
                self.browser.navigate(self.browser.tabs[0]['id'], url)
                self.status_lbl.config(text='🛰️', fg=PAL['teal'])
                self.after(500, lambda: self.content_lbl.config(text=self.browser.tabs[0].get('content', 'Loading Page...')))
                self.after(1000, lambda: self.status_lbl.config(text=icon_text, fg=PAL['green']))
        self.url_e.bind('<Return>', _go)
        ttk.Button(nav, text='GO', command=_go).pack(side='right', padx=10)
        mode_text = 'Safe & Happy Mode' if is_child else 'React • Python • Nginx'
        self.tech_lbl = tk.Label(nav, text=mode_text, font=('Inter Bold', 8), fg=PAL['cyan'], bg=PAL['bg2'])
        self.tech_lbl.pack(side='right', padx=5)
        surgeon_text = '🖌️ Magic Brush' if is_child else '✂️ Surgeon'
        tk.Button(nav, text=surgeon_text, font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['gold'], relief='flat', command=self._text_surgeon).pack(side='right', padx=5)