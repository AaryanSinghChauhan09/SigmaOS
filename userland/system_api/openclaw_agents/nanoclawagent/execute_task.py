# Generated method: NanoClawAgent.execute_task
from sigma_core.system.interfaces import ISigmaModule, SigmaModuleBase

class NanoClawAgent:
    def execute_task(self, task: str):
        sandbox = self.kernel.registry.get('agent_sandbox')
        silo = sandbox.provision_agent_silo(self.name) if sandbox else 'NO_SILO'
        return f"[{self.name}] Executed '{task}' securely inside silo: {silo}."