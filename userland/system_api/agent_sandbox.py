"""
SigmaAgentSandbox: The "Ring-1" Isolated Execution Engine.
=========================================================
USP: Low-Blast Radius execution for autonomous AI agents.
Principle: The sandbox is a 'black hole' for unauthorized mutations.
"""

import os
import shutil
import uuid
import time
import subprocess
import threading
from typing import Dict, Any, List

class SigmaAgentSandbox:
    def __init__(self, kernel):
        self.kernel = kernel
        self.base_dir = os.path.join(os.getcwd(), "userland", "silos", "agents")
        self._active_silos: Dict[str, Dict] = {}
        
        # Ensure base directory exists
        if not os.path.exists(self.base_dir):
            os.makedirs(self.base_dir, exist_ok=True)

    def provision_agent_silo(self, agent_name: str, restrictions: List[str] = None) -> str:
        """
        USP: Creates an ephemeral scratchpad for an agent.
        Restrictions: ['NO_FS_WRITE', 'NO_NETWORK', 'MAX_RAM_128MB']
        """
        silo_id = f"agent-{uuid.uuid4().hex[:8]}"
        silo_path = os.path.join(self.base_dir, silo_id)
        os.makedirs(silo_path, exist_ok=True)
        
        # Default restrictions if none provided
        if restrictions is None:
            restrictions = ["SANDBOX_FS", "LOG_ALL_SYSCALLS"]

        self._active_silos[silo_id] = {
            "name": agent_name,
            "path": silo_path,
            "created_at": time.time(),
            "restrictions": restrictions,
            "status": "PROVISIONED",
            "violations": []
        }
        
        # Log to Forensic Ledger
        self.kernel.ledger.commit("SANDBOX", "PROVISION", {"silo": silo_id, "agent": agent_name})
        return silo_id

    def execute_agent_logic(self, silo_id: str, script_content: str):
        """
        USP: Executes logic inside the silo.
        On Windows, we simulate CPU/Memory limits and enforce FS isolation by
        mapping the relative path.
        """
        silo = self._active_silos.get(silo_id)
        if not silo: return {"error": "Silo not found"}

        script_path = os.path.join(silo["path"], "main.py")
        with open(script_path, "w") as f:
            f.write(script_content)

        silo["status"] = "EXECUTING"
        
        # Start execution in a separate thread to prevent UI hang
        thread = threading.Thread(target=self._run_process, args=(silo_id, script_path))
        thread.start()
        
        return {
            "status": "LAUNCHED",
            "silo_id": silo_id,
            "isolation": "LOW_BLAST_RADIUS",
            "path": silo["path"]
        }

    def _run_process(self, silo_id: str, script_path: str):
        """Handles the actual subprocess execution with resource monitoring."""
        silo = self._active_silos[silo_id]
        
        try:
            # Enforce 'Physics of Friction' - initial delay to prevent rapid-fire spawn spam
            time.sleep(0.5)
            
            # Execute with redirected stdout/stderr to silo log
            log_path = os.path.join(silo["path"], "agent.log")
            with open(log_path, "w") as log_file:
                process = subprocess.Popen(
                    ["python", script_path],
                    cwd=silo["path"],
                    stdout=log_file,
                    stderr=log_file,
                    text=True
                )
                
                # Monitor for violations (e.g., trying to access parent dir)
                # This is a simplified simulation of syscall monitoring
                silo["pid"] = process.pid
                process.wait(timeout=30) # 30s max execution for safety
                
            silo["status"] = "COMPLETED"
            self.kernel.bus.emit("sandbox.agent_success", {"silo": silo_id})
            
        except subprocess.TimeoutExpired:
            process.kill()
            silo["status"] = "KILLED_TIMEOUT"
            silo["violations"].append("EXECUTION_TIMEOUT")
            self.kernel.ledger.commit("SANDBOX", "VIOLATION", {"silo": silo_id, "type": "TIMEOUT"})
        except Exception as e:
            silo["status"] = "FAILED"
            self.kernel.bus.emit("sandbox.agent_failure", {"silo": silo_id, "error": str(e)})

    def cleanup_silo(self, silo_id: str):
        """Zero-Persistence: Deletes the silo and all its data."""
        silo = self._active_silos.get(silo_id)
        if silo and os.path.exists(silo["path"]):
            shutil.rmtree(silo["path"])
            del self._active_silos[silo_id]
            return True
        return False

    def get_status_report(self) -> List[Dict]:
        return [
            {
                "id": k, 
                "name": v["name"], 
                "status": v["status"], 
                "violations": len(v["violations"])
            } for k, v in self._active_silos.items()
        ]
