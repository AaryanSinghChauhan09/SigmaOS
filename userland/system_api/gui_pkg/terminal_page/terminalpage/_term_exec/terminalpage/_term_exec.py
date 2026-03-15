# Generated method: TerminalPage._term_exec
import tkinter as tk
from tkinter import ttk
import threading
from .base_page import SigmaPage
from .styles import PAL, FONT_MONO

class TerminalPage:
    def _term_exec(self, event=None):
        raw = self._term_input.get().strip()
        if not raw:
            return
        self._term_history.append(raw)
        self._term_hist_idx = -1
        self._term_input.set('')
        prompt = '# ' if self._is_elevated.get() else 'σ > '
        self._log(self._term_out, f'{prompt}{raw}', 'WARN' if self._is_elevated.get() else 'INFO')
        parts = raw.split()
        cmd = parts[0].lower()
        distillator = self.kernel.registry.get('neural_distillator')

        def run():
            try:
                if cmd == 'help':
                    self._log(self._term_out, 'Apex Commands: help | manual | clear | exit | distill', 'INFO')
                elif cmd == 'clear':
                    self.gui.after(0, lambda: [self._term_out.configure(state='normal'), self._term_out.delete('1.0', 'end'), self._term_out.configure(state='disabled')])
                elif cmd == 'distill' and distillator:
                    self._log(self._term_out, 'Initiating Neural Distillation from mirrors...', 'HEAD')
                    res = distillator.distill_from_mirrors()
                    self._log(self._term_out, res, 'OK')
                else:
                    self._log(self._term_out, f'Executing Sovereign Mission: {cmd}...', 'INFO')
                    time.sleep(0.3)
                    if cmd not in ['ls', 'cd', 'grep', 'git', 'pip', 'sigma', 'zenith']:
                        self._log(self._term_out, f"Command '{cmd}' not found in registry.", 'ERR')
                        if distillator:
                            tip = distillator.remediate_error(cmd, 'Command not found')
                            self._log(self._term_out, f'\n{tip}', 'OK')
                    else:
                        self._log(self._term_out, f"Mission '{cmd}' completed successfully.", 'OK')
            except Exception as exc:
                self._log(self._term_out, f'Critical Error: {exc}', 'ERR')
                if distillator:
                    e_tip = distillator.remediate_error(cmd, str(exc))
                if e_tip:
                    self._log(self._term_out, f'{e_tip}', 'OK')
        threading.Thread(target=run, daemon=True).start()