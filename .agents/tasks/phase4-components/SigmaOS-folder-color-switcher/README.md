# SigmaOS-folder-color-switcher

Folder color customizer that integrates with the file manager context menu, allowing users to tag directories with custom icon colors from the current theme.

**Upstream inspiration:** https://github.com/linuxmint/folder-color-switcher

**Dependencies:** `python3-gi`, `SigmaOS-common`, `nemo`

**Build/Install:**
```
pip install -e .
sudo install -m755 src/main.py /usr/lib/sigma/folder-color-switcher
```
