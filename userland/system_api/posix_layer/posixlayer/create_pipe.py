# Generated method: PosixLayer.create_pipe


class PosixLayer:
    def create_pipe(self, pipe_id):
        self.active_pipes[pipe_id] = []
        print(f'[POSIX] Pipe {pipe_id} created.')