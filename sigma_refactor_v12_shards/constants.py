from functools import lru_cache
import os
import ast
import textwrap
import re

ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'artifacts', '.gemini'}
PROTECTED_FILES = {'__init__.py', 'sigma_refactor_v12.py', 'bulletproof_healer.py', 'nuclear_flatten.py', 'base_sovereign.py', 'system_factory.py', 'decorators.py', 'system_interfaces.py', 'driver_interfaces.py', 'security_interfaces.py', 'event_interfaces.py', 'command_interfaces.py', 'verification_interfaces.py', 'resilience_interfaces.py', 'storage_interfaces.py', 'kernel_core.py', 'memory_manager.py', 'sovereignty_manager.py', 'device_manager.py', 'event_bus.py', 'commander.py', 'scheduler.py', 'scheduler_strategies.py', 'power_manager.py', 'chaos_monkey.py', 'privacy_guard.py', 'fractal_storage.py', 'proof_ledger.py', 'system_auditor.py', 'kernel_states.py', 'chat_engine.py', 'base_sovereign_page.py', 'bootstrap.py'}
PROTECTED_DIRS = {'interfaces', 'kernel', 'security', 'drivers', 'analytics', 'social'}
PERSONAL = re.compile('\\bSOVEREIGN_USER\\b|\\bchauhan\\b', re.I)
RELIGIOUS = re.compile('\\bgod\\b|\\blord\\b|\\bfaith\\b|\\bspirit\\b|\\bholy\\b|\\bprayer\\b', re.I)
VULGAR = re.compile('\\bshit\\b|\\bfuck\\b|\\bbitch\\b|\\bdamn\\b', re.I)