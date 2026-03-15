# Generated method: SovereignSerial.write_string
import time

class SovereignSerial:
    def write_string(self, string: str):
        scrubber = self.kernel.registry.get('privacy')
        if scrubber:
            string = scrubber.scrub(string)
        for char in string:
            self.write_char(char)