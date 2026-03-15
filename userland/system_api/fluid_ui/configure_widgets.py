"""fluid_ui.configure_widgets — Live telemetry widget manager."""
from typing import Dict, Any, List, Optional


def configure_widgets(state: dict, add: Optional[List[str]] = None, remove: Optional[List[str]] = None) -> Dict[str, Any]:
    """Personalization: Dynamically anchor or remove live telemetry widgets."""
    if add:
        state["active_widgets"].extend(add)
    if remove:
        state["active_widgets"] = [w for w in state["active_widgets"] if w not in remove]
    state["active_widgets"] = list(dict.fromkeys(state["active_widgets"]))
    return {
        "status": "Fluid Layout Adjusted",
        "active_widgets": state["active_widgets"],
        "message": f"UI Nexus: Now anchoring {len(state['active_widgets'])} live widgets to the desktop plane.",
    }
