"""omni_automator.launch_mission — Mission launcher."""
import uuid
from typing import List, Dict
from userland.system_api.omni_automator.mission_node import MissionNode
from userland.system_api.omni_automator.decompose_intent import decompose_intent


def launch_mission(
    intent: str,
    active_missions: Dict[str, List[MissionNode]],
    stats: dict,
) -> str:
    """Launches a new autonomous mission from a natural-language intent."""
    uid_str = uuid.uuid4().hex
    mid = f"mission-{uid_str[:8]}"
    active_missions[mid] = decompose_intent(intent)
    stats["workflows_executed"] += 1
    return f"OmniAutomator Apex: Mission '{mid}' launched for intent: '{intent}'."
