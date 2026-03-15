# Generated method: SovereignSerial.__init__
import time

class SovereignSerial:
    def __init__(self, kernel):
        self.kernel = kernel
        self.base_port = 1016
        self.log_buffer = []
        self.is_initialized = False
        self.baud_rate = 38400
        self.init_port()