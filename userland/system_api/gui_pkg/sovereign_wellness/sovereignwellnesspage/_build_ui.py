"""
Auto-split from userland\system_api\gui_pkg\sovereign_wellness.py — SovereignWellnessPage._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import time
import random
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_MONO



class SovereignWellnessPage:
    def _build_ui(self):
        guardian = self.gui.kernel.registry.get('guardian')
        is_child = guardian and guardian.is_child_mode()
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        top_row = tk.Frame(body, bg=PAL['bg'])
        top_row.pack(fill='x', pady=(0, 10))
        bot_row = tk.Frame(body, bg=PAL['bg'])
        bot_row.pack(fill='both', expand=True)
        focus_title = 'Safety Shield' if is_child else 'Algorithm Shield (YouTube/Social)'
        focus_fr = self._card(top_row, focus_title)
        focus_fr.master.pack(side='left', fill='both', expand=True, padx=(0, 5))
        tk.Label(focus_fr, text='Keep Scary Things Away' if is_child else 'Purge Brain-Rot & Doomscrolling', font=FONT_SMALL, bg=PAL['card'], fg=PAL['dim']).pack(anchor='w')
        if is_child:
            features = [('Block Sad Videos', True), ('Magic Safe Mode', True), ('Bright Lights Off', False), ('New Toys Daily', True)]
        else:
            features = [('Block YouTube Shorts/Reels', True), ('Hide Social Feeds (LinkedIn/X)', True), ('Scene Change Detector (Anti-Seizure)', False), ('Algorithm Reset (Daily Drop)', True)]
        for text, state in features:
            var = tk.BooleanVar(value=state)
            cb = tk.Checkbutton(focus_fr, text=text, variable=var, bg=PAL['card'], fg=PAL['text'], selectcolor=PAL['bg2'], activebackground=PAL['card'])
            cb.pack(anchor='w')
        btn_text = '✨ Keep Me Safe' if is_child else '🚀 Force Digital Fast'
        ttk.Button(focus_fr, text=btn_text, command=lambda: self.gui._log_voice('Shield: Everything is safe and sound.')).pack(fill='x', pady=5)
        bio_title = 'Growing Big & Strong' if is_child else 'Sovereign Bio-Vitality'
        bio_fr = self._card(top_row, bio_title)
        bio_fr.master.pack(side='left', fill='both', expand=True, padx=5)
        if is_child:
            metrics = [('Stand Up Tall', 'Looking Good!', 'ACTIVE'), ('Drink Your Water', 'Glug Glug!', 'LOG'), ('Eye Rest', 'Close your eyes', 'WAIT')]
        else:
            metrics = [('Posture Guardian', 'AI-Slouch Detection', 'ACTIVE'), ('Hydration Gauge', '2.1L / 3.5L Target', 'LOG'), ('Eye Saver (20-20-20)', 'Break in 12m', 'WAIT')]
        for title, desc, stat in metrics:
            m_fr = tk.Frame(bio_fr, bg=PAL['bg2'], pady=4, padx=8)
            m_fr.pack(fill='x', pady=2)
            tk.Label(m_fr, text=title, font=FONT_BOLD, fg=PAL['text'], bg=PAL['bg2']).pack(side='left')
            if stat == 'LOG':
                tk.Button(m_fr, text='+ Bottle' if is_child else '+250ml', bg=PAL['accent'], fg='white', font=('Inter Bold', 7), relief='flat', padx=10, command=lambda: self.gui._log_voice('Yummy water!')).pack(side='right')
            else:
                tk.Label(m_fr, text=stat, font=FONT_SMALL, fg=PAL['cyan'] if stat == 'ACTIVE' else PAL['gold'], bg=PAL['bg2']).pack(side='right')
        wisdom_title = 'Happy Story Time' if is_child else 'Wisdom & Deep Focus'
        wisdom_fr = self._card(bot_row, wisdom_title)
        wisdom_fr.master.pack(side='left', fill='both', expand=True, padx=(0, 5))
        s_tabs = tk.Frame(wisdom_fr, bg=PAL['card'])
        s_tabs.pack(fill='x', pady=(0, 10))
        tab_list = ['Funny', 'Hero', 'Animal', 'Magic'] if is_child else ['Stoic', 'Marcus Aurelius', 'Seneca', 'Zen-Neutral']
        for t in tab_list:
            tk.Button(s_tabs, text=t, font=('Inter Bold', 7), bg=PAL['bg3'], fg=PAL['dim'], relief='flat', padx=8).pack(side='left', padx=2)
        quote_text = '"Be kind to everyone, and keep a big smile on your face!"' if is_child else '"The objective of life is ... Marcus Aurelius"'
        self.quote_disp = tk.Label(wisdom_fr, text=quote_text, font=('Inter Italic', 10), bg=PAL['bg2'], fg=PAL['text'], wraplength=300, pady=15, padx=10)
        self.quote_disp.pack(fill='x', pady=5)
        if is_child:
            btns = [('📖 Read a Story', PAL['cyan']), ('🎨 Color a Picture', PAL['gold']), ('🎈 Play a Game', PAL['teal'])]
        else:
            btns = [('🧠 Deep Work Session: 60m', PAL['cyan']), ('📊 Daily Cognitive Audit', PAL['gold']), ('🔔 Vitality Notifications', PAL['teal'])]
        for b, c in btns:
            tk.Button(wisdom_fr, text=b, font=FONT_SMALL, bg=PAL['bg3'], fg=c, relief='flat', pady=6, anchor='w', command=lambda cmd=b: self._handle_logic(cmd)).pack(fill='x', pady=2)
        zen_title = 'Sleepy Time Sounds' if is_child else 'Zen Master & High-Frequency Healing'
        zen_fr = self._card(bot_row, zen_title)
        zen_fr.master.pack(side='left', fill='both', expand=True, padx=5)
        tk.Label(zen_fr, text='Happy Sounds' if is_child else 'Binaural Retuner (432Hz/528Hz)', font=FONT_SMALL, bg=PAL['card'], fg=PAL['dim']).pack(anchor='w')
        sound_list = ['Twinkle Twinkle', 'Ocean Waves', 'Birds Chirping', 'Raindrops'] if is_child else ['432Hz Solfeggio', '528Hz DNA Repair', 'Alpha Deep Focus', 'White Noise']
        for freq in sound_list:
            tk.Button(zen_fr, text=f'▶ {freq}', font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['text'], relief='flat', pady=4, anchor='w', command=lambda f=freq: self.gui._log_voice(f'Playing {f}')).pack(fill='x', pady=1)
        tk.Label(zen_fr, text='Fun Tools' if is_child else 'Wellness Tools', font=FONT_SMALL, bg=PAL['card'], fg=PAL['dim']).pack(anchor='w', pady=(10, 5))
        if is_child:
            tools = [('🧘 Deep Breathing', self._breathing_overlay), ('🖼️ Picture Book', lambda: self.gui._log_voice('Opening picture book...')), ('🧸 Toy Box', lambda: self.gui._log_voice('Looking for toys...')), ('🎭 Happy Filter', self._toggle_grayscale)]
        else:
            tools = [('🧘 Guided 4-7-8 Breathing', self._breathing_overlay), ('🖼️ Vision Board Mosaic', lambda: self.gui._log_voice('Gen: Synthesizing vision board...')), ('🌡️ AIC/Blood Sugar Log', lambda: self.gui._log_voice('Health: Telemetry log ready.')), ('🎭 Anti-Dopamine Grayscale', self._toggle_grayscale)]
        for label, cmd in tools:
            tk.Button(zen_fr, text=label, font=FONT_SMALL, bg=PAL['bg3'], fg=PAL['accent2'], relief='flat', pady=5, anchor='w', command=cmd).pack(fill='x', pady=1)
        growth_title = 'Learning & Growing' if is_child else 'Growth & Digital Vitality'
        growth_fr = self._card(bot_row, growth_title)
        growth_fr.master.pack(side='left', fill='both', expand=True, padx=(5, 0))
        tk.Label(growth_fr, text='Good Habits' if is_child else 'Habits & Journaling', font=FONT_SMALL, bg=PAL['card'], fg=PAL['dim']).pack(anchor='w')
        habit_list = ['Brush Teeth', 'Eat Vegetables', 'Clean Up Toys', 'Say Thank You'] if is_child else ['Morning Reflection', 'Deep Work (4h)', 'Zero Inbox', 'Workout']
        for habit in habit_list:
            h_v = tk.BooleanVar()
            cb = tk.Checkbutton(growth_fr, text=habit, variable=h_v, bg=PAL['card'], fg=PAL['text'], selectcolor=PAL['bg2'])
            cb.pack(anchor='w')
        tk.Label(growth_fr, text='How Big Am I?' if is_child else 'Health Calcs', font=FONT_SMALL, bg=PAL['card'], fg=PAL['dim']).pack(anchor='w', pady=(10, 5))
        calc_list = ['Height Chart', 'Weight Tracker', 'Energy Meter'] if is_child else ['BMI / TDEE Calc', 'Calorie Tracker', 'Sleep Debt Audit']
        for calc in calc_list:
            tk.Button(growth_fr, text=f'🧮 {calc}', font=FONT_SMALL, bg=PAL['bg2'], fg=PAL['cyan'], relief='flat', anchor='w', command=lambda c=calc: self.gui._log_voice(f'Checking {c}')).pack(fill='x', pady=1)
        if not is_child:
            tk.Label(growth_fr, text='Memento Mori', font=FONT_SMALL, bg=PAL['card'], fg=PAL['dim']).pack(anchor='w', pady=(10, 5))
            tk.Label(growth_fr, text='Life Gauge: 28,471 days remaining', font=FONT_BOLD, bg=PAL['bg2'], fg=PAL['red']).pack(fill='x', pady=2)
        else:
            tk.Label(growth_fr, text='Future Hero', font=FONT_SMALL, bg=PAL['card'], fg=PAL['dim']).pack(anchor='w', pady=(10, 5))
            tk.Label(growth_fr, text='You can be anything!', font=FONT_BOLD, bg=PAL['bg2'], fg=PAL['green']).pack(fill='x', pady=2)
        guardian_title = 'Safe Robot Friend' if is_child else 'AI Health Guardian'
        ai_fr = self._card(body, guardian_title)
        ai_fr.master.pack(fill='x', pady=(10, 0))
        tk.Label(ai_fr, text='Robot Friend says Hello!' if is_child else 'Real-time Cognitive Load Analysis', font=FONT_SMALL, bg=PAL['card'], fg=PAL['dim']).pack(anchor='w')
        guard_msg = "'Hello! I am your robot friend. Remember to stretch and drink some yummy water!'" if is_child else "Guardian: 'You have been in Deep Work...'"
        tk.Label(ai_fr, text=guard_msg, font=('Inter', 9), bg=PAL['bg2'], fg=PAL['accent'], wraplength=700, pady=10).pack(fill='x', pady=5)
        btn_fr = tk.Frame(ai_fr, bg=PAL['card'])
        btn_fr.pack(fill='x')
        btn1_text = '😴 Nap Time' if is_child else '🧘 Start Soul Mode'
        tk.Button(btn_fr, text=btn1_text, font=FONT_BOLD, bg=PAL['accent'], fg='white', relief='flat', padx=20, pady=8, command=lambda: self.gui._log_voice('Shhh... Nap time.')).pack(side='left')
        btn2_text = '🩺 Check Boo-Boo' if is_child else '🩺 AI Symptom Check'
        tk.Button(btn_fr, text=btn2_text, font=FONT_SMALL, bg=PAL['bg3'], fg=PAL['text'], relief='flat', padx=20, pady=8, command=lambda: self.gui._log_voice('Checking boo-boo...')).pack(side='left', padx=10)
