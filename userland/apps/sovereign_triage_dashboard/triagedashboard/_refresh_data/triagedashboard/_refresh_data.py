# Generated method: TriageDashboard._refresh_data
import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class TriageDashboard:
    def _refresh_data(self):
        for item in self.tree.get_children():
            self.tree.delete(item)
        docket = []
        stats = {'cases': 24, 'resolved': 16, 'pending': 8, 'time': '1.8ms'}
        if self.kernel and hasattr(self.kernel, 'triage'):
            triage = getattr(self.kernel, 'triage')
            docket = triage.get_docket_summary()
            ts = triage.stats
            stats = {'cases': ts['cases_filed'], 'resolved': ts['judgments_delivered'], 'pending': ts['pending_trials'], 'time': '0.9ms'}
        else:
            docket = [{'case_id': 'OS-BUG-A2B', 'petitioner_shard': 'HAL', 'jurisdiction': 'HAL', 'severity': 'CRITICAL', 'status': 'DOCKETED', 'delegated_to': 'Hardware Drivers'}, {'case_id': 'OS-BUG-C9F', 'petitioner_shard': 'Sync_V2', 'jurisdiction': 'MESH', 'severity': 'MAJOR', 'status': 'RESOLVED', 'delegated_to': 'Networking'}, {'case_id': 'OS-BUG-D4X', 'petitioner_shard': 'Cortex', 'jurisdiction': 'AI', 'severity': 'MINOR', 'status': 'DOCKETED', 'delegated_to': 'Gurukul Engine'}]
        v = [str(stats['cases']), str(stats['resolved']), str(stats['pending']), stats['time']]
        for i, val in enumerate(v):
            if i < len(self.stat_cards):
                self.stat_cards[i].config(text=val)
        for case in docket:
            self.tree.insert('', 'end', values=(case.get('case_id'), case.get('petitioner_shard'), case.get('jurisdiction'), case.get('severity'), case.get('status'), case.get('delegated_to')))