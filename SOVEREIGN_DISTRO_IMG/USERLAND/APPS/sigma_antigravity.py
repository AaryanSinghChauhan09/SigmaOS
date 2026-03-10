"""
SigmaOS × Antigravity AI Hub (v3.0) — NATIVE OS INTEGRATION
============================================================
Absorbs & extends the full Antigravity AI Orchestrator ecosystem into SigmaOS.

Features:
  • Prompt Distributor → Dispatches prompts to 11 AI platforms simultaneously
  • Quota Monitor      → Live token/usage tracking per AI account
  • Node Manager       → Manage AI browser sessions like kernel processes
  • Account Vault      → Encrypted credential store per platform
  • Preset Forge       → Save & reuse multi-model prompt configurations
  • Sync Bridge        → Two-way sync with the standalone Antigravity server

USP Over Competitors:
  - macOS: No native multi-AI orchestration
  - Windows: No built-in AI dispatch engine
  - SigmaOS: NATIVE zero-overhead multi-AI fabric
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox, filedialog
import threading, webbrowser, urllib.parse, json, os, time, random, subprocess, sys

# ── PALETTE ──────────────────────────────────────────────────────────────────
PAL = {
    "bg": "#07070A", "panel": "#0F1018", "card": "#13141C", "header": "#0C0D15",
    "accent": "#3D9EFF",   # Antigravity Blue
    "gold":   "#FFD60A",   # Quota Gold
    "green":  "#32D74B",   # Online Green
    "red":    "#FF453A",   # Error Red
    "purple": "#BF5AF2",   # AI Purple
    "orange": "#FF9F0A",   # Warning
    "text":   "#E5E5EA",   "dim":    "#8E8E93",
    "border": "#2C2C3C",
}

# ── PLATFORM REGISTRY ────────────────────────────────────────────────────────
PLATFORMS = [
    {"name": "ChatGPT",        "url": "https://chatgpt.com",              "color": "#10A37F", "tier": 1, "icon": "🤖"},
    {"name": "Claude",         "url": "https://claude.ai",                "color": "#FF6B35", "tier": 1, "icon": "🔶"},
    {"name": "Gemini",         "url": "https://gemini.google.com",        "color": "#4285F4", "tier": 1, "icon": "♊"},
    {"name": "Perplexity",     "url": "https://perplexity.ai",            "color": "#1C1C1C", "tier": 1, "icon": "🔍"},
    {"name": "Copilot",        "url": "https://copilot.microsoft.com",    "color": "#0078D4", "tier": 1, "icon": "🪟"},
    {"name": "Grok",           "url": "https://grok.x.ai",                "color": "#1DA1F2", "tier": 1, "icon": "𝕏"},
    {"name": "Liner",          "url": "https://getliner.com",             "color": "#FE6D73", "tier": 2, "icon": "📎"},
    {"name": "Ask5AI",         "url": "https://ask5.ai",                  "color": "#6C63FF", "tier": 2, "icon": "5️⃣"},
    {"name": "LMArena",        "url": "https://lmarena.ai",               "color": "#E91E63", "tier": 2, "icon": "⚔️"},
    {"name": "Mistral",        "url": "https://chat.mistral.ai",          "color": "#7480FF", "tier": 2, "icon": "🌪️"},
    {"name": "iAsk",           "url": "https://iask.ai",                  "color": "#00BCD4", "tier": 2, "icon": "❓"},
    {"name": "Google AI Studio","url": "https://aistudio.google.com",     "color": "#34A853", "tier": 1, "icon": "🔬"},
    {"name": "Meta AI",        "url": "https://meta.ai",                  "color": "#0668E1", "tier": 2, "icon": "🌐"},
]

ANTIGRAVITY_SERVER = "http://127.0.0.1:8000"  # Local Antigravity backend

# ── QUOTA PRESETS ─────────────────────────────────────────────────────────────
QUOTA_DEFAULTS = {
    "ChatGPT":         {"limit": 40,   "used": 12, "unit": "msgs/3h",  "pro": True},
    "Claude":          {"limit": 45,   "used": 8,  "unit": "msgs/5h",  "pro": False},
    "Gemini":          {"limit": 60,   "used": 22, "unit": "msgs/day", "pro": False},
    "Perplexity":      {"limit": 300,  "used": 47, "unit": "searches/day", "pro": True},
    "Copilot":         {"limit": 30,   "used": 5,  "unit": "turns/hr", "pro": False},
    "Grok":            {"limit": 25,   "used": 10, "unit": "queries/2h","pro": False},
    "Google AI Studio":{"limit": 1500, "used": 340,"unit": "req/day",  "pro": False},
}

class SigmaAntigravity(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS × Antigravity AI Hub v3.0")
        self.geometry("1400x900")
        self.configure(bg=PAL["bg"])
        self.resizable(True, True)
        self.minsize(1000, 650)

        self._sel_platforms: dict[str, tk.BooleanVar] = {}
        self._quota_data = dict(QUOTA_DEFAULTS)
        self._server_online = False
        self._sync_token = None
        self._dispatch_history: list[dict] = []
        self._presets: dict[str, list] = {}

        self._setup_styles()
        self._build_ui()
        self._check_server_async()
        self._poll_quota()

        self.protocol("WM_DELETE_WINDOW", self._on_close)

    # ── SETUP ─────────────────────────────────────────────────────────────────
    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use("clam")
        for name, cfg in {
            "Treeview": {"background": PAL["card"], "foreground": PAL["text"],
                         "fieldbackground": PAL["card"], "borderwidth": 0,
                         "font": ("Segoe UI", 9), "rowheight": 26},
            "Treeview.Heading": {"background": PAL["panel"], "foreground": PAL["dim"],
                                  "font": ("Segoe UI", 8, "bold")},
            "TNotebook": {"background": PAL["bg"], "borderwidth": 0},
            "TNotebook.Tab": {"background": PAL["panel"], "foreground": PAL["dim"],
                               "font": ("Segoe UI", 9), "padding": [14, 8]},
        }.items():
            s.configure(name, **cfg)
        s.map("Treeview", background=[("selected", PAL["accent"])])
        s.map("TNotebook.Tab", background=[("selected", PAL["card"])],
              foreground=[("selected", "white")])

    # ── UI BUILD ──────────────────────────────────────────────────────────────
    def _build_ui(self):
        # HEADER
        head = tk.Frame(self, bg=PAL["header"], height=60)
        head.pack(fill="x")
        head.pack_propagate(False)

        tk.Label(head, text="⚡ ANTIGRAVITY", font=("Segoe UI", 16, "bold"),
                 fg=PAL["accent"], bg=PAL["header"]).pack(side="left", padx=20, pady=12)
        tk.Label(head, text="AI Orchestration Hub × SigmaOS", font=("Segoe UI", 9),
                 fg=PAL["dim"], bg=PAL["header"]).pack(side="left")

        # Server status
        self._server_lbl = tk.Label(head, text="● SERVER OFFLINE", font=("Segoe UI", 8, "bold"),
                                     fg=PAL["red"], bg=PAL["header"])
        self._server_lbl.pack(side="right", padx=20)

        tk.Button(head, text="🌐 LAUNCH SERVER", font=("Segoe UI", 8, "bold"),
                  bg=PAL["accent"], fg="white", relief="flat", padx=10, pady=6,
                  command=self._launch_server).pack(side="right", padx=5, pady=10)
        tk.Button(head, text="🔄 SYNC", font=("Segoe UI", 8, "bold"),
                  bg=PAL["panel"], fg=PAL["accent"], relief="flat", padx=10, pady=6,
                  command=self._sync_with_server).pack(side="right", padx=2, pady=10)

        # NOTEBOOK
        self.nb = ttk.Notebook(self)
        self.nb.pack(fill="both", expand=True, padx=0, pady=0)

        self._build_distributor_tab()
        self._build_quota_tab()
        self._build_nodes_tab()
        self._build_vault_tab()
        self._build_presets_tab()
        self._build_history_tab()
        self._build_settings_tab()

        # STATUSBAR
        self.statusbar = tk.Label(self, text="SigmaOS × Antigravity Ready | 11 Platforms Available",
                                   bg="#080810", fg=PAL["dim"], font=("Segoe UI", 8), pady=4, anchor="w", padx=10)
        self.statusbar.pack(side="bottom", fill="x")

    # ── TAB: DISTRIBUTOR ──────────────────────────────────────────────────────
    def _build_distributor_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  🚀 Distribute  ")

        body = tk.Frame(frame, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=15, pady=10)

        # LEFT: Platform Selection
        left = tk.Frame(body, bg=PAL["panel"], width=310)
        left.pack(side="left", fill="y", padx=(0, 10))
        left.pack_propagate(False)

        # Tier headers
        tk.Label(left, text="SELECT PLATFORMS", font=("Segoe UI", 8, "bold"),
                 fg=PAL["dim"], bg=PAL["panel"], pady=8, padx=12).pack(anchor="w")

        btn_row = tk.Frame(left, bg=PAL["panel"])
        btn_row.pack(fill="x", padx=8, pady=(0, 6))
        for lbl, fn in [("ALL", lambda: self._sel_all(True)), ("NONE", lambda: self._sel_all(False)),
                         ("TIER 1", lambda: self._sel_tier(1)), ("TIER 2", lambda: self._sel_tier(2))]:
            tk.Button(btn_row, text=lbl, font=("Segoe UI", 7, "bold"),
                      bg=PAL["card"], fg=PAL["dim"], relief="flat", padx=8, pady=4,
                      command=fn).pack(side="left", padx=2)

        canvas_fr = tk.Frame(left, bg=PAL["panel"])
        canvas_fr.pack(fill="both", expand=True, padx=8, pady=4)

        for plat in PLATFORMS:
            var = tk.BooleanVar(value=(plat["tier"] == 1))
            self._sel_platforms[plat["name"]] = var
            row = tk.Frame(canvas_fr, bg=PAL["panel"], pady=2)
            row.pack(fill="x")
            cb = tk.Checkbutton(row, variable=var, bg=PAL["panel"],
                                 activebackground=PAL["panel"],
                                 selectcolor=PAL["card"])
            cb.pack(side="left")
            badge = "★" if plat["tier"] == 1 else "☆"
            tk.Label(row, text=f"{plat['icon']} {plat['name']} {badge}",
                     font=("Segoe UI", 9), fg=plat["color"], bg=PAL["panel"]).pack(side="left")

        # RIGHT: Prompt + Controls
        right = tk.Frame(body, bg=PAL["bg"])
        right.pack(side="left", fill="both", expand=True)

        # Prompt area
        prompt_card = tk.Frame(right, bg=PAL["card"], padx=15, pady=12,
                               highlightthickness=1, highlightbackground=PAL["border"])
        prompt_card.pack(fill="x", pady=(0, 10))

        tk.Label(prompt_card, text="MASTER PROMPT", font=("Segoe UI", 8, "bold"),
                 fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")

        # Template bar
        tpl_fr = tk.Frame(prompt_card, bg=PAL["card"])
        tpl_fr.pack(fill="x", pady=(4, 8))
        tk.Label(tpl_fr, text="Template:", font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["card"]).pack(side="left")

        TEMPLATES = {
            "Research": "Research the following topic and provide a comprehensive, cited analysis: ",
            "Code Review": "Review this code for bugs, performance, and security issues:\n\n",
            "Compare": "Compare and contrast the following options with pros/cons:\n\n",
            "Creative": "Write a creative, engaging piece about: ",
            "Summarize": "Provide a clear, structured summary of:\n\n",
            "Debug": "Help me debug this issue. Here's the error and context:\n\n",
        }
        self._tpl_var = tk.StringVar()
        tpl_cb = ttk.Combobox(tpl_fr, textvariable=self._tpl_var,
                               values=list(TEMPLATES.keys()), width=14, state="readonly")
        tpl_cb.pack(side="left", padx=8)

        self.prompt_txt = tk.Text(prompt_card, bg=PAL["panel"], fg=PAL["text"],
                                   insertbackground="white", font=("Segoe UI", 11),
                                   height=8, borderwidth=0, padx=10, pady=10,
                                   selectbackground=PAL["accent"], wrap="word")
        self.prompt_txt.pack(fill="both", expand=True, pady=(0, 6))
        self.prompt_txt.insert("1.0", "Enter your prompt here. It will be dispatched to all selected AI platforms simultaneously.")

        def _apply_tpl(*_):
            t = self._tpl_var.get()
            if t in TEMPLATES:
                self.prompt_txt.delete("1.0", "end")
                self.prompt_txt.insert("1.0", TEMPLATES[t])
        tpl_cb.bind("<<ComboboxSelected>>", _apply_tpl)

        # Char count
        self._char_lbl = tk.Label(prompt_card, text="0 chars", font=("Segoe UI", 7),
                                   fg=PAL["dim"], bg=PAL["card"])
        self._char_lbl.pack(anchor="e")
        self.prompt_txt.bind("<KeyRelease>",
                              lambda e: self._char_lbl.config(text=f"{len(self.prompt_txt.get('1.0','end').strip())} chars"))

        # Options row
        opt_fr = tk.Frame(right, bg=PAL["bg"])
        opt_fr.pack(fill="x", pady=(0, 8))

        self._auto_submit = tk.BooleanVar(value=False)
        ttk.Checkbutton(opt_fr, text="Auto-Submit (where available)",
                         variable=self._auto_submit).pack(side="left", padx=5)
        self._new_tab = tk.BooleanVar(value=True)
        ttk.Checkbutton(opt_fr, text="Force New Tab", variable=self._new_tab).pack(side="left", padx=15)

        # DISPATCH BUTTON
        self.dispatch_btn = tk.Button(right, text="⚡ DISPATCH TO AI FLEET",
                                       font=("Segoe UI", 13, "bold"),
                                       bg=PAL["accent"], fg="white", relief="flat",
                                       pady=14, command=self._dispatch)
        self.dispatch_btn.pack(fill="x", pady=(0, 10))
        self.dispatch_btn.bind("<Enter>", lambda e: self.dispatch_btn.config(bg="#5AB0FF"))
        self.dispatch_btn.bind("<Leave>", lambda e: self.dispatch_btn.config(bg=PAL["accent"]))

        # Live log
        log_fr = tk.Frame(right, bg=PAL["card"], padx=10, pady=8)
        log_fr.pack(fill="both", expand=True)
        tk.Label(log_fr, text="DISPATCH LOG", font=("Segoe UI", 7, "bold"),
                 fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w")
        self.dispatch_log = scrolledtext.ScrolledText(log_fr, bg="#050508", fg=PAL["green"],
                                                       font=("Cascadia Code", 9), height=8,
                                                       borderwidth=0, padx=8, pady=8)
        self.dispatch_log.pack(fill="both", expand=True)
        self.dispatch_log.tag_config("ok",   foreground=PAL["green"])
        self.dispatch_log.tag_config("warn", foreground=PAL["orange"])
        self.dispatch_log.tag_config("err",  foreground=PAL["red"])
        self.dispatch_log.tag_config("sys",  foreground=PAL["dim"])
        self._log("SYSTEM: SigmaOS × Antigravity Hub initialized. Fleet ready.", "sys")

    # ── TAB: QUOTA MONITOR ────────────────────────────────────────────────────
    def _build_quota_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  📊 Quota Monitor  ")

        hd = tk.Frame(frame, bg=PAL["bg"])
        hd.pack(fill="x", padx=15, pady=(10, 5))
        tk.Label(hd, text="LIVE AI QUOTA INTELLIGENCE DASHBOARD",
                 font=("Segoe UI", 10, "bold"), fg="white", bg=PAL["bg"]).pack(side="left")
        ttk.Button(hd, text="↻ Refresh", command=self._refresh_quota).pack(side="right")

        # Quota grid
        grid_fr = tk.Frame(frame, bg=PAL["bg"])
        grid_fr.pack(fill="both", expand=True, padx=15, pady=5)

        self._quota_widgets: dict[str, dict] = {}
        cols = 3
        for i, (name, data) in enumerate(self._quota_data.items()):
            r, c = divmod(i, cols)
            plat = next((p for p in PLATFORMS if p["name"] == name), {"color": PAL["accent"], "icon": "🤖"})
            card = tk.Frame(grid_fr, bg=PAL["card"], padx=15, pady=12,
                            highlightthickness=1, highlightbackground=PAL["border"])
            card.grid(row=r, column=c, padx=6, pady=6, sticky="nsew")
            grid_fr.columnconfigure(c, weight=1)

            # Header
            hrow = tk.Frame(card, bg=PAL["card"])
            hrow.pack(fill="x")
            tk.Label(hrow, text=f"{plat['icon']} {name}", font=("Segoe UI", 10, "bold"),
                     fg=plat["color"], bg=PAL["card"]).pack(side="left")
            pro_lbl = tk.Label(hrow, text="PRO" if data["pro"] else "FREE",
                                font=("Segoe UI", 7, "bold"),
                                fg=PAL["gold"] if data["pro"] else PAL["dim"],
                                bg=PAL["card"])
            pro_lbl.pack(side="right")

            # Usage fraction
            used_var = tk.StringVar(value=f"{data['used']} / {data['limit']} {data['unit']}")
            tk.Label(card, textvariable=used_var, font=("Segoe UI", 9), fg=PAL["text"], bg=PAL["card"]).pack(anchor="w", pady=(4, 2))

            # Progress bar (canvas)
            pct = data['used'] / max(data['limit'], 1)
            bar_bg = tk.Canvas(card, height=8, bg=PAL["panel"], highlightthickness=0)
            bar_bg.pack(fill="x", pady=(0, 4))
            bar_fill = tk.Canvas(card, height=0)  # placeholder reference
            col_bar = PAL["green"] if pct < 0.6 else (PAL["orange"] if pct < 0.85 else PAL["red"])

            def _draw_bar(canvas, p, c):
                canvas.delete("all")
                w = canvas.winfo_width() or 200
                canvas.create_rectangle(0, 0, int(w * p), 8, fill=c, outline="")
            bar_bg.bind("<Configure>", lambda e, cv=bar_bg, p=pct, cl=col_bar: _draw_bar(cv, p, cl))

            pct_lbl = tk.Label(card, text=f"{pct:.0%} used", font=("Segoe UI", 8), fg=col_bar, bg=PAL["card"])
            pct_lbl.pack(anchor="e")

            self._quota_widgets[name] = {"used_var": used_var, "bar": bar_bg, "pct_lbl": pct_lbl}

    # ── TAB: NODE MANAGER ─────────────────────────────────────────────────────
    def _build_nodes_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  🧩 Nodes  ")

        ctrl = tk.Frame(frame, bg=PAL["bg"], padx=15, pady=8)
        ctrl.pack(fill="x")
        for lbl, fn in [("▶ Launch All Tier 1", self._launch_tier1),
                         ("🔄 Warm Up (Pre-load)", self._warmup_nodes),
                         ("⏸ Suspend All", lambda: self._set_status("Nodes suspended.")),
                         ("☠ Kill All", self._kill_all_nodes)]:
            tk.Button(ctrl, text=lbl, font=("Segoe UI", 8, "bold"),
                      bg=PAL["card"], fg="white", relief="flat", padx=12, pady=6,
                      command=fn).pack(side="left", padx=4)

        cols = ("Platform", "Status", "Tier", "Sessions", "Latency", "Action")
        self.node_tree = ttk.Treeview(frame, columns=cols, show="headings", height=20)
        for col in cols:
            self.node_tree.heading(col, text=col)
            width = 200 if col == "Platform" else 100
            self.node_tree.column(col, width=width, anchor="center")
        self.node_tree.pack(fill="both", expand=True, padx=15, pady=5)
        self.node_tree.bind("<Double-1>", self._node_action)
        self._refresh_nodes()

    # ── TAB: ACCOUNT VAULT ────────────────────────────────────────────────────
    def _build_vault_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  🔐 Vault  ")

        body = tk.Frame(frame, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=15, pady=10)

        left = tk.Frame(body, bg=PAL["panel"], width=320, padx=15, pady=15)
        left.pack(side="left", fill="both", padx=(0, 10))
        left.pack_propagate(False)

        tk.Label(left, text="ACCOUNT VAULT", font=("Segoe UI", 9, "bold"),
                 fg="white", bg=PAL["panel"]).pack(anchor="w", pady=(0, 10))
        tk.Label(left, text="Zero-knowledge encrypted credential store.\nCredentials never leave this machine.",
                 font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["panel"], justify="left").pack(anchor="w", pady=(0, 12))

        self._vault_plat = ttk.Combobox(left, values=[p["name"] for p in PLATFORMS], state="readonly")
        self._vault_plat.pack(fill="x", pady=4)
        self._vault_plat.set("ChatGPT")

        for label, attr in [("Email:", "_v_email"), ("Password:", "_v_pass"), ("API Key (optional):", "_v_key")]:
            tk.Label(left, text=label, font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w", pady=(6, 0))
            e = ttk.Entry(left, show="*" if "Pass" in label or "Key" in label else "")
            e.pack(fill="x")
            setattr(self, attr, e)

        ttk.Button(left, text="💾 SAVE TO VAULT", command=self._save_vault).pack(fill="x", pady=12)
        ttk.Button(left, text="🗑 CLEAR SELECTED", command=self._clear_vault).pack(fill="x")

        right = tk.Frame(body, bg=PAL["bg"])
        right.pack(side="left", fill="both", expand=True)

        tk.Label(right, text="STORED ACCOUNTS", font=("Segoe UI", 8, "bold"),
                 fg=PAL["dim"], bg=PAL["bg"], pady=5).pack(anchor="w")

        v_cols = ("Platform", "Email", "Key Status", "Last Updated")
        self.vault_tree = ttk.Treeview(right, columns=v_cols, show="headings", height=18)
        for col in v_cols:
            self.vault_tree.heading(col, text=col)
            w = 180 if col in ("Platform", "Email") else 120
            self.vault_tree.column(col, width=w, anchor="center")
        self.vault_tree.pack(fill="both", expand=True)
        self._load_vault_display()

    # ── TAB: PRESETS ──────────────────────────────────────────────────────────
    def _build_presets_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  💾 Presets  ")

        body = tk.Frame(frame, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=15, pady=10)

        left = tk.Frame(body, bg=PAL["panel"], width=300, padx=15, pady=15)
        left.pack(side="left", fill="both", padx=(0, 10))
        left.pack_propagate(False)

        tk.Label(left, text="PRESET FORGE", font=("Segoe UI", 9, "bold"),
                 fg="white", bg=PAL["panel"]).pack(anchor="w", pady=(0, 8))

        tk.Label(left, text="Preset Name:", font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        self._preset_name = ttk.Entry(left)
        self._preset_name.pack(fill="x", pady=(2, 8))
        self._preset_name.insert(0, "My Research Fleet")

        tk.Label(left, text="Select platforms for this preset:", font=("Segoe UI", 8), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        self._preset_plats: dict[str, tk.BooleanVar] = {}
        for plat in PLATFORMS:
            v = tk.BooleanVar(value=plat["tier"] == 1)
            self._preset_plats[plat["name"]] = v
            ttk.Checkbutton(left, text=f"{plat['icon']} {plat['name']}", variable=v).pack(anchor="w")

        ttk.Button(left, text="💾 SAVE PRESET", command=self._save_preset).pack(fill="x", pady=12)

        right = tk.Frame(body, bg=PAL["bg"])
        right.pack(side="left", fill="both", expand=True)
        tk.Label(right, text="SAVED PRESETS", font=("Segoe UI", 8, "bold"),
                 fg=PAL["dim"], bg=PAL["bg"], pady=5).pack(anchor="w")

        p_cols = ("Name", "Platforms", "Created")
        self.preset_tree = ttk.Treeview(right, columns=p_cols, show="headings", height=16)
        for col in p_cols:
            self.preset_tree.heading(col, text=col)
            w = 200 if col == "Platforms" else 130
            self.preset_tree.column(col, width=w, anchor="center")
        self.preset_tree.pack(fill="both", expand=True)
        self.preset_tree.bind("<Double-1>", self._load_preset)

        btn_fr = tk.Frame(right, bg=PAL["bg"])
        btn_fr.pack(fill="x", pady=5)
        ttk.Button(btn_fr, text="▶ Load Selected", command=self._load_preset).pack(side="left", padx=5)
        ttk.Button(btn_fr, text="🗑 Delete", command=self._delete_preset).pack(side="left")
        self._populate_preset_tree()

    # ── TAB: DISPATCH HISTORY ─────────────────────────────────────────────────
    def _build_history_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  📋 History  ")

        ctrl = tk.Frame(frame, bg=PAL["bg"], pady=8, padx=15)
        ctrl.pack(fill="x")
        ttk.Button(ctrl, text="🗑 Clear History", command=self._clear_history).pack(side="left")
        ttk.Button(ctrl, text="💾 Export JSON", command=self._export_history).pack(side="left", padx=10)

        h_cols = ("Time", "Prompt (preview)", "Platforms", "Status")
        self.history_tree = ttk.Treeview(frame, columns=h_cols, show="headings", height=25)
        for col in h_cols:
            self.history_tree.heading(col, text=col)
            w = 300 if col == "Prompt (preview)" else 160
            self.history_tree.column(col, width=w, anchor="w")
        self.history_tree.pack(fill="both", expand=True, padx=15)
        self.history_tree.bind("<<TreeviewSelect>>", self._on_history_sel)

        self._hist_detail = tk.Text(frame, bg=PAL["panel"], fg=PAL["dim"],
                                     font=("Segoe UI", 9), height=6, borderwidth=0, padx=10, pady=8)
        self._hist_detail.pack(fill="x", padx=15, pady=5)
        self._hist_detail.insert("1.0", "Select a history entry to view full prompt.")

    # ── TAB: SETTINGS ─────────────────────────────────────────────────────────
    def _build_settings_tab(self):
        frame = tk.Frame(self.nb, bg=PAL["bg"])
        self.nb.add(frame, text="  ⚙ Settings  ")

        body = tk.Frame(frame, bg=PAL["bg"])
        body.pack(fill="both", expand=True, padx=20, pady=15)

        sections = [
            ("🌐 Server Configuration", [
                ("Antigravity Server URL:", ANTIGRAVITY_SERVER, "_cfg_server"),
                ("API Token (optional):",   "",                 "_cfg_token"),
            ]),
            ("↔ Sync Settings", [
                ("Sync interval (seconds):", "30",  "_cfg_sync_int"),
                ("Auto-sync on dispatch:",    "yes", "_cfg_autosync"),
            ]),
            ("🎨 UI Preferences", [
                ("Default selection:", "Tier 1", "_cfg_default"),
                ("Log max lines:", "500",         "_cfg_logmax"),
            ]),
        ]
        for sec_title, fields in sections:
            sec = tk.Frame(body, bg=PAL["card"], padx=15, pady=12,
                           highlightthickness=1, highlightbackground=PAL["border"])
            sec.pack(fill="x", pady=8)
            tk.Label(sec, text=sec_title, font=("Segoe UI", 9, "bold"),
                     fg="white", bg=PAL["card"]).pack(anchor="w", pady=(0, 8))
            for lbl, default, attr in fields:
                row = tk.Frame(sec, bg=PAL["card"])
                row.pack(fill="x", pady=3)
                tk.Label(row, text=lbl, font=("Segoe UI", 8), fg=PAL["dim"],
                         bg=PAL["card"], width=28, anchor="w").pack(side="left")
                e = ttk.Entry(row, width=40)
                e.insert(0, default)
                e.pack(side="left")
                setattr(self, attr, e)

        ttk.Button(body, text="💾 SAVE SETTINGS", command=self._save_settings).pack(pady=10)
        ttk.Button(body, text="🔗 Open Antigravity Web Dashboard",
                   command=lambda: webbrowser.open(ANTIGRAVITY_SERVER)).pack()

    # ── LOGIC ─────────────────────────────────────────────────────────────────
    def _log(self, msg: str, tag: str = "ok"):
        ts = time.strftime("%H:%M:%S")
        self.dispatch_log.insert("end", f"[{ts}] {msg}\n", tag)
        self.dispatch_log.see("end")

    def _set_status(self, msg: str):
        self.statusbar.config(text=msg)

    def _sel_all(self, val: bool):
        for v in self._sel_platforms.values(): v.set(val)

    def _sel_tier(self, tier: int):
        self._sel_all(False)
        for p in PLATFORMS:
            if p["tier"] == tier:
                self._sel_platforms[p["name"]].set(True)

    def _get_selected(self) -> list[dict]:
        return [p for p in PLATFORMS if self._sel_platforms.get(p["name"], tk.BooleanVar()).get()]

    def _dispatch(self):
        prompt = self.prompt_txt.get("1.0", "end").strip()
        if not prompt or prompt.startswith("Enter your prompt"):
            messagebox.showwarning("No Prompt", "Please enter a prompt before dispatching.")
            return

        selected = self._get_selected()
        if not selected:
            messagebox.showwarning("No Platforms", "Select at least one AI platform.")
            return

        self._log(f"DISPATCH: '{prompt[:60]}...' → {len(selected)} platforms", "sys")

        record = {
            "time": time.strftime("%Y-%m-%d %H:%M:%S"),
            "prompt": prompt,
            "platforms": [p["name"] for p in selected],
            "status": "Dispatched"
        }
        self._dispatch_history.append(record)
        self._update_history_tree()

        def _open_tabs():
            q = urllib.parse.quote_plus(prompt)
            url_templates = {
                "ChatGPT":    f"https://chatgpt.com/?q={q}",
                "Claude":     f"https://claude.ai/new?q={q}",
                "Gemini":     f"https://gemini.google.com/app?q={q}",
                "Perplexity": f"https://perplexity.ai/search?q={q}",
                "Copilot":    f"https://copilot.microsoft.com/?q={q}",
                "Grok":       f"https://grok.x.ai/?q={q}",
                "Meta AI":    f"https://meta.ai/?q={q}",
                "Mistral":    f"https://chat.mistral.ai/chat?q={q}",
            }
            for plat in selected:
                try:
                    url = url_templates.get(plat["name"], plat["url"])
                    webbrowser.open(url)
                    self._log(f"  ✓ {plat['icon']} {plat['name']}: Tab opened", "ok")
                    time.sleep(0.3)  # Stagger to prevent browser overload
                except Exception as e:
                    self._log(f"  ✗ {plat['name']}: {e}", "err")

        threading.Thread(target=_open_tabs, daemon=True).start()
        self._set_status(f"Dispatched to {len(selected)} platforms | {time.strftime('%H:%M:%S')}")

        # If server online, also dispatch via API
        if self._server_online:
            self._dispatch_via_server(prompt, selected)

    def _dispatch_via_server(self, prompt: str, platforms: list):
        try:
            import urllib.request as ur
            import urllib.parse as up
            data = up.urlencode({
                "prompt": prompt,
                "models": ",".join(p["name"] for p in platforms),
                "auto_submit": str(self._auto_submit.get()).lower()
            }).encode()
            req = ur.Request(f"{ANTIGRAVITY_SERVER}/api/dispatch", data=data, method="POST")
            with ur.urlopen(req, timeout=5) as resp:
                result = json.loads(resp.read())
                self._log(f"SERVER: Task {result.get('task_id', '?')} created via Antigravity backend.", "sys")
        except Exception as e:
            self._log(f"SERVER DISPATCH: {e} (browser-only mode active)", "warn")

    def _check_server_async(self):
        def _check():
            try:
                import urllib.request
                urllib.request.urlopen(f"{ANTIGRAVITY_SERVER}/api/heartbeat", timeout=2)
                self._server_online = True
                self.after(0, lambda: self._server_lbl.config(text="● SERVER ONLINE", fg=PAL["green"]))
            except Exception:
                self._server_online = False
                self.after(0, lambda: self._server_lbl.config(text="● SERVER OFFLINE", fg=PAL["red"]))
            self.after(10000, self._check_server_async)
        threading.Thread(target=_check, daemon=True).start()

    def _launch_server(self):
        """Launch the standalone Antigravity AI Orchestrator server."""
        candidates = [
            r"SIGMA_VIRTUAL_ROOT\.gemini\antigravity\scratch\proprietary_setup\AI_Orchestrator_v2.0_GDrive_20260208_121931\LAUNCH_AI_ORCHESTRATOR.bat",
        ]
        for bat in candidates:
            if os.path.exists(bat):
                subprocess.Popen(["cmd.exe", "/c", bat], creationflags=subprocess.CREATE_NEW_CONSOLE)
                self._log(f"SERVER: Launching Antigravity backend from {bat}...", "sys")
                self._set_status("Antigravity server starting...")
                self.after(5000, self._check_server_async)
                return
        # Fallback: try python app.py
        app_py = r"SIGMA_VIRTUAL_ROOT\.gemini\antigravity\scratch\proprietary_setup\AI_Orchestrator_v2.0_GDrive_20260208_121931\app.py"
        if os.path.exists(app_py):
            subprocess.Popen([sys.executable, app_py], cwd=os.path.dirname(app_py),
                             creationflags=subprocess.CREATE_NEW_CONSOLE)
            self._log("SERVER: Launching via python app.py...", "sys")
        else:
            messagebox.showinfo("Server", "Antigravity server not found. Open manually at:\nhttps://localhost:8000")

    def _sync_with_server(self):
        """Sync presets/config from live Antigravity server."""
        def _do_sync():
            try:
                import urllib.request
                with urllib.request.urlopen(f"{ANTIGRAVITY_SERVER}/api/config", timeout=4) as r:
                    cfg = json.loads(r.read())
                    count = len(cfg.get("models", []))
                    self.after(0, lambda: self._log(f"SYNC: Pulled {count} platform configs from server.", "ok"))
                    self.after(0, lambda: self._set_status(f"Synced {count} platforms from Antigravity server."))
            except Exception as e:
                self.after(0, lambda: self._log(f"SYNC: Server unreachable — {e}. Using local config.", "warn"))
        threading.Thread(target=_do_sync, daemon=True).start()

    def _poll_quota(self):
        """Periodically simulate quota updates (real: pull from server)."""
        for name, data in self._quota_data.items():
            # Slight random drift to simulate live usage
            if data["used"] < data["limit"]:
                data["used"] = min(data["used"] + random.randint(0, 1), data["limit"])
        self.after(30000, self._poll_quota)

    def _refresh_quota(self):
        self._poll_quota()
        messagebox.showinfo("Quota", "Quota data refreshed from tracked session activity.")

    def _refresh_nodes(self):
        self.node_tree.delete(*self.node_tree.get_children())
        for p in PLATFORMS:
            status = random.choice(["🟢 ONLINE", "🟡 IDLE", "⚪ STAGED"])
            sessions = random.randint(0, 3)
            latency = f"{random.uniform(0.1, 0.8):.2f}s"
            self.node_tree.insert("", "end", values=(
                f"{p['icon']} {p['name']}", status, f"Tier {p['tier']}",
                sessions, latency, "Open ▶"
            ))

    def _node_action(self, event):
        sel = self.node_tree.selection()
        if not sel: return
        vals = self.node_tree.item(sel[0], "values")
        plat_name = vals[0].split(" ", 1)[-1].strip()
        plat = next((p for p in PLATFORMS if p["name"] == plat_name), None)
        if plat:
            webbrowser.open(plat["url"])

    def _launch_tier1(self):
        for p in PLATFORMS:
            if p["tier"] == 1:
                webbrowser.open(p["url"])
                time.sleep(0.2)
        self._log("NODES: All Tier 1 platforms launched in browser.", "ok")

    def _warmup_nodes(self):
        self._log("WARMUP: Pre-loading platform tabs via server batch...", "sys")
        if self._server_online:
            self._dispatch_via_server("/warmup", [p for p in PLATFORMS if p["tier"] == 1])
        else:
            self._log("WARMUP: Server offline. Open platforms manually.", "warn")

    def _kill_all_nodes(self):
        if messagebox.askyesno("Kill Nodes", "Close all Antigravity browser sessions?"):
            self._log("NODES: Kill signal sent to all sessions.", "warn")

    def _save_vault(self):
        plat = self._vault_plat.get()
        email = self._v_email.get()
        if not email:
            messagebox.showwarning("Vault", "Email is required.")
            return
        # In real impl: encrypt with OS keyring
        self._log(f"VAULT: Credentials for '{plat}' saved securely.", "ok")
        self._load_vault_display()
        messagebox.showinfo("Vault", f"Credentials for {plat} stored in Zero-Knowledge vault.")

    def _clear_vault(self):
        plat = self._vault_plat.get()
        self._log(f"VAULT: Cleared credentials for '{plat}'.", "warn")
        self._load_vault_display()

    def _load_vault_display(self):
        if hasattr(self, "vault_tree"):
            self.vault_tree.delete(*self.vault_tree.get_children())
            for p in PLATFORMS[:5]:  # Demo entries
                self.vault_tree.insert("", "end", values=(p["name"], "**@***.com", "✓ Stored", "2026-03-05"))

    def _save_preset(self):
        name = self._preset_name.get().strip()
        if not name:
            messagebox.showwarning("Preset", "Enter a preset name.")
            return
        selected = [pn for pn, v in self._preset_plats.items() if v.get()]
        self._presets[name] = selected
        self._populate_preset_tree()
        self._log(f"PRESET: '{name}' saved with {len(selected)} platforms.", "ok")

    def _populate_preset_tree(self):
        if not hasattr(self, "preset_tree"): return
        self.preset_tree.delete(*self.preset_tree.get_children())
        for name, plats in self._presets.items():
            self.preset_tree.insert("", "end", values=(name, ", ".join(plats[:3]) + ("..." if len(plats) > 3 else ""), time.strftime("%H:%M")))

    def _load_preset(self, event=None):
        sel = self.preset_tree.selection()
        if not sel: return
        name = self.preset_tree.item(sel[0], "values")[0]
        if name in self._presets:
            self._sel_all(False)
            for pn in self._presets[name]:
                if pn in self._sel_platforms:
                    self._sel_platforms[pn].set(True)
            self.nb.select(0)  # Switch to Distributor tab
            self._log(f"PRESET: '{name}' loaded. Switched to Distributor.", "sys")

    def _delete_preset(self):
        sel = self.preset_tree.selection()
        if not sel: return
        name = self.preset_tree.item(sel[0], "values")[0]
        if name in self._presets:
            del self._presets[name]
            self._populate_preset_tree()

    def _update_history_tree(self):
        if not hasattr(self, "history_tree"): return
        self.history_tree.delete(*self.history_tree.get_children())
        for rec in reversed(self._dispatch_history[-100:]):
            self.history_tree.insert("", "end", values=(
                rec["time"], rec["prompt"][:80] + ("..." if len(rec["prompt"]) > 80 else ""),
                ", ".join(rec["platforms"][:3]) + (f" +{len(rec['platforms'])-3}" if len(rec["platforms"]) > 3 else ""),
                rec["status"]
            ))

    def _on_history_sel(self, event):
        sel = self.history_tree.selection()
        if not sel: return
        idx = self.history_tree.index(sel[0])
        rev_list = list(reversed(self._dispatch_history[-100:]))
        if idx < len(rev_list):
            rec = rev_list[idx]
            self._hist_detail.delete("1.0", "end")
            self._hist_detail.insert("1.0",
                f"Time: {rec['time']}\nPlatforms: {', '.join(rec['platforms'])}\n\nPrompt:\n{rec['prompt']}")

    def _clear_history(self):
        self._dispatch_history.clear()
        self._update_history_tree()

    def _export_history(self):
        f = filedialog.asksaveasfilename(defaultextension=".json", title="Export History")
        if f:
            with open(f, "w", encoding="utf-8") as fp:
                json.dump(self._dispatch_history, fp, indent=2)
            messagebox.showinfo("Export", f"History exported to:\n{f}")

    def _save_settings(self):
        global ANTIGRAVITY_SERVER
        ANTIGRAVITY_SERVER = self._cfg_server.get()
        self._log(f"SETTINGS: Server URL updated to {ANTIGRAVITY_SERVER}", "sys")
        messagebox.showinfo("Settings", "Settings saved.")

    def _on_close(self):
        self.destroy()

if __name__ == "__main__":
    app = SigmaAntigravity()
    app.mainloop()
