# Generated method: SovereignSearch.stop_service
from typing import List, Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SovereignSearch:
    def stop_service(self):
        self.log_event('service_stop', {'id': 'SovereignSearch'})