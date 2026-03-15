# Generated method: ModuleRegistry.health_check
from typing import Dict, Any, Callable, Optional, List
import threading

class ModuleRegistry:
    def health_check(self) -> Dict[str, str]:
        """
            Perform health check on all registered modules
        
            Returns:
                Dictionary mapping module names to health status
            """
        health = {}
        with self._lock:
            for name, module in self._modules.items():
                if hasattr(module, 'health_check'):
                    try:
                        health[name] = module.health_check()
                    except Exception as e:
                        health[name] = f'ERROR: {str(e)}'
                else:
                    health[name] = 'OK'
        return health