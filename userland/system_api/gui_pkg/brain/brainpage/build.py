# Generated method: BrainPage.build
import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class BrainPage:
    def build(self):
        self.controller._build_page_header(self, 'Cognitive Fabric', 'Intent-Aware OS Synchronization')
        self.brain_log = self.controller._console(self, height=15)
        self.brain_log.pack(fill='both', expand=True, pady=10)

        def sync():
            if hasattr(self.controller.kernel, 'brain'):
                res = self.controller.kernel.brain.synchronize_intent()
                self.controller._log(self.brain_log, res, 'OK')
                rec = self.controller.kernel.brain.predict_next_command()
                self.controller._log(self.brain_log, f'Recommendation: {rec}', 'INFO')
            else:
                self.controller._log(self.brain_log, 'Brain module not found in kernel.', 'ERR')
        tk.Button(self, text='⚡ Synchronize Intent', bg=PAL['purple'], fg='white', command=sync).pack(pady=10)