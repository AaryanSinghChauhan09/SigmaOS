# Generated method: PosixLayer.send_signal


class PosixLayer:
    def send_signal(self, pid, sig_num):
        sig_name = self.signals.get(sig_num, 'UNKNOWN')
        print(f'[POSIX] Sending {sig_name} to PID {pid}')