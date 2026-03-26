"""omni_automator.constants — Mission library, presets, and config."""
from typing import Dict, Any

MISSION_LIBRARY: Dict[str, list] = {
    "Hardening": ["Kill_Legacy_Shims", "Update_Sovereign_Policies", "Seal_Shadow_Vault"],
    "Optimization": ["Flush_VRAM", "Steer_IRQs", "Trigger_Prewarmer"],
    "Sync": ["Mesh_Merkle_Verify", "Push_to_Origin_Master"],
}

PRESETS: Dict[str, Dict[str, Any]] = {
    "Gaming_Apex": {
        "name": "🎮 Gaming Apex Mode",
        "tuning": "Gaming",
        "actions": ["Hyper_Drive_Engage", "Starve_Background", "Apply_Aura:CyberPunk"],
        "description": "Unlocks maximum silicon potential for zero-latency gameplay.",
    },
    "Nightly_Purge": {
        "name": "🧹 Nightly System Purge",
        "actions": ["Flush_VRAM", "Mesh_Sync_Critical", "Scrub_Temp_Files", "Apply_Aura:DeepSpace"],
        "description": "Optimizes storage and security while the user rests.",
    },
    "Deep_Focus": {
        "name": "🧠 Deep Focus Protocol",
        "tuning": "Efficiency",
        "actions": ["Mute_Notifications", "Block_Distractions", "Apply_Aura:Monolith", "Starve_Background"],
        "description": "Engages zero-interruption hyper-focus state.",
    },
    "Creative_Flow": {
        "name": "🎨 Creative Flow State",
        "tuning": "Performance",
        "actions": ["Boost_GPU_Priority", "Enable_Spatial_Audio", "Apply_Aura:Fluency"],
        "description": "Allocates maximum media/render power and fluid aesthetics.",
    },
}
