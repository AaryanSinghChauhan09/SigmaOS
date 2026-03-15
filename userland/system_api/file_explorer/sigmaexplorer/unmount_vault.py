# Generated method: SigmaExplorer.unmount_vault
import os
import time

class SigmaExplorer:
    def unmount_vault(self, provider: str) -> str:
        mount_point = f'SigmaVault://{provider}'
        if mount_point in self.active_mounts:
            self.active_mounts.remove(mount_point)
            return f'Unmounted {mount_point} safely.'
        return 'Vault not found.'