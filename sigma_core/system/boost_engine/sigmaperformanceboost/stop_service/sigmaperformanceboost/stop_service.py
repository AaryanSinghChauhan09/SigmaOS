# Generated method: SigmaPerformanceBoost.stop_service
from __future__ import annotations
import os
import sys
import gc
import time
import ctypes
import platform
import threading
import subprocess
import shutil
from typing import Dict, Any, List, Callable, Optional
from concurrent.futures import ThreadPoolExecutor, as_completed

class SigmaPerformanceBoost:
    def stop_service(self) -> None:
        self.log_event('service_stop', {'id': 'TurboBoost'})