import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_SMALL, FONT_BOLD, FONT_MONO

class CustomizerPage(SigmaPage):
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, "Customization Studio", "The Living Canvas & Sentient Palette Engine")
        self.build()

    def build(self):
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True)

        l_fr = tk.Frame(body, bg=PAL["bg2"], width=450)
        l_fr.pack(side="left", fill="both", padx=5)
        l_fr.pack_propagate(False)

        # 1. AI Themes
        ai_c = self.gui._card(l_fr, "🌈 Generative Theme Engine")
        ai_c.master.pack(fill="x", pady=5)
        m_var = tk.StringVar(value="Focus")
        for m in ["Focus", "Creative", "Night", "Neon"]:
            tk.Radiobutton(ai_c, text=m, variable=m_var, value=m, bg=PAL["card"], fg=PAL["text"],
                           command=lambda m=m: self.gui._log_voice(self.gui.kernel.registry.get("customizer").generate_ai_theme(m)["message"])).pack(side="left", padx=5)

        # 2. Branding Auras
        aura_c = self.gui._card(l_fr, "✨ Sovereign Branding Auras")
        aura_c.master.pack(fill="x", pady=5)
        tk.Label(aura_c, text="Select an OS Persona:", bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w")
        a_var = tk.StringVar(value="omni")
        auras = ["omni", "nexus", "synergy", "fusion", "prism", "horizon", "unity"]
        for i in range(0, len(auras), 4):
            row = tk.Frame(aura_c, bg=PAL["card"])
            row.pack(fill="x")
            for a in auras[i:i+4]:
                tk.Radiobutton(row, text=a.capitalize(), variable=a_var, value=a, bg=PAL["card"], fg=PAL["text"],
                               command=lambda a=a: self.gui._log_voice(self.gui.kernel.registry.get("customizer").apply_branding_aura(a)["msg"])).pack(side="left", padx=2)

        # 3. Chromatic Palette Orchestration (NEW USP: Sentient Palette)
        color_c = self.gui._card(l_fr, "🖌️ Chromatic Orchestration")
        color_c.master.pack(fill="x", pady=5)
        
        def _apply_colors():
             acc = random.choice(["#FF4757", "#2ED573", "#7B2FBE", "#00FFFF", "#FFD700"])
             bg = random.choice(["#0D0D1A", "#1A1A24", "#0F172A"])
             self.gui._log_voice(self.gui.kernel.registry.get("customizer").apply_color_palette(acc, bg))
             
        ttk.Button(color_c, text="🎲 Randomize Global Palette", command=_apply_colors).pack(side="left", padx=5)
        
        def _trigger_sentient():
            # USP: Sentient Palette - AI detects user "vibe" and shifts colors
            self.gui._morphic_island("Activating Sentient Palette... [SCANNING COGNITIVE LOAD]", PAL["teal"])
            self.gui.after(1500, lambda: self.gui._log_voice("Sentient Palette: Shifting to 'Deep Focus' (Slate/Cyan) based on current IDE activity."))
            
        ttk.Button(color_c, text="🧠 Trigger Sentient Palette", command=_trigger_sentient, style="Teal.TButton").pack(side="left", padx=5)

        # 4. Icon & Layout
        lc_c = self.gui._card(l_fr, "📐 Layout & Icon Packs")
        lc_c.master.pack(fill="x", pady=5)
        tk.Label(lc_c, text="Sidebar:", bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w")
        s_var = tk.StringVar(value="Left")
        for s in ["Left", "Right", "Floating"]:
            tk.Radiobutton(lc_c, text=s, variable=s_var, value=s, bg=PAL["card"], fg=PAL["text"],
                           command=lambda s=s: self.gui._log_voice(self.gui.kernel.registry.get("customizer").switch_layout(s, "Comfortable"))).pack(side="left")
        
        tk.Label(lc_c, text="\nIcon Pack:", bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w")
        p_var = tk.StringVar(value="Sovereign_3D")
        for p in ["Sovereign_3D", "Fluent", "Retro_8Bit"]:
            tk.Radiobutton(lc_c, text=p, variable=p_var, value=p, bg=PAL["card"], fg=PAL["text"],
                           command=lambda p=p: self.gui._log_voice(self.gui.kernel.registry.get("customizer").swap_icon_pack(p))).pack(side="left", padx=5)

        r_fr = tk.Frame(body, bg=PAL["bg"])
        r_fr.pack(side="left", fill="both", expand=True, padx=5)

        # 5. Sound & Physics
        sp_c = self.gui._card(r_fr, "🔉 Acoustics & OS Physics")
        sp_c.master.pack(fill="x", pady=5)
        v_var = tk.StringVar(value="Calm")
        for v in ["Calm", "Mechanical", "Cyber"]:
            tk.Radiobutton(sp_c, text=v, variable=v_var, value=v, bg=PAL["card"], fg=PAL["text"],
                           command=lambda v=v: self.gui._log_voice(self.gui.kernel.registry.get("customizer").apply_soundscape(v))).pack(side="left", padx=5)

        tk.Label(sp_c, text="\nAnimation Curve:", bg=PAL["card"], fg=PAL["dim"]).pack(anchor="w")
        c_var = tk.StringVar(value="Quartic")
        for c in ["Quartic", "Bouncy", "Elastic"]:
            tk.Radiobutton(sp_c, text=c, variable=c_var, value=c, bg=PAL["card"], fg=PAL["text"],
                           command=lambda c=c: self.gui._log_voice(self.gui.kernel.registry.get("customizer").adjust_animation_studio(c, 300))).pack(side="left", padx=5)
                           
        # 6. Typography
        typo_c = self.gui._card(r_fr, "📝 Typography Morpher")
        typo_c.master.pack(fill="x", pady=5)
        def _morph_typo(w, s):
             self.gui._log_voice(self.gui.kernel.registry.get("customizer").morph_fonts(w, s)["message"])
        ttk.Button(typo_c, text="Sleek (Thin, 0.9x)", command=lambda: _morph_typo("Thin", 0.9)).pack(side="left", padx=2)
        ttk.Button(typo_c, text="Standard (Regular, 1x)", command=lambda: _morph_typo("Regular", 1.0)).pack(side="left", padx=2)
        ttk.Button(typo_c, text="Accessible (Bold, 1.3x)", command=lambda: _morph_typo("Bold", 1.3)).pack(side="left", padx=2)
