# Generated method: SovereignBridge.__init__
import os
import sys
import ctypes
import os as _pathlib_os
class Path:
    def __init__(self, p): self._p = str(p)
    def __str__(self): return self._p
    def __truediv__(self, o): return Path(_pathlib_os.path.join(self._p, str(o)))
    def exists(self): return _pathlib_os.path.exists(self._p)
    def is_dir(self): return _pathlib_os.path.isdir(self._p)
    def is_file(self): return _pathlib_os.path.isfile(self._p)
    def mkdir(self, parents=False, exist_ok=False):
        if not _pathlib_os.path.exists(self._p):
            try: _pathlib_os.makedirs(self._p)
            except OSError: pass
    def resolve(self): return Path(_pathlib_os.path.abspath(self._p))
    @property
    def name(self): return _pathlib_os.path.basename(self._p)
    @property
    def stem(self): return _pathlib_os.path.splitext(self.name)[0]
    @property
    def suffix(self): return _pathlib_os.path.splitext(self.name)[1]
    @property
    def parent(self): return Path(_pathlib_os.path.dirname(self._p))
    def __fspath__(self): return self._p
    def __repr__(self): return f'Path({self._p!r})'
    def iterdir(self):
        try: return [Path(_pathlib_os.path.join(self._p,n)) for n in _pathlib_os.listdir(self._p)]
        except OSError: return []
    def read_text(self, encoding='utf-8', errors='replace'):
        fd=_pathlib_os.open(self._p,_pathlib_os.O_RDONLY); d=b''
        while True:
            c=_pathlib_os.read(fd,65536)
            if not c: break
            d+=c
        _pathlib_os.close(fd); return d.decode(encoding,errors)
    def write_text(self, text, encoding='utf-8'):
        b=text.encode(encoding)
        fd=_pathlib_os.open(self._p,_pathlib_os.O_WRONLY|_pathlib_os.O_CREAT|_pathlib_os.O_TRUNC,0o666)
        _pathlib_os.write(fd,b); _pathlib_os.close(fd)

class SovereignBridge:
    def __init__(self, kernel):
        self.kernel = kernel
        self._c_lib = None
        self._rust_lib = None
        self._load_binaries()