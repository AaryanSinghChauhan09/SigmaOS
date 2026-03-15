# Generated method: PosixLayer.__init__


class PosixLayer:
    def __init__(self, kernel):
        self.kernel = kernel
        self.signals = {1: 'SIGHUP', 2: 'SIGINT', 9: 'SIGKILL', 11: 'SIGSEGV'}
        self.active_pipes = {}