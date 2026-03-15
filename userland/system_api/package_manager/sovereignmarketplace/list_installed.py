# Generated method: SovereignMarketplace.list_installed
import os
import json
import shutil
import hashlib
import time
from pathlib import Path

class SovereignMarketplace:
    def list_installed(self):
        return self.pkg_mgr.list_installed()