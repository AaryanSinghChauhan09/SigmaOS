"""
SigmaOS Agent Orchestrator (v3.0 Apex)
=======================================
USP: Multi-agent coordination via Modular Mission Planning and Consensus.
Modular Architecture: Delegating to MissionPlanner and SwarmConsensus.
"""
import uuid
import threading
from typing import Dict, List, Any, Optional
from concurrent.futures import ThreadPoolExecutor
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from .mission_planner import MissionPlanner
from .swarm_consensus import SwarmConsensus

class SigmaAgentIsolate:
    def __init__(self, agent_id: str, role: str, persona: str, goal: str, kernel=None):
        self.agent_id = agent_id
        self.role = role
        self.persona = persona
        self.goal = goal
        self.kernel = kernel
        self.status = "IDLE"
        self.memory: List[str] = []
        self.stats = {"tasks_completed": 0}

    def execute_step(self, context: str) -> str:
        self.status = "WORKING"
        response = f"[{self.role}] Processed: {context}. [Persona: {self.persona}]"
        self.memory.append(f"Task: {context} -> Response: {response[:50]}...")
        self.stats["tasks_completed"] += 1
        self.status = "SUCCESS"
        return response

class SigmaAgentOrchestrator(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.planner = MissionPlanner(kernel)
        self.consensus = SwarmConsensus(kernel)
        self.active_swarms: Dict[str, List[SigmaAgentIsolate]] = {}
        self.stats = {"swarms_deployed": 0, "consensus_reached": 0}

    def start_service(self) -> str:
        self.log_event("service_start", {"id": "AgentOrchestrator"})
        return "Agent Orchestrator v3: Swarm Delegation Active."

    def stop_service(self) -> None:
        self.log_event("service_stop", {"id": "AgentOrchestrator"})

    def deploy_swarm(self, goal: str, roles: List[str]) -> str:
        swarm_id = f"swarm-{uuid.uuid4().hex[:6]}"
        isolates = []
        # Use Planner to refine personas
        for role in roles:
            persona = self.planner.refine_persona(role)
            agent_id = f"agent-{uuid.uuid4().hex[:4]}"
            isolates.append(SigmaAgentIsolate(agent_id, role, persona, goal, self.kernel))
        
        self.active_swarms[swarm_id] = isolates
        self.stats["swarms_deployed"] += 1
        return swarm_id

    def execute_swarm_mission(self, swarm_id: str, goal: str) -> str:
        swarm = self.active_swarms.get(swarm_id)
        if not swarm: return "Error: Swarm not found."
        
        # Decompose goal into tasks
        tasks = self.planner.decompose_goal(goal)
        all_results = []
        
        with ThreadPoolExecutor(max_workers=len(swarm)) as executor:
            for task in tasks:
                 futures = [executor.submit(a.execute_step, task) for a in swarm]
                 results = [f.result() for f in futures]
                 # Reach consensus on each task result
                 winning_consensus = self.consensus.reach_consensus(results)
                 all_results.append(winning_consensus)
        
        self.stats["consensus_reached"] += 1
        return f"Mission Complete. Consensus Sequence: {all_results}"

    def health_check(self) -> str:
        return f"OK — Swarms: {len(self.active_swarms)} | Consensus: {self.stats['consensus_reached']}"
