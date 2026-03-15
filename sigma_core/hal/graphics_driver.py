"""
SigmaOS Graphics Driver (HAL v3.0 Apex)
=======================================
Direct low-level GDI32/User32 orchestration.
"""
import ctypes
from ctypes import wintypes
import struct

class GDI32Driver:
    @staticmethod
    def get_desktop_handles():
        u32 = ctypes.windll.user32
        g32 = ctypes.windll.gdi32
        hdesktopdc = u32.GetDC(0)
        hdesktop = u32.GetDesktopWindow()
        width = u32.GetSystemMetrics(0)
        height = u32.GetSystemMetrics(1)
        return u32, g32, hdesktopdc, hdesktop, width, height

    @staticmethod
    def create_capture_bitmap(g32, hdc, width, height):
        hcapturedc = g32.CreateCompatibleDC(hdc)
        hbitmap = g32.CreateCompatibleBitmap(hdc, width, height)
        g32.SelectObject(hcapturedc, hbitmap)
        return hcapturedc, hbitmap

class BMPEncoder:
    @staticmethod
    def construct_bmp(width, height, pixels):
        total_size = len(pixels)
        # BITMAPINFOHEADER (40 bytes)
        bi = struct.pack("<IiiHHIIiiII", 40, width, height, 1, 24, 0, total_size, 0, 0, 0, 0)
        # BITMAPFILEHEADER (14 bytes)
        bfp = struct.pack("<2sIHHI", b'BM', 14 + 40 + total_size, 0, 0, 14 + 40)
        return bfp + bi + pixels
