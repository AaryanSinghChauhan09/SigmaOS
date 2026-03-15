# Generated method: SigmaDataViz.data_profile
import math
import time
from typing import List, Dict, Any, Optional

class SigmaDataViz:
    def data_profile(self, dataset: List[float], name: str='Dataset') -> Dict[str, Any]:
        """Performs statistical profiling for data analysts."""
        if not dataset:
            return {'error': 'Empty dataset'}
        dataset.sort()
        n = len(dataset)
        mean = sum(dataset) / n
        median = dataset[n // 2] if n % 2 != 0 else (dataset[n // 2 - 1] + dataset[n // 2]) / 2
        variance = sum(((x - mean) ** 2 for x in dataset)) / n
        std_dev = math.sqrt(variance)
        profile = {'name': name, 'count': n, 'min': float(dataset[0]), 'max': float(dataset[-1]), 'mean': float(int(mean * 100)) / 100.0, 'median': float(int(median * 100)) / 100.0, 'std_dev': float(int(std_dev * 100)) / 100.0, 'range': float(int((dataset[-1] - dataset[0]) * 100)) / 100.0}
        return profile