# Generated method: SovereignCompositor.__init__
from dataclasses import dataclass, field
from typing import List

class SovereignCompositor:
    def __init__(self, kernel):
        self.kernel = kernel
        self.screen_width = 1024
        self.screen_height = 768
        self.windows: List[Window] = []
        self.back_buffer = []
        self.dirty_rects = []