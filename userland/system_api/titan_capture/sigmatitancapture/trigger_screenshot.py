# Generated method: SigmaTitanCapture.trigger_screenshot
import os
import sys
import time
import platform
import ctypes
from ctypes import wintypes
from typing import Dict, List, Any
from sigma_core.hal.graphics_driver import GDI32Driver, BMPEncoder

class SigmaTitanCapture:
    def trigger_screenshot(self, filename: str='sigma_capture.bmp'):
        if platform.system() != 'Windows' or not GDI32Driver:
            return 'ERR: TitanCapture requires Win32 GDI32 Driver.'
        try:
            u32, g32, hdesktopdc, hdesktop, width, height = GDI32Driver.get_desktop_handles()
            hcapturedc, hbitmap = GDI32Driver.create_capture_bitmap(g32, hdesktopdc, width, height)
            g32.BitBlt(hcapturedc, 0, 0, width, height, hdesktopdc, 0, 0, 13369376)
            total_size = (width * 24 + 31) // 32 * 4 * height
            pixels = ctypes.create_string_buffer(total_size)
            import struct
            bi = struct.pack('<IiiHHIIiiII', 40, width, height, 1, 24, 0, total_size, 0, 0, 0, 0)
            g32.GetDIBits(hdesktopdc, hbitmap, 0, height, pixels, bi, 0)
            bmp_data = BMPEncoder.construct_bmp(width, height, pixels.raw)
            path = os.path.join(os.getcwd(), filename)
            with open(path, 'wb') as f:
                f.write(bmp_data)
            self.stats['captures'] += 1
            g32.DeleteObject(hbitmap)
            g32.DeleteDC(hcapturedc)
            u32.ReleaseDC(hdesktop, hdesktopdc)
            return f'Captured: {filename} ({width}x{height})'
        except Exception as e:
            return f'Capture Failed: {str(e)}'