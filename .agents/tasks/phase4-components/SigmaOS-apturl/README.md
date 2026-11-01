# SigmaOS-apturl

Handles `apt:` URL scheme to enable one-click package installation from web browsers. Parses the URL, resolves package names, and invokes the package manager with user confirmation dialogs.

**Upstream inspiration:** https://github.com/linuxmint/apturl

**Dependencies:** `python3-apt`, `python3-gi`, `SigmaOS-common`

**Build/Install:**
```
pip install -e .
sudo install -m755 src/main.py /usr/bin/sigma-apturl
sudo update-alternatives --install /usr/bin/apturl apturl /usr/bin/sigma-apturl 50
```
