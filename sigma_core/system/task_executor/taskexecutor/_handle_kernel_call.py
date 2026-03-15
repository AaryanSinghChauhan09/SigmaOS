# Generated method: TaskExecutor._handle_kernel_call
import os
import subprocess
import time
from typing import Dict, Any, List, Optional

class TaskExecutor:
    def _handle_kernel_call(self, params: Dict[str, Any]) -> Dict[str, Any]:
        module_id = params.get('module')
        method_name = params.get('method')
        args = params.get('args', [])
        if not isinstance(module_id, str) or not isinstance(method_name, str):
            return {'status': 'ERROR', 'output': 'Module ID and Method Name must be strings'}
        module = self.kernel.registry.get(module_id)
        if module and hasattr(module, method_name):
            method = getattr(module, method_name)
            output = method(*args)
            return {'status': 'SUCCESS', 'output': output}
        return {'status': 'ERROR', 'output': f'Module or method not found: {module_id}.{method_name}'}