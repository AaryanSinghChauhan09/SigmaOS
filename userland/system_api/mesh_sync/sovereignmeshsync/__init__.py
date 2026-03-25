# Generated method: SovereignMeshSync.__init__
import os
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaFS as _sfs
    class shutil:
        copy = staticmethod(_sfs.copy); copy2 = staticmethod(_sfs.copy)
        copytree = staticmethod(lambda s,d,**kw: None)
        rmtree   = staticmethod(lambda p,**kw: None)
        move     = staticmethod(_sfs.copy)
except Exception:
    import shutil
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaHash as _sigma_hash
    class hashlib:
        @staticmethod
        def sha256(data=b''):
            class _H:
                def __init__(self, d): self._d = d if isinstance(d,bytes) else d.encode()
                def hexdigest(self): return _sigma_hash.fnv1a_hex(self._d)
                def digest(self): return _sigma_hash.fnv1a_64(self._d).to_bytes(8,'big')
                def update(self, d): self._d += d if isinstance(d,bytes) else d.encode()
            return _H(data)
        md5 = sha256; sha1 = sha256; sha3_256 = sha256; sha3_512 = sha256
except Exception:
    import hashlib
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
from typing import List, Dict

class SovereignMeshSync:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.watched_folders: List[Path] = []
        self._sync_history: List[Dict[str, str]] = []
        self._connected_peers: List[str] = []