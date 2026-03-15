"""sigma_forge.forge_agent — Agent scaffold generator."""
import os
from sigma_forge.forge_app import _write


def forge_agent(name: str, output_dir: str = "userland/apps") -> str:
    """Generate a new SigmaOS agent scaffold."""
    class_name = "".join(x.capitalize() for x in name.replace("_", " ").split())
    content = f'''"""
{name} Agent for SigmaOS
"""
from sigma_core.agent_orchestrator import SigmaAgentIsolate

class {class_name}(SigmaAgentIsolate):
    def __init__(self, agent_id, role="{name}", persona="Advanced", goal="Optimize", kernel=None):
        super().__init__(agent_id, role, persona, goal, kernel)

    def execute_mission(self, context):
        print(f"[AGENT:{{self.role}}] Handling mission: {{context}}")
        return f"Mission sequence for {{context}} finalized."
'''
    return _write(name, output_dir, content, "agent")
