"""
SigmaOS Sovereign AI Agent v1.0
================================
USP: Full-System Agentic Assistance.
An autonomous agent that coordinates kernel modules, performs complex 
task decomposition, and assists the user in achieving OS-level objectives.
"""
import time
from typing import Dict, List, Any, Optional

class SovereignAgent:
    def __init__(self, kernel):
        self.kernel = kernel
        self.active_missions: List[Dict[str, Any]] = []
        self.agent_id = "SIGMA-ALPHA-1"
        self.executor: Any = None # Type hint for linter

    def help_complete_task(self, task_description: str) -> str:
        """High-level entry point for helping the user with ANY task."""
        if not self.executor:
            from ..system.task_executor import TaskExecutor
            self.executor = TaskExecutor(self.kernel)

        print(f"[AGENT] {self.agent_id} analyzing mission: '{task_description}'")
        
        # 1. Consult the Automation Brain for intent classification
        brain = self.kernel.registry.get("automation_brain")
        if not brain:
            return "ERROR: Automation Brain Offline."
            
        intent_res = brain.process_intent(task_description)
        
        # 2. Decompose into mission plan
        mission = {
            "id": f"M-{int(time.time())}",
            "description": task_description,
            "category": intent_res.get("category"),
            "steps": self._generate_steps(intent_res),
            "status": "EXECUTING"
        }
        self.active_missions.append(mission)
        
        # 3. Coordinate execution
        self._coordinate_execution(mission)
        
        # 4. Automate any task (Generic automation pattern)
        if "automate" in task_description.lower():
            self._handle_generic_automation(task_description)
        
        return f"Mission {mission['id']} initiated/automated. Status: {mission['status']}"

    def _generate_steps(self, intent_res: Dict[str, Any]) -> List[str]:
        # Original logic for _generate_steps
        modules = intent_res.get("modules", [])
        steps = []
        for mod in modules:
            if not isinstance(mod, str): # Add type check for module names
                print(f"[AGENT WARNING] Non-string module name encountered: {mod}")
                continue
            steps.append(f"Activate and coordinate with {mod} module.")
        steps.append("Validate system stability post-operation.")
        return steps

    def _coordinate_execution(self, mission: Dict[str, Any]):
        """Simulates the coordination of multiple kernel subsystems."""
        print(f"[AGENT] Orchestrating mission {mission['id']}...")
        self.kernel._morphic_island(f"AGENT: Executing {mission['category']} mission", "#00FFFF") # Cyan
        
        # Systematic execution via TaskExecutor
        for step in mission["steps"]:
            # Mock execution of steps via executor
            self.executor.execute_action("kernel_call", {"module": "troubleshooter", "method": "run_analysis"})
            time.sleep(0.1)

        mission["status"] = "COMPLETED"
        print(f"[AGENT] Mission {mission['id']} completed successfully.")
        self.kernel._morphic_island(f"AGENT: Mission {mission['id']} Success", "#00FF00") # Green

    def _handle_generic_automation(self, prompt: str):
        """Patterns to automate any generic task provided in the prompt."""
        print(f"[AGENT] Engaging Generic Automation engine for: {prompt}")
        # Resolve 'create' or 'delete' patterns
        if "create" in prompt.lower() or "new" in prompt.lower():
            self.executor.execute_action("file_op", {"op": "touch", "path": "automated_asset.txt"})
        elif "clean" in prompt.lower() or "remove" in prompt.lower():
             self.executor.execute_action("file_op", {"op": "rm", "path": "automated_asset.txt"})

    def get_agent_telemetry(self) -> Dict[str, Any]:
        return {
            "agent_id": self.agent_id,
            "total_missions": len(self.active_missions),
            "current_vibe": "PROACTIVE",
            "intelligence_level": "SOVEREIGN-MAX"
        }

if __name__ == "__main__":
    # Test stub
    class MockKernel:
        def __init__(self): 
            from .automation_brain import AutomationBrain
            self.registry = {"automation_brain": AutomationBrain(self)}
        def _morphic_island(self, m, c): print(f"UI Island: [{c}] {m}")
    
    agent = SovereignAgent(MockKernel())
    res = agent.help_complete_task("Clean system and optimize performance")
    print(res)
