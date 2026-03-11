"""
SigmaOS Aethereal Shell (v1.0 Apex)
===================================
USP: The primary HMI (Human-Machine Interface) for SigmaOS.
Supports: Swarm Orchestration, Kernel Syscalls, and SigmaFS Temporal Rewind.
"""

import sys
import os
import time
from typing import List, Optional, Any
try:
    from .interfaces import SigmaModuleBase, ISigmaService
except ImportError:
    import sys
    import os
    _ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    if _ROOT not in sys.path:
        sys.path.insert(0, _ROOT)
    from sigma_core.interfaces import SigmaModuleBase, ISigmaService

class SovereignShell(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.prompt = "Σos> "
        self.history: List[str] = []
        self._running = False

    def start_service(self):
        self._running = True
        self.log_event("shell_start", {"msg": "Aethereal Interface Online"})
        return "Sovereign Shell: Interface Ready."

    def stop_service(self):
        self._running = False

    def execute_command(self, cmd_line: str) -> str:
        """USP: Kernel-Bus Command Dispatcher."""
        parts = cmd_line.strip().split()
        if not parts: return ""
        
        main_cmd = parts[0].lower()
        args = parts[1:] if len(parts) > 1 else []
        
        self.history.append(cmd_line)
        self.log_event("shell_command", {"cmd": main_cmd, "args": args})

        # --- Command Routing ---
        if main_cmd == "system":
            return self._handle_system(args)
        elif main_cmd == "swarm":
            return self._handle_swarm(args)
        elif main_cmd == "fs":
            return self._handle_fs(args)
        elif main_cmd == "turbo":
            if self.kernel: return self.kernel.apply_turbo_mode()
            return "Kernel not attached."
        elif main_cmd == "vibe":
            return self._handle_vibe(args)
        elif main_cmd == "auto":
            return self._handle_auto(args)
        elif main_cmd == "clear":
            return "\033[2J\033[H" # ANSI Clear Screen
        elif main_cmd in ["help", "?"]:
            return self._get_help()
        else:
            return f"Error: Command '{main_cmd}' unknown to the Aethereal Bus."

    def _handle_system(self, args: List[str]) -> str:
        if not self.kernel: return "Error: Kernel Offline."
        if not args or "health" in args:
            return str(self.kernel.health_check())
        if "telemetry" in args:
            hal = self.kernel.registry.get("hal")
            return str(hal.get_hardware_state()) if hal else "HAL missing."
        return "Usage: system [health|telemetry]"

    def _handle_swarm(self, args: List[str]) -> str:
        orch = self.kernel.registry.get("agent_orchestrator")
        if not orch: return "Orchestrator Offline."
        if not args: return "Usage: swarm [deploy|list|consensus] <args>"
        
        sub = args[0].lower()
        if sub == "deploy":
            roles = args[1:] if len(args) > 1 else ["Generalist"]
            sid = orch.deploy_swarm("User Mission", roles)
            return f"Swarm: Deployed {sid} with roles: {roles}"
        elif sub == "list":
            return f"Active Swarms: {list(orch.active_swarms.keys())}"
        
        return f"Swarm: Executing mission {args}..."

    def _handle_fs(self, args: List[str]) -> str:
        fs = self.kernel.registry.get("sigma_fs") or self.kernel.registry.get("silos")
        if not fs: return "SigmaFS Offline."
        if "snapshot" in args:
            if hasattr(fs, "create_snapshot"):
                res = fs.create_snapshot("Manual-Shell-Snap")
                return f"FS: {res['message']}"
            return "FS Module does not support snapshots."
        if "rewind" in args:
            if hasattr(fs, "temporal_rewind"):
                res = fs.temporal_rewind(60)
                return f"FS: {res['message']}"
            return "FS Module does not support temporal rewind."
        return "Usage: fs [snapshot|rewind|list]"

    def _handle_vibe(self, args: List[str]) -> str:
        if not self.kernel: return "Kernel Required."
        cust = self.kernel.registry.get("customizer")
        if not cust: return "Customizer Offline."
        if not args: return "Usage: vibe [Glass|Classic|Aura|Brutalist]"
        res = cust.apply_morphic_preset(args[0])
        return f"VIBE: Applied '{args[0]}' Morph. Output: {res['status']}"

    def _handle_auto(self, args: List[str]) -> str:
        if not self.kernel: return "Kernel Required."
        auto = self.kernel.registry.get("automator")
        if not auto: return "OmniAutomator Offline."
        if not args: return f"Auto Status: {auto.health_check()}"
        sub = args[0].lower()
        if sub == "start":
            auto.start_sentinel()
            return "SENTINEL: Proactive Intelligence Loop STARTED."
        if sub == "stop":
            auto.stop_sentinel()
            return "SENTINEL: Proactive Intelligence Loop STOPPED."
        if sub == "mission":
            intent = " ".join(args[1:]) if len(args) > 1 else "Optimize System"
            return auto.launch_mission(intent)
        return "Usage: auto [start|stop|mission <intent>]"

    def _get_help(self) -> str:
        return """
Sovereign Shell Help
--------------------
system [cmd]       - Hardware & Kernel telemetry
swarm [cmd] [args] - Deploy or manage AI swarms
fs [cmd] [args]    - SigmaFS temporal operations
vibe [preset]      - Shift Morphological Aesthetic
auto [cmd] [args]  - OmniAutomator / Proactive Sentinel
turbo              - Engage Max Throughput mode
clear              - Clear the terminal screen
help               - This help menu
exit               - Terminate session
        """

    def health_check(self) -> str:
        return f"OK — Shell: {len(self.history)} commands processed."
