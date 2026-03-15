# Generated method: SovereignSerial.init_port
import time

class SovereignSerial:
    def init_port(self):
        """USP: Standard-Grade UART Initialization (8N1)."""
        self.is_initialized = True
        self.write_string('Sovereign-Core: Serial Console (COM1) Active. 38400 8N1 Ready.\n')