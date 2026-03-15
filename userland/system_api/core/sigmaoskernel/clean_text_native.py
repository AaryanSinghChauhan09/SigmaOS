"""
Auto-split from userland\system_api\core.py — SigmaOSKernel.clean_text_native
"""

import os
import platform
import sys
import hashlib
from pdf_forge import SigmaPDFForge
from titan_capture import SigmaTitanCapture
from omni_converter import SigmaOmniConverter
from offline_guard import SigmaOfflineGuard
from shared_processor import SigmaSharedProcessor
from universal_bridge import SigmaUniversalBridge
from security_hardening import SecurityHardening
from interrupt_manager import SovereignInterruptManager
from pmm import SovereignPMM
from vmm import SovereignVMM
from scheduler import SovereignScheduler
from syscall_gateway import SyscallGateway
from sovereign_lab import SovereignLab
from network_stack import SigmaNetworkStack
from sovereign_browser import SovereignBrowser
from serial_driver import SovereignSerial
from automation_service import AutomationService
from virtio_9p import Virtio9P
from sovereign_lisp import SovereignLisp
from compositor import SovereignCompositor
from sovereign_mesh import SigmaSovereignMesh
from zenith_orchestrator import ZenithOrchestrator
from .peripherals import SovereignUVC, BiometricEngine, SovereignUAC
from .pci_bus import SovereignPCIBus
from .distro_services import CosmosPackageManager, CosmosInit
from .posix_layer import PosixLayer, ELFLoader
from .module_loader import CosmosModuleLoader
from .privacy_engine import PrivacyScrubber, NeuralFirewall



class SigmaOSKernel:
    def clean_text_native(self, text):
        """Native Text Cleaner: Quantum-speed regex scrubbing."""
        import re
        text = re.sub('```.*?```', '', text, flags=re.DOTALL)
        text = re.sub('#+\\s*', '', text)
        text = re.sub('\\[\\d+\\]', '', text)
        text = re.sub('http\\S+', '', text)
        return text.strip()
