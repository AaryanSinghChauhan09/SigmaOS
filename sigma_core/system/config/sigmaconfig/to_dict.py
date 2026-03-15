# Generated method: SigmaConfig.to_dict


class SigmaConfig:
    def to_dict(self):
        """Export configuration as dictionary"""
        return {'os_name': self.OS_NAME, 'version': self.VERSION, 'build': self.BUILD, 'features': {'gui': self.ENABLE_GUI, 'cli': self.ENABLE_CLI, 'agentic': self.ENABLE_AGENTIC, 'mesh': self.ENABLE_MESH}, 'security': {'zero_trust': self.ZERO_TRUST_MODE, 'level': self.SECURITY_LEVEL, 'ebpf': self.ENABLE_EBPF_MONITORING}}