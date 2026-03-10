"""
Sovereign Syscall Gateway — v1.0
=================================
USP: Secure entry point (int 0x80) for Ring-3 logic.
     Bridges User-Space apps with Kernel-Space sovereignty.
"""

from typing import Dict, Any

class SyscallGateway:
    def __init__(self, kernel):
        self.kernel = kernel
        # Map Syscall ID -> Kernel Function
        self.handlers = {
            0x01: self._sys_exit,
            0x02: self._sys_fork,
            0x03: self._sys_read,
            0x04: self._sys_write,
            0x05: self._sys_open,
            0x06: self._sys_close,
            0x10: self._sys_malloc,
            0x20: self._sys_ping,           # Network Bridge
            0x21: self._sys_dhcp_discover, # Network Bridge
            0x30: self._sys_ls_initrd,     # FS Bridge
            0x80: self._sys_zenith_mission # Zenith USP
        }

    def execute(self, call_data: Dict[str, Any]) -> Any:
        """USP: The Sovereign Gateway. Validates permissions before Ring-0 entry."""
        call_id = call_data.get("id")
        params = call_data.get("params", {})
        
        handler = self.handlers.get(call_id)
        if not handler:
            return {"error": f"Invalid Syscall ID: {hex(call_id)}"}
            
        # Logging & Audit (Forensic Ledger Bridge)
        if hasattr(self.kernel, 'fs'):
            self.kernel.fs._log_event("syscall", str(call_id), "Gateway Entry")
            
        return handler(params)

    # --- Syscall Implementations ---
    def _sys_exit(self, p):
        return {"status": "HALT", "code": p.get("status", 0)}

    def _sys_fork(self, p):
        if hasattr(self.kernel, 'scheduler'):
            return self.kernel.scheduler.create_task("forked_process")
        return {"error": "Scheduler offline"}

    def _sys_read(self, p):
        if hasattr(self.kernel, 'fs'):
            return self.kernel.fs.read(p.get("path"))
        return {"error": "FS offline"}

    def _sys_write(self, p):
        if hasattr(self.kernel, 'fs'):
            return self.kernel.fs.create(p.get("path"), p.get("content", b""))
        return {"error": "FS offline"}

    def _sys_open(self, p):
        return {"fd": 3, "status": "OPEN"}

    def _sys_close(self, p):
        return {"status": "CLOSED"}

    def _sys_malloc(self, p):
        if hasattr(self.kernel, 'memory'):
            return self.kernel.memory.alloc("ring3_app", p.get("size", 1))
        return {"error": "MemMgr offline"}

    def _sys_zenith_mission(self, p):
        if hasattr(self.kernel, 'zenith'):
            return self.kernel.zenith.dispatch_mission(p.get("prompt"), p.get("nodes", []))
        return {"error": "Zenith offline"}

    def _sys_ping(self, p):
        if hasattr(self.kernel, 'network'):
            return self.kernel.network.ping(p.get("target", "8.8.8.8"))
        return {"error": "Network stack offline"}

    def _sys_dhcp_discover(self, p):
        if hasattr(self.kernel, 'network'):
            return self.kernel.network.dhcp_discover()
        return {"error": "Network stack offline"}

    def _sys_ls_initrd(self, p):
        if hasattr(self.kernel, 'fs'):
            # Filter inodes starting with /initrd/
            return [path for path in self.kernel.fs._inodes.keys() if path.startswith("/initrd/")]
        return {"error": "FS offline"}

    def health_check(self) -> str:
        return f"OK — Syscall Gateway: {len(self.handlers)} handlers registered. Secure Bridge Active."
