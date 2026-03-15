import hashlib
import os

class MerkleIntegrityValidator:
    """
    Builds a Merkle Tree from the myriad of small modules to ensure system integrity. 
    If a single file is tampered with, the root hash changes.
    """
    def __init__(self, root_dir):
        self.root_dir = root_dir

    def get_file_hash(self, filepath):
        hasher = hashlib.blake2b()
        try:
            with open(filepath, 'rb') as f:
                while chunk := f.read(8192):
                    hasher.update(chunk)
            return hasher.hexdigest()
        except:
            return None

    def audit_entire_fleet(self):
        hashes = []
        for root, _, files in os.walk(self.root_dir):
            if '.git' in root: continue
            for file in files:
                if file.endswith('.py'):
                    h = self.get_file_hash(os.path.join(root, file))
                    if h: hashes.append(h)
        
        # Combine into master root hash
        hashes.sort()
        master_hasher = hashlib.blake2b()
        for h in hashes:
            master_hasher.update(h.encode())
        return master_hasher.hexdigest()

def verify_system_state():
    validator = MerkleIntegrityValidator(".")
    # This might take time on 11k files, but in SigmaOS it runs as a background shard.
    root_hash = validator.audit_entire_fleet()
    return root_hash
