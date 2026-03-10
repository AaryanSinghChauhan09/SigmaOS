"""
SigmaOS Global Configuration Manager
Centralizes all tunable parameters, paths, and runtime settings.
"""
import os
import json
from pathlib import Path

class SigmaConfig:
    """Singleton configuration store for the entire SigmaOS ecosystem."""
    _instance = None
    SILENT = False

    def __new__(cls, *args, **kwargs):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            cls._instance._initialized = False
        return cls._instance

    def __init__(self):
        if self._initialized:
            return
        self._initialized = True

        self.BASE_DIR = Path(__file__).resolve().parent.parent
        self.KERNEL_DIR  = self.BASE_DIR / "userland" / "system_api"
        self.NATIVE_KERNEL_DIR = self.BASE_DIR / "kernel"
        self.ECOSYSTEM_DIR = self.BASE_DIR / "ecosystem"
        self.WORKSPACE_DIR = self.BASE_DIR / "workspace"
        self.ASSETS_DIR    = self.BASE_DIR / "assets"
        self.LOGS_DIR      = self.BASE_DIR / "logs"
        self.CONFIG_FILE   = self.BASE_DIR / "sigma_config.json"

        # OS Meta
        self.OS_NAME    = "SigmaOS Sovereign"
        self.VERSION    = "2.0-Apex"
        self.BUILD      = "Sovereign-Lattice"
        self.BASE_KERNEL = "Hardened Linux LTS"

        # UI Modules and their corresponding functions/classes
        self.UI_MODULES = [
            ("sigma_silo_manager","SigmaSiloManager",        "silo_manager"),
            ("quick_settings","SigmaQuickSettings",        "quick_settings"),
        ]

        # Performance defaults
        self.PERF = {
            "vm.swappiness": 5,
            "vm.vfs_cache_pressure": 40,
            "kernel.sched_autogroup_enabled": 1,
            "kernel.kptr_restrict": 2,
            "net.ipv4.tcp_fastopen": 3,
        }

        # ==== New Tunable Modes ====
        self.MODES = {
            "default": {
                "NO_ANIM": False,
                "LAZY_LOADING": False,
                "ASYNC_UPDATES": False,
                "ENERGY_COOLDOWN": False,
                "AI_CACHING": False,
                "AI_MICROSERVICE": False,
            },
            "tuned": {
                "NO_ANIM": True,
                "LAZY_LOADING": True,
                "ASYNC_UPDATES": True,
                "ENERGY_COOLDOWN": True,
                "AI_CACHING": True,
                "AI_MICROSERVICE": True,
            },
            "apex": {
                "NO_ANIM": True,
                "LAZY_LOADING": True,
                "ASYNC_UPDATES": True,
                "ENERGY_COOLDOWN": True,
                "AI_CACHING": True,
                "AI_MICROSERVICE": True,
                "HYPER_DRIVE": True,
                "ULTRA_LOW_LATENCY": True,
            }
        }
        
        # ==== Personalization: Vibes & Personas ====
        self.VIBES = {
            "Minimalist": {"accent": "#5AC8FA", "font": "Inter", "complexity": "low", "focus": "zen"},
            "Cyberpunk":  {"accent": "#FF2D55", "font": "JetBrains Mono", "complexity": "high", "focus": "performance"},
            "Enterprise": {"accent": "#007AFF", "font": "Segoe UI", "complexity": "med", "focus": "productivity"},
            "Gamer":      {"accent": "#AF52DE", "font": "Outfit", "complexity": "max", "focus": "hyper-drive"}
        }
        self.ACTIVE_VIBE = "Enterprise"
        
        self.USER_PERSONA = {
            "name": "Sovereign-User Sovereign",
            "role": "Lead Architect",
            "voice_preference": "Sophisticated",
            "ai_autonomy_level": 0.85 # 0.0 to 1.0 proactivity
        }

        # Current mode – can be overridden via config file or UI
        self.ACTIVE_MODE = "default"


        # Security defaults
        self.SECURITY = {
            "level": "ELITE (Level 5)",
            "encryption": "AES-GCM-256 (Quantum-Safe)",
        }

        # Shortcuts dictionary – can be overridden via sigma_config.json
        self.SHORTCUTS = {
            "global": {
                "open_terminal": "Ctrl+Alt+T",
                "open_automator": "Ctrl+Alt+S",
                "open_ai_nexus": "Ctrl+Alt+N",
                "check_updates": "Ctrl+Alt+U",
                "lock_screen": "Ctrl+Alt+L",
                "quit": "Ctrl+Alt+Q"
            }
        }

        # Developer mode flag – enables debug console and extra logs
        self.DEVELOPER_MODE = False

        # Local Only Mode
        self.LOCAL_ONLY_MODE = True # Force local-first by default

        # Ensure dirs exist
        for d in [self.WORKSPACE_DIR, self.ASSETS_DIR, self.LOGS_DIR]:
            d.mkdir(parents=True, exist_ok=True)

        # Apply active mode settings after directories are ready
        self.apply_mode(self.ACTIVE_MODE)

        self._load_persisted()

    def _load_persisted(self):
        if self.CONFIG_FILE.exists():
            try:
                with open(self.CONFIG_FILE) as f:
                    overrides = json.load(f)
                for k, v in overrides.items():
                    setattr(self, k, v)
                # Apply mode if overridden
                if "ACTIVE_MODE" in overrides:
                    self.apply_mode(self.ACTIVE_MODE)
                # Apply shortcuts if overridden
                if "SHORTCUTS" in overrides:
                    self.SHORTCUTS = overrides["SHORTCUTS"]
                # Apply developer flag if overridden
                if "DEVELOPER_MODE" in overrides:
                    self.DEVELOPER_MODE = overrides["DEVELOPER_MODE"]
            except Exception:
                pass

    def apply_mode(self, mode_name: str):
        """Apply a predefined mode, toggling all six optimisation flags and developer flag."""
        if mode_name not in self.MODES:
            if not self.SILENT: print(f"[WARN] Unknown mode '{mode_name}'. Falling back to 'default'.")
            mode_name = "default"
        mode_cfg = self.MODES[mode_name]
        for flag, value in mode_cfg.items():
            setattr(self, flag, value)
        # Ensure developer mode flag is also respected (default false unless explicitly set)
        if hasattr(self, "DEVELOPER_MODE"):
            # keep existing value unless mode explicitly defines it
            pass
        self.ACTIVE_MODE = mode_name
        if not self.SILENT: print(f"[INFO] SigmaConfig: Mode '{mode_name}' applied.")

    def apply_vibe(self, vibe_name: str):
        """USP: Shifts the visual and behavioral DNA of the OS."""
        if vibe_name in self.VIBES:
            self.ACTIVE_VIBE = vibe_name
            # Map vibe to mode
            if vibe_name == "Minimalist": self.apply_mode("tuned")
            elif vibe_name == "Gamer": self.apply_mode("apex")
            else: self.apply_mode("default")
            if not self.SILENT: print(f"[INFO] SigmaConfig: Vibe '{vibe_name}' active.")

    def save(self):
        data = {
            "OS_NAME": self.OS_NAME,
            "VERSION": self.VERSION,
        }
        with open(self.CONFIG_FILE, "w") as f:
            json.dump(data, f, indent=2)

    def as_dict(self):
        return {
            "os_name": self.OS_NAME,
            "version": self.VERSION,
            "build": self.BUILD,
            "base_kernel": self.BASE_KERNEL,
            "base_dir": str(self.BASE_DIR),
            "active_mode": self.ACTIVE_MODE,
            "active_vibe": self.ACTIVE_VIBE,
            "persona": self.USER_PERSONA,
            "features": {flag: getattr(self, flag, False) for flag in self.MODES["apex"].keys() if flag != "ACTIVE_MODE"}
        }

    def is_feature_enabled(self, feature_name: str) -> bool:
        """USP: Centralized feature-flag gating for kernel/app logic."""
        return getattr(self, feature_name, False)
