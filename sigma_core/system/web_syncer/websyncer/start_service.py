# Generated method: WebSyncer.start_service
import os
import time
import requests
from .interfaces import SigmaModuleBase

class WebSyncer:
    def start_service(self):
        self.sync_sites()
        return 'OK'