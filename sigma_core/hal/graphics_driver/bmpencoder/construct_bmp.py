# Generated method: BMPEncoder.construct_bmp
import ctypes
from ctypes import wintypes
import struct

class BMPEncoder:
    @staticmethod
    def construct_bmp(width, height, pixels):
        total_size = len(pixels)
        bi = struct.pack('<IiiHHIIiiII', 40, width, height, 1, 24, 0, total_size, 0, 0, 0, 0)
        bfp = struct.pack('<2sIHHI', b'BM', 14 + 40 + total_size, 0, 0, 14 + 40)
        return bfp + bi + pixels