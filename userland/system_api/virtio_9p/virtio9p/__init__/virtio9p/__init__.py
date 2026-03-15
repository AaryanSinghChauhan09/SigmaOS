# Generated method: Virtio9P.__init__
import uuid

class Virtio9P:
    def __init__(self, kernel):
        self.kernel = kernel
        self.mount_tag = 'host_data'
        self.session_active = False
        self.msize = 8192
        self.pci_id = {'vendor': 6900, 'device': 4105}
        self.fid_counter = 1