"""
SigmaOS Multimedia Subsystem (Modular Shard)
Handles GPU-accelerated codecs and media playback.
"""
from sigmaos.kernel.subsystem import Subsystem

class MultimediaSubsystem(Subsystem):
    def __init__(self):
        super().__init__("Multimedia")
        self.codecs = ["AV1", "HEVC", "Opus"]

    def load_codec(self, codec: str):
        if codec in self.codecs:
            print(f"[Media] Loading GPU-accelerated codec: {codec}")
        else:
            print(f"[Media] Codec {codec} not found.")
