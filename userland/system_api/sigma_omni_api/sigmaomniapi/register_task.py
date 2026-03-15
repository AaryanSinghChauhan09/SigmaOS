# Generated method: SigmaOmniAPI.register_task
import os
import json
import subprocess
import sys

class SigmaOmniAPI:
    def register_task(self, name, command):
        """Registers a recurring or triggered automation task."""
        self.task_registry[name] = command
        print(f"[*] Task '{name}' Registered for Sovereign Automation.")