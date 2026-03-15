# Generated method: SovereignSerial.get_serial_logs
import time

class SovereignSerial:
    def get_serial_logs(self) -> str:
        return ''.join(self.log_buffer[-1000:])