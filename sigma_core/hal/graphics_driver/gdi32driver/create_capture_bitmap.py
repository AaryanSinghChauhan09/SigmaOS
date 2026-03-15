# Generated method: GDI32Driver.create_capture_bitmap
import ctypes
from ctypes import wintypes
import struct

class GDI32Driver:
    @staticmethod
    def create_capture_bitmap(g32, hdc, width, height):
        hcapturedc = g32.CreateCompatibleDC(hdc)
        hbitmap = g32.CreateCompatibleBitmap(hdc, width, height)
        g32.SelectObject(hcapturedc, hbitmap)
        return (hcapturedc, hbitmap)