# Generated method: SigmaMLEngine.evaluate_performance
import time
import random
from typing import List, Dict, Any, Tuple

class SigmaMLEngine:
    def evaluate_performance(self, y_true: List[int], y_pred: List[int]) -> Dict[str, float]:
        """Calculates Precision, Recall, and F1 for data scientists."""
        tp = sum((1 for t, p in zip(y_true, y_pred) if t == 1 and p == 1))
        fp = sum((1 for t, p in zip(y_true, y_pred) if t == 0 and p == 1))
        fn = sum((1 for t, p in zip(y_true, y_pred) if t == 1 and p == 0))
        tn = sum((1 for t, p in zip(y_true, y_pred) if t == 0 and p == 0))
        precision = tp / (tp + fp) if tp + fp > 0 else 0.0
        recall = tp / (tp + fn) if tp + fn > 0 else 0.0
        f1 = 2 * (precision * recall) / (precision + recall) if precision + recall > 0 else 0.0
        return {'precision': float(int(precision * 10000)) / 10000.0, 'recall': float(int(recall * 10000)) / 10000.0, 'f1_score': float(int(f1 * 10000)) / 10000.0, 'accuracy': float(int((tp + tn) / len(y_true) * 10000)) / 10000.0 if len(y_true) > 0 else 0.0}