# Generated method: ExplorerPage._mount_silo
import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage:
    def _mount_silo(self, path):
        if not os.path.isdir(path):
            return self.gui._notify('Silo Error', 'Can only silo directories.', 'ERR')
        sid = self.kernel.silo_fs.create_silo(os.path.basename(path), path)
        self.gui._notify('Silo Active', f'Mounted as {sid}', 'OK')
        self._refresh_silos()