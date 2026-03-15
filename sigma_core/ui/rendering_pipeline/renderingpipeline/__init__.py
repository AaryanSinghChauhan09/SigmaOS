# Generated method: RenderingPipeline.__init__
import time
from typing import Dict, Any, List

class RenderingPipeline:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.frame_buffer = []
        self.v_sync = True
        self.refresh_rate = 144
        self.last_frame_ts = time.time()