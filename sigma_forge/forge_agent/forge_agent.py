# Generated file: forge_agent
import os
from sigma_forge.forge_app import _write

def forge_agent(name: str, output_dir: str='userland/apps') -> str:
    """Generate a new SigmaOS agent scaffold."""
    class_name = ''.join((x.capitalize() for x in name.replace('_', ' ').split()))
    content = f'"""\n{name} Agent for SigmaOS\n"""\nfrom sigma_core.agent_orchestrator import SigmaAgentIsolate\n\nclass {class_name}(SigmaAgentIsolate):\n    def __init__(self, agent_id, role="{name}", persona="Advanced", goal="Optimize", kernel=None):\n        super().__init__(agent_id, role, persona, goal, kernel)\n\n    def execute_mission(self, context):\n        print(f"[AGENT:{{self.role}}] Handling mission: {{context}}")\n        return f"Mission sequence for {{context}} finalized."\n'
    return _write(name, output_dir, content, 'agent')