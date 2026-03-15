# Generated method: GDI32Driver.get_desktop_handles
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
        return (u32, g32, hdesktopdc, hdesktop, width, height)