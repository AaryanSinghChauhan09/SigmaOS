# Generated method: NeuralProcessPredictor.__init__
import collections

class NeuralProcessPredictor:
    def __init__(self):
        self.history = collections.deque(maxlen=100)
        self.weights = collections.defaultdict(int)