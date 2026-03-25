# Generated method: SigmaAutomationLayer.__init__
import time
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaJSON as _json_lib
    class json:
        loads = staticmethod(_json_lib.loads)
        dumps = staticmethod(_json_lib.dumps)
        load  = staticmethod(lambda f: _json_lib.loads(f.read()))
        dump  = staticmethod(lambda d, f, **kw: f.write(_json_lib.dumps(d)))
except Exception:
    import json
import uuid
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaThread as _T, SigmaLock as _L
    class threading:
        Thread = _T; Lock = _L; RLock = _L; Event = _L
        @staticmethod
        def current_thread(): return None
        @staticmethod
        def active_count(): return 1
except Exception:
    import threading
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
from typing import Dict, List, Any

class SigmaAutomationLayer:
    def __init__(self, kernel):
        self.kernel = kernel
        self.config_dir = Path(_os.path.join(_sigma_root, 'config', 'automation'))
        self.config_dir.mkdir(parents=True, exist_ok=True)
        self.backups_file = self.config_dir / 'scheduled_backups.json'
        self.users_file = self.config_dir / 'users.json'
        self.updates_file = self.config_dir / 'updates.json'
        self.backups_schedule = self._load_data(self.backups_file, [])
        self.users = self._load_data(self.users_file, {'root': {'uid': 0, 'groups': ['root', 'wheel', 'sudo']}})
        self.update_policy = self._load_data(self.updates_file, {'auto_update': True, 'channel': 'stable', 'time': '03:00'})
        self._start_automation_daemon()