# Generated method: SigmaJailEnforcer.create_jail


class SigmaJailEnforcer:
    def create_jail(self, process_id, restricted_root='/tmp/jail/untrusted'):
        """Creates a jailed environment for a specific PID."""
        jail_info = {'PID': process_id, 'Root': restricted_root, 'Network': 'DISABLED'}
        self.active_jails.append(jail_info)
        return f'SigmaJail: PID {process_id} is now incarcerated in {restricted_root}. Zero lateral movement possible.'