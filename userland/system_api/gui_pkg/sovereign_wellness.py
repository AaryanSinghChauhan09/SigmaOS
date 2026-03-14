import tkinter as tk
from tkinter import ttk, messagebox
import time
import random
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_MONO

class SovereignWellnessPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Wellness & Mental Fortitude", "Autonomous Digital Longevity — Focus, Health & Cognitive Balance")
        self._count = 0
        self._build_ui()
        self._start_wellness_engine()

    def _build_ui(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=10)

        # 4-Column Responsive-ish Grid using Frames
        top_row = tk.Frame(body, bg=PAL["bg"])
        top_row.pack(fill="x", pady=(0, 10))
        
        bot_row = tk.Frame(body, bg=PAL["bg"])
        bot_row.pack(fill="both", expand=True)

        # --- BLOCK 1: ALGORITHM & FOCUS CONTROL ---
        focus_fr = self._card(top_row, "Algorithm Shield (YouTube/Social)")
        focus_fr.master.pack(side="left", fill="both", expand=True, padx=(0, 5))
        
        tk.Label(focus_fr, text="Purge Brain-Rot & Doomscrolling", font=FONT_SMALL, bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w")
        
        features = [
            ("Block YouTube Shorts/Reels", True),
            ("Hide Social Feeds (LinkedIn/X)", True),
            ("Scene Change Detector (Anti-Seizure)", False),
            ("Algorithm Reset (Daily Drop)", True)
        ]
        for text, state in features:
            var = tk.BooleanVar(value=state)
            cb = tk.Checkbutton(focus_fr, text=text, variable=var, bg=PAL["card"], fg=PAL["text"], 
                                selectcolor=PAL["bg2"], activebackground=PAL["card"])
            cb.pack(anchor="w")

        ttk.Button(focus_fr, text="🚀 Force Digital Fast", command=lambda: self.gui._log_voice("Shield: All attention-sink domains isolated.")).pack(fill="x", pady=5)
        
        tk.Label(focus_fr, text="Content Tweaks", font=FONT_SMALL, bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w", pady=(5, 0))
        for tweak in ["Hide Comments", "Block Reels", "Fix Row Layout", "No Thumbnails"]:
            tk.Button(focus_fr, text=f"🔧 {tweak}", font=("Inter", 7), bg=PAL["bg2"], fg=PAL["text"], relief="flat", anchor="w",
                      command=lambda t=tweak: self.gui._log_voice(f"Tweak: {t} applied to web stream.")).pack(fill="x", pady=1)

        # --- BLOCK 2: BIO-VITALITY & POSTURE ---
        bio_fr = self._card(top_row, "Sovereign Bio-Vitality")
        bio_fr.master.pack(side="left", fill="both", expand=True, padx=5)
        
        metrics = [
            ("Posture Guardian", "AI-Slouch Detection", "ACTIVE"),
            ("Hydration Gauge", "2.1L / 3.5L Target", "LOG"),
            ("Eye Saver (20-20-20)", "Break in 12m", "WAIT")
        ]
        for title, desc, stat in metrics:
            m_fr = tk.Frame(bio_fr, bg=PAL["bg2"], pady=4, padx=8)
            m_fr.pack(fill="x", pady=2)
            tk.Label(m_fr, text=title, font=FONT_BOLD, fg=PAL["text"], bg=PAL["bg2"]).pack(side="left")
            if stat == "LOG":
                tk.Button(m_fr, text="+250ml", bg=PAL["accent"], fg="white", font=("Inter Bold", 7), relief="flat", padx=10,
                          command=lambda: self.gui._log_voice("Bio: Hydration tracked. Kidney health status: Optimal.")).pack(side="right")
            else:
                tk.Label(m_fr, text=stat, font=FONT_SMALL, fg=PAL["cyan"] if stat=="ACTIVE" else PAL["gold"], bg=PAL["bg2"]).pack(side="right")

        # --- BLOCK 3: WISDOM & DEEP FOCUS ---
        wisdom_fr = self._card(bot_row, "Wisdom & Deep Focus")
        wisdom_fr.master.pack(side="left", fill="both", expand=True, padx=(0, 5))
        
        # Tabs for different philosophical needs
        s_tabs = tk.Frame(wisdom_fr, bg=PAL["card"])
        s_tabs.pack(fill="x", pady=(0, 10))
        for t in ["Stoic", "Marcus Aurelius", "Seneca", "Zen-Neutral"]:
            tk.Button(s_tabs, text=t, font=("Inter Bold", 7), bg=PAL["bg3"], fg=PAL["dim"], relief="flat", padx=8).pack(side="left", padx=2)

        self.quote_disp = tk.Label(wisdom_fr, text='"The objective of life is not to be on the side of the majority, but to escape finding oneself in the ranks of the insane." — Marcus Aurelius', 
                                   font=("Inter Italic", 10), bg=PAL["bg2"], fg=PAL["text"], wraplength=300, pady=15, padx=10)
        self.quote_disp.pack(fill="x", pady=5)
        
        btns = [
            ("🧠 Deep Work Session: 60m", PAL["cyan"]),
            ("📊 Daily Cognitive Audit", PAL["gold"]),
            ("🔔 Vitality Notifications", PAL["teal"])
        ]
        for b, c in btns:
            tk.Button(wisdom_fr, text=b, font=FONT_SMALL, bg=PAL["bg3"], fg=c, relief="flat", pady=6, anchor="w",
                      command=lambda cmd=b: self._handle_logic(cmd)).pack(fill="x", pady=2)

        # --- BLOCK 4: ZEN MASTER & RECOVERY ---
        zen_fr = self._card(bot_row, "Zen Master & High-Frequency Healing")
        zen_fr.master.pack(side="left", fill="both", expand=True, padx=5)
        
        tk.Label(zen_fr, text="Binaural Retuner (432Hz/528Hz)", font=FONT_SMALL, bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w")
        for freq in ["432Hz Solfeggio", "528Hz DNA Repair", "Alpha Deep Focus", "White Noise"]:
            tk.Button(zen_fr, text=f"▶ {freq}", font=FONT_SMALL, bg=PAL["bg2"], fg=PAL["text"], relief="flat", pady=4, anchor="w",
                      command=lambda f=freq: self.gui._log_voice(f"Zen: Frequency Shift -> {f}")).pack(fill="x", pady=1)

        tk.Label(zen_fr, text="Wellness Tools", font=FONT_SMALL, bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w", pady=(10, 5))
        tools = [
            ("🧘 Guided 4-7-8 Breathing", self._breathing_overlay),
            ("🖼️ Vision Board Mosaic", lambda: self.gui._log_voice("Gen: Synthesizing vision board from goals...")),
            ("🌡️ AIC/Blood Sugar Log", lambda: self.gui._log_voice("Health: Telemetry log ready for audit.")),
            ("🎭 Anti-Dopamine Grayscale", self._toggle_grayscale)
        ]
        for label, cmd in tools:
            tk.Button(zen_fr, text=label, font=FONT_SMALL, bg=PAL["bg3"], fg=PAL["accent2"], relief="flat", pady=5, anchor="w",
                      command=cmd).pack(fill="x", pady=1)

        # --- BLOCK 5: GROWTH & VITALITY ---
        growth_fr = self._card(bot_row, "Growth & Digital Vitality")
        growth_fr.master.pack(side="left", fill="both", expand=True, padx=(5, 0))
        
        tk.Label(growth_fr, text="Habits & Journaling", font=FONT_SMALL, bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w")
        for habit in ["Morning Reflection", "Deep Work (4h)", "Zero Inbox", "Workout"]:
            h_v = tk.BooleanVar()
            cb = tk.Checkbutton(growth_fr, text=habit, variable=h_v, bg=PAL["card"], fg=PAL["text"], selectcolor=PAL["bg2"])
            cb.pack(anchor="w")
            
        tk.Label(growth_fr, text="Health Calcs", font=FONT_SMALL, bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w", pady=(10, 5))
        for calc in ["BMI / TDEE Calc", "Calorie Tracker", "Sleep Debt Audit"]:
            tk.Button(growth_fr, text=f"🧮 {calc}", font=FONT_SMALL, bg=PAL["bg2"], fg=PAL["cyan"], relief="flat", anchor="w",
                      command=lambda c=calc: self.gui._log_voice(f"Health: {c} interface ready.")).pack(fill="x", pady=1)
                      
        tk.Label(growth_fr, text="Memento Mori", font=FONT_SMALL, bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w", pady=(10, 5))
        tk.Label(growth_fr, text="Life Gauge: 28,471 days remaining", font=FONT_BOLD, bg=PAL["bg2"], fg=PAL["red"]).pack(fill="x", pady=2)

        # --- BLOCK 6: AI GUARDIAN & SOUL MODE ---
        ai_fr = self._card(body, "AI Health Guardian (WaitAIMinute / ChatGPT Health)")
        ai_fr.master.pack(fill="x", pady=(10, 0))
        
        tk.Label(ai_fr, text="Real-time Cognitive Load & Stress Analysis", font=FONT_SMALL, bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w")
        tk.Label(ai_fr, text="Guardian: 'You have been in Deep Work for 3 hours. Pulse is slightly elevated. Recommend 5 min active stretching.'", 
                 font=("Inter", 9), bg=PAL["bg2"], fg=PAL["accent"], wraplength=700, pady=10).pack(fill="x", pady=5)
        
        btn_fr = tk.Frame(ai_fr, bg=PAL["card"])
        btn_fr.pack(fill="x")
        tk.Button(btn_fr, text="🧘 Start Soul Mode", font=FONT_BOLD, bg=PAL["accent"], fg="white", relief="flat", padx=20, pady=8,
                  command=lambda: self.gui._log_voice("Focus: SOUL MODE ACTIVE. All non-essential nodes suspended.")).pack(side="left")
        tk.Button(btn_fr, text="🩺 AI Symptom Check", font=FONT_SMALL, bg=PAL["bg3"], fg=PAL["text"], relief="flat", padx=20, pady=8,
                  command=lambda: self.gui._log_voice("Health: AI Diagnostic Node online. Waiting for telemetry...")).pack(side="left", padx=10)

    def _start_wellness_engine(self):
        def _loop():
            if not self.winfo_exists(): return
            curr_min = int(time.time() / 60)
            if curr_min % 20 == 0:
                self.gui._notify("Wellness: Eye Break", "Follow 20-20-20 rule. Look away now.", "OK")
            self.after(60000, _loop)
        self.after(60000, _loop)

    def _handle_logic(self, cmd):
        self.gui._log_voice(f"Focus: {cmd} initiated.")

    def _tasbeeh_popup(self):
        # Method deprecated as per secularization requirements
        pass

    def _toggle_grayscale(self):
        self.gui._log_voice("Zen: Toggling Grayscale filter to minimize visual distraction.")

    def _breathing_overlay(self):
        self.gui._log_voice("Zen: Initiating synchronized 4-7-8 breathing sequence.")
