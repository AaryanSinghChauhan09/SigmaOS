"""
Sovereign Serial UART Driver — v1.0
====================================
USP: Headless Console Bridge (COM1 @ 0x3F8).
     Enables real-time external monitoring and Antigravity log-sync.
"""

import time

class SovereignSerial:
    def __init__(self, kernel):
        self.kernel = kernel
        self.base_port = 0x3F8
        self.log_buffer = []
        self.is_initialized = False
        self.baud_rate = 38400
        
        self.init_port()

    def init_port(self):
        """USP: Standard-Grade UART Initialization (8N1)."""
        # Simulated Port I/O
        self.is_initialized = True
        self.write_string("Sovereign-Core: Serial Console (COM1) Active. 38400 8N1 Ready.\n")

    def write_char(self, char: str):
        if not self.is_initialized: return
        self.log_buffer.append(char)
        # In a real kernel, this writes to 0x3F8
        # print(f"[COM1] {char}", end="", flush=True)

    def write_string(self, string: str):
        # Privacy Scrubbing BEFORE output to physical port/external monitor
        scrubber = self.kernel.registry.get("privacy")
        if scrubber:
            string = scrubber.scrub(string)
            
        for char in string:
            self.write_char(char)

    def get_serial_logs(self) -> str:
        return "".join(self.log_buffer[-1000:]) # Return last 1000 chars

    def health_check(self) -> str:
        return f"OK — Serial (COM1): {len(self.log_buffer)} chars piped. Headless Bridge ACTIVE."
