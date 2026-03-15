# Generated class core: EventBus
from typing import Dict, Callable, List, Any, Tuple
import threading
import time
import queue
from concurrent.futures import ThreadPoolExecutor

class EventBus: