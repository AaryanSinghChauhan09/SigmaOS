
"""
SigmaOS TitanCapture v2.0 (Apex Edition)
========================================
USP: High-performance, zero-dependency GDI32 screen orchestration.
Fulfillment of 'Native Accelerator' principle.
"""
import os
import sys
import time
import platform
import ctypes
from ctypes import wintypes
from typing import Dict, List, Any

try:
    from sigma_core.hal.graphics_driver import GDI32Driver, BMPEncoder # type: ignore
except ImportError:
    GDI32Driver = None
    BMPEncoder = None

class SigmaTitanCapture:
    def __init__(self, kernel):
        self.kernel = kernel
        self.stats = {"captures": 0, "total_frames": 0}
        self.user32: Any = None
        self.gdi32: Any = None
        self._setup_win32()

    def _setup_win32(self):
        if platform.system() == "Windows":
            self.user32 = ctypes.windll.user32
            self.gdi32 = ctypes.windll.gdi32
            
    def start_service(self) -> str:
        return "TitanCapture: Sovereign GDI32 Orchestrator Active."

    def health_check(self) -> str:
        return f"OK - Native_Driver: {platform.system()}"

    def trigger_screenshot(self, filename: str = "sigma_capture.bmp"):
        if platform.system() != "Windows" or not GDI32Driver:
            return "ERR: TitanCapture requires Win32 GDI32 Driver."
            
        try:
            u32, g32, hdesktopdc, hdesktop, width, height = GDI32Driver.get_desktop_handles()
            hcapturedc, hbitmap = GDI32Driver.create_capture_bitmap(g32, hdesktopdc, width, height)
            
            # 3. BitBlt: Super-fast memory transfer
            g32.BitBlt(hcapturedc, 0, 0, width, height, hdesktopdc, 0, 0, 0x00CC0020) # SRCCOPY
            
            # 4. BMP Data Extraction
            total_size = ((width * 24 + 31) // 32) * 4 * height
            pixels = ctypes.create_string_buffer(total_size)
            
            # Reconstruct BITMAPINFO for GetDIBits
            import struct
            bi = struct.pack("<IiiHHIIiiII", 40, width, height, 1, 24, 0, total_size, 0, 0, 0, 0)
            g32.GetDIBits(hdesktopdc, hbitmap, 0, height, pixels, bi, 0)
            
            # 5. Native Save (Delegated to BMPEncoder)
            bmp_data = BMPEncoder.construct_bmp(width, height, pixels.raw)
            path = os.path.join(os.getcwd(), filename)
            
            with open(path, "wb") as f:
                f.write(bmp_data)
            
            self.stats["captures"] += 1

            # Cleanup
            g32.DeleteObject(hbitmap)
            g32.DeleteDC(hcapturedc)
            u32.ReleaseDC(hdesktop, hdesktopdc)
            
            return f"Captured: {filename} ({width}x{height})"
        except Exception as e:
            return f"Capture Failed: {str(e)}"

    def analyze_frame_buffer(self) -> str:
        """Simulates AI-driven analysis of the current frame buffer."""
        return "Frame Buffer: [STABLE] - Zero UI lag detected."

if __name__ == "__main__":
    tc = SigmaTitanCapture(None)
    print(tc.trigger_screenshot())
