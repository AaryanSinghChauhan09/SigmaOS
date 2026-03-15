# Generated class core: MockKernel
import time
from typing import Dict

class MockKernel:

    class ResourceGov:

        def boost_foreground(self, v):
            print(f'FG Boost: {v}')

        def throttle_background(self, v):
            print(f'BG Throttle: {v}')
    resource_governor = ResourceGov()