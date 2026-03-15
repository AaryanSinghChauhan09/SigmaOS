# Generated method: NCERTLabEngine.run_experiment
import sys
import os
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class NCERTLabEngine:
    def run_experiment(self, subject: str, experiment_id: str, *args):
        shard = {'physics': self._phy, 'chemistry': self._chem, 'biology': self._bio, 'maths': self._math}.get(str(subject).lower())
        if not shard:
            return {'error': f"Subject '{subject}' not found"}
        method = getattr(shard, experiment_id, None)
        if not method:
            return {'error': f"Experiment '{experiment_id}' not found in {subject}"}
        return method(*args)