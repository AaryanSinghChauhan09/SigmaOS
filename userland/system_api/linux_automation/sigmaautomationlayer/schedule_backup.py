# Generated method: SigmaAutomationLayer.schedule_backup
import time
import json
import uuid
import threading
from pathlib import Path
from typing import Dict, List, Any

class SigmaAutomationLayer:
    def schedule_backup(self, target_dir: str, cron_expr: str, retention_days: int) -> str:
        """Schedules an enterprise backup routine."""
        job_id = f'BKP-{uuid.uuid4().hex[:6].upper()}'
        self.backups_schedule.append({'id': job_id, 'target': target_dir, 'cron': cron_expr, 'retention': retention_days, 'last_run': None})
        self._save_data(self.backups_file, self.backups_schedule)
        return f'Backup job {job_id} scheduled ({cron_expr}).'