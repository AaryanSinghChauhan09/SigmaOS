# Generated method: SovereignLisp.alloc
from dataclasses import dataclass
from typing import List, Any

class SovereignLisp:
    def alloc(self, obj_type: str, value: Any) -> LispObject:
        """USP: Heap Allocation with GC Trigger."""
        if len(self.heap) >= self.max_heap:
            self.collect_garbage()
        obj = LispObject(obj_type, value)
        self.heap.append(obj)
        return obj