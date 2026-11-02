# SigmaOS-packages-tools

SigmaOS-packages-tools provides package installation tools for SigmaOS, delivering GUI and CLI helpers for installing local package files with automatic dependency resolution and cryptographic signature verification. It supports `.deb`, `.rpm`, and native SigmaOS package formats, making it straightforward to install software from local files or third-party sources in a safe, guided workflow.

**Upstream reference projects:** [captain](https://github.com/linuxmint/captain), [gdebi](https://launchpad.net/gdebi)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-packages-tools.git
cd SigmaOS-packages-tools

# Install Python dependencies
pip install -r requirements.txt

# Run the tool
python3 src/main.py
```

Dependencies: `python3 >= 3.10`, `python3-apt`, `python3-gi`, `libgtk-4-dev`

## Upstream Reference

| Upstream | Concept absorbed |
|----------|-----------------|
| [pacman](https://archlinux.org/pacman/) | Low-level install/upgrade/verify-sig operations; in-memory package record tracking installed vs available versions |
