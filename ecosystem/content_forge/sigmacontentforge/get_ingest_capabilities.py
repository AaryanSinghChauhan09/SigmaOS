# Generated method: SigmaContentForge.get_ingest_capabilities
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import time

class SigmaContentForge:
    def get_ingest_capabilities(self):
        """Returns the list of supported content-types and actions."""
        return {'Input': ['PDF', 'DOCX', 'PNG', 'MKV', 'EXE_Binary', 'Memory_Dump'], 'Actions': ['Audit', 'Convert', 'Redact', 'Shard', 'OCR', 'Sign'], 'Encryption': 'Lattice-PQC (Kyber-1024)'}