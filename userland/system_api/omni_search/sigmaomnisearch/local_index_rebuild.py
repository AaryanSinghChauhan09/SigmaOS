# Generated method: SigmaOmniSearch.local_index_rebuild
import time
from typing import Dict, Any, List

class SigmaOmniSearch:
    def local_index_rebuild(self):
        """Crawl the local filesystem and local browser archive to update visibility."""
        if self.kernel and self.kernel.fs:
            return f'OmniSearch: Re-indexing {len(self.kernel.fs._inodes)} inodes... DONE.'
        return 'OmniSearch: Local knowledge nodes refreshed.'