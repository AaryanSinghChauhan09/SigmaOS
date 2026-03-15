# Generated method: PosixLayer.write_pipe


class PosixLayer:
    def write_pipe(self, pipe_id, data):
        if pipe_id in self.active_pipes:
            self.active_pipes[pipe_id].append(data)
            return True
        return False