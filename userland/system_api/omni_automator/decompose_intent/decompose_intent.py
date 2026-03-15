# Generated file: decompose_intent
from typing import List
from userland.system_api.omni_automator.mission_node import MissionNode

def decompose_intent(intent: str) -> List[MissionNode]:
    """Decomposes a natural-language intent string into a MissionNode DAG."""
    nodes = []
    low_intent = intent.lower()
    nodes.append(MissionNode('n0', 'Ingest_Context', 'action', {'intent': intent}))
    if 'security' in low_intent or 'harden' in low_intent:
        nodes.extend([MissionNode('n1', 'Seal_Vaults', 'action'), MissionNode('n2', 'Audit_Syscalls', 'decision')])
        nodes[0].next_node_id = 'n1'
        nodes[1].next_node_id = 'n2'
    else:
        nodes.append(MissionNode('n1', 'Autonomous_Execution', 'action'))
        nodes[0].next_node_id = 'n1'
    return nodes