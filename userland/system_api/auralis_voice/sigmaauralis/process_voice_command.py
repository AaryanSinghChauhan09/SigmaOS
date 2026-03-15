# Generated method: SigmaAuralis.process_voice_command
import os
import sys
import threading
import time
import json
import subprocess
from pathlib import Path

class SigmaAuralis:
    def process_voice_command(self, audio_data_raw: str):
        """
            USP: Local Inference Routing via Local AI Nexus.
            Translates 'Maximize this' into 'sigma_core.layout.maximize_active()'
            """
        clean_command = audio_data_raw.strip().lower()
        if clean_command.startswith(self.hotword.lower()):
            clean_command = clean_command[len(self.hotword):].strip(', ').strip()
        print(f'[AURALIS] Processing Local Command: {clean_command}')
        self.history.append({'t': time.time(), 'cmd': clean_command})
        intent_res = None
        if self.kernel:
            automator = self.kernel.registry.get('omni_work') or self.kernel.registry.get('omni_automator')
            if automator:
                intent_res = automator.launch_agentic_pipeline(clean_command)
        response_msg = f'Auralis processed: {clean_command}'
        if 'lock' in clean_command:
            if self.kernel:
                self.kernel.bus.emit('kernel.security', {'action': 'lockdown'})
            response_msg = '🔒 Execution: System Lockdown for Sovereignty.'
        elif 'sync' in clean_command or 'github' in clean_command:
            root = Path(__file__).parent.parent.parent
            sync_script = root / 'sync.ps1' if sys.platform == 'win32' else root / 'sync.sh'
            if sync_script.exists():
                subprocess.Popen(['powershell.exe', '-File', str(sync_script)] if sys.platform == 'win32' else ['bash', str(sync_script)], shell=True)
                response_msg = '♻️ Execution: Workspace Sync to GitHub initiated.'
            else:
                response_msg = '⚠️ Error: Sync script not found in root.'
        elif 'open' in clean_command and 'browser' in clean_command:
            if self.kernel:
                self.kernel.bus.emit('app.launch', {'app': 'browser'})
            response_msg = '🌐 Execution: Launching Sovereign Browser.'
        elif 'optimize' in clean_command or 'debloat' in clean_command:
            if self.kernel:
                self.kernel.bus.emit('sys.optimize', {'level': 'Apex'})
            response_msg = '⚡ Execution: System Optimization & De-bloat active.'
        elif 'focus' in clean_command:
            if self.kernel:
                self.kernel.bus.emit('mode.change', {'mode': 'Focus'})
            response_msg = '📵 Execution: Strategic Focus Mode engaged.'
        elif intent_res:
            response_msg = f'🧠 AI Analysis: {intent_res}'
        if self.kernel:
            self.kernel.bus.emit('auralis.command_executed', {'command': clean_command, 'response': response_msg})
        return response_msg