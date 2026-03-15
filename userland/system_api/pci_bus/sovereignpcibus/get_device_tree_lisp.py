# Generated method: SovereignPCIBus.get_device_tree_lisp
from dataclasses import dataclass, field
from typing import List, Optional

class SovereignPCIBus:
    def get_device_tree_lisp(self):
        """Returns the tree in a Lisp-serialized format."""

        def _serialize(node):
            return {'name': node.name, 'vid': hex(node.vendor_id), 'did': hex(node.device_id), 'status': node.status, 'children': [_serialize(c) for c in node.children]}
        return _serialize(self.root)