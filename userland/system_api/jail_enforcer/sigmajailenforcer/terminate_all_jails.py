# Generated method: SigmaJailEnforcer.terminate_all_jails


class SigmaJailEnforcer:
    def terminate_all_jails(self):
        """Standard 'Power-Scrub' to kill and wipe all jailed environments."""
        count = len(self.active_jails)
        self.active_jails = []
        return f'SigmaJail: Purged {count} incarcerated environments. Memory scrubbed.'