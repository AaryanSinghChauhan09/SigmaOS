# Generated method: WebSyncer.get_local_path
import os
import time
import requests
from .interfaces import SigmaModuleBase

class WebSyncer:
    def get_local_path(self, site_name):
        return os.path.join(self.mirror_dir, site_name)