"""omni_automator.get_preview_card — Transparent execution preview."""
from typing import Dict, Any
from userland.system_api.omni_automator.constants import PRESETS


def get_preview_card(preset_key: str) -> Dict[str, Any]:
    """USP: Transparent Execution Log Previews before committing to ring-0 hardware routines."""
    p = PRESETS.get(preset_key)
    if not p:
        return {"Error": "Preset Not Found"}
    return {
        "Card_Title": f"🔍 Preview: {p['name']}",
        "Expected_Resource_Shift": f"CPU/GPU will pivot to '{p.get('tuning', 'Balanced')}' mode.",
        "Execution_DAG": p.get("actions", []),
        "Impact_Rating": "High (Kernel Modifications)" if "tuning" in p else "Low (Userland Only)",
        "Trust_Level": "VERIFIED_0xAPEX",
    }
