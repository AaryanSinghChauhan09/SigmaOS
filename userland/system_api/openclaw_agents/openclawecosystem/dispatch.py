# Generated method: OpenClawEcosystem.dispatch
from sigma_core.system.interfaces import ISigmaModule, SigmaModuleBase

class OpenClawEcosystem:
    def dispatch(self, agent_id: str, task: str):
        if agent_id in self.agents:
            return self.agents[agent_id].execute_task(task)
        return 'Agent not found.'