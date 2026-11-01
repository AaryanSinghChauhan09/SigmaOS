# SigmaOS-packages-kit

SigmaOS-packages-kit is the package management toolkit for SigmaOS, providing a high-level DBus API for package installation, removal, and updates across multiple back-end package managers. It abstracts the differences between apt, dnf, and other backends so that graphical tools and system services can interact with packages through a single, consistent interface without requiring root access in the calling process.

**Upstream reference projects:** [aptkit](https://launchpad.net/aptkit), [PackageKit](https://www.freedesktop.org/software/PackageKit/)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-packages-kit.git
cd SigmaOS-packages-kit

# Install Python dependencies
pip install -r requirements.txt

# Start the daemon
python3 src/main.py
```

Dependencies: `python3 >= 3.10`, `python3-dbus`, `python3-gi`
