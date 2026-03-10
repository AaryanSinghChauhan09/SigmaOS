"""
SigmaOS Configuration Management Module
Handles OS-level configuration, constants, and settings.
"""

class SigmaConfig:
    """
    Central configuration for SigmaOS.
    Provides OS metadata, paths, and global settings.
    """
    
    # OS Metadata
    OS_NAME = "SigmaOS Sovereign"
    VERSION = "2.0.0 (Apex)"
    BUILD = "2026.03.10-APEX-ULTRA"
    BASE_KERNEL = "Modular Event-Driven Kernel"
    RELEASE_DATE = "2026-03-10"
    
    # Feature Flags
    ENABLE_GUI = True
    ENABLE_CLI = True
    ENABLE_AGENTIC = True
    ENABLE_MESH = True
    SILENT = False
    
    # Paths
    ROOT_PATH = "."
    KERNEL_PATH = "./kernel"
    ECOSYSTEM_PATH = "./ecosystem"
    USERLAND_PATH = "./userland"
    WORKSPACE_DIR = "."
    
    # Module Configuration
    AUTO_REGISTER_MODULES = True
    ENABLE_ANOMALY_DETECTION = True
    ENABLE_PREDICTIVE_SCHEDULING = True
    
    # Performance
    ZRAM_ENABLED = True
    HIGH_PERFORMANCE_IO = True
    ADAPTIVE_ENERGY = True
    
    # Security
    ZERO_TRUST_MODE = True
    SECURITY_LEVEL = "QUANTUM_SAFE"
    ENABLE_EBPF_MONITORING = True
    
    # --- [SECURE IDENTITY VAULT] ---
    # These should be changed by the user via the Global Config Hub (Ctrl+,)
    DEFAULT_USER_EMAIL = "aaryan@gmail.com"
    DEFAULT_VAULT_SECRET = "sovereign-ultra-secret-42"
    DEFAULT_OAUTH_TOKEN = "SIGMA_SECURE_TOKEN_2026"
    
    # VIBES & Themes
    VIBES = {
        "Minimalist": {"accent": "#5AC8FA", "focus": "Zen"},
        "Cyberpunk":  {"accent": "#FF2D55", "focus": "Hacking"},
        "Enterprise": {"accent": "#007AFF", "focus": "Work"},
        "Gamer":      {"accent": "#AF52DE", "focus": "Performance"}
    }
    
    def __init__(self):
        """Initialize configuration instance"""
        self._custom_settings = {}
    
    def get(self, key, default=None):
        """Get a configuration value"""
        if key in self._custom_settings:
            return self._custom_settings[key]
        return getattr(self, key, default)
    
    def set(self, key, value):
        """Set a custom configuration value"""
        self._custom_settings[key] = value
    
    def apply_vibe(self, vibe):
        """
        Declarative State Management (Automation Principle).
        Transforms the OS 'DNA' based on the requested persona.
        """
        if vibe not in self.VIBES: return
        
        v_data = self.VIBES[vibe]
        self.set("CURRENT_VIBE", vibe)
        
        # Policy shifts based on vibe
        if vibe == "Minimalist":
            self.ZRAM_ENABLED = True
            self.ADAPTIVE_ENERGY = True
            self.ENABLE_EBPF_MONITORING = False # Stealth
        elif vibe == "Gamer":
            self.ZRAM_ENABLED = False # Fast raw RAM
            self.ADAPTIVE_ENERGY = False # Performance mode
            self.SECURITY_LEVEL = "Standard"
        elif vibe == "Enterprise":
            self.SECURITY_LEVEL = "QUANTUM_SAFE"
            self.ZERO_TRUST_MODE = True
            
        print(f"[DNA] System State re-calculated for {vibe}. Applied policy shifts.")
        return v_data

    def to_dict(self):
        """Export configuration as dictionary"""
        return {
            "os_name": self.OS_NAME,
            "version": self.VERSION,
            "build": self.BUILD,
            "features": {
                "gui": self.ENABLE_GUI,
                "cli": self.ENABLE_CLI,
                "agentic": self.ENABLE_AGENTIC,
                "mesh": self.ENABLE_MESH
            },
            "security": {
                "zero_trust": self.ZERO_TRUST_MODE,
                "level": self.SECURITY_LEVEL,
                "ebpf": self.ENABLE_EBPF_MONITORING
            }
        }
