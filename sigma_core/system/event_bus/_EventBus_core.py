# Generated class core: EventBus
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus:
    """
    Central event bus for SigmaOS kernel.
    Enables decoupled communication between modules via events.
    USP: High-Performance Concurrent Event Delivery Engine with Priority Queueing.
    """