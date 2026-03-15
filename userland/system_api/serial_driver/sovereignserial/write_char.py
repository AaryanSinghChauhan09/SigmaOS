# Generated method: SovereignSerial.write_char
import time

class SovereignSerial:
    def write_char(self, char: str):
        if not self.is_initialized:
            return
        self.log_buffer.append(char)