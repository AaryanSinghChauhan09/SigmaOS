# Generated method: SigmaISOManager.__init__


class SigmaISOManager:
    def __init__(self, kernel):
        self.kernel = kernel
        self.iso_version = '2.0.0-Sovereign'
        base_user_dir = os.environ.get('USERPROFILE') or os.environ.get('HOME')
        if base_user_dir:
            self.iso_path = Path(base_user_dir) / '.gemini' / 'antigravity' / 'scratch' / 'SigmaOS' / 'ISO_IMAGE' / 'SigmaOS_Sovereign_v2.iso'
            self.config_dir = Path(base_user_dir) / '.gemini' / 'antigravity' / 'scratch' / 'SigmaOS' / 'config' / 'zenith'
        else:
            self.iso_path = Path('SigmaOS_Sovereign_v2.iso')
            self.config_dir = Path('config/zenith')
        self.config_dir.mkdir(parents=True, exist_ok=True)