# SigmaOS-actions

## Overview

SigmaOS-actions provides the context menu actions and extension framework for the SigmaOS file manager. It allows users and developers to add custom right-click actions that appear when files or directories are selected, enabling powerful per-filetype workflows directly from the file manager.

## Features

- Declarative action definitions via `.action` descriptor files
- Per-mime-type and per-extension action filtering
- Bulk-selection support (actions can operate on multiple selected files)
- Script-based actions: any executable can be wrapped as a context menu entry
- Built-in actions: compress, open terminal here, copy path, share via email
- Live reload of action definitions without restarting the file manager

## Upstream References

- [cinnamon-spices-actions](https://github.com/linuxmint/cinnamon-spices-actions) — Community-contributed context menu actions for Cinnamon file manager

## Building

This component is written in Python. No compilation step is required.

```bash
# Install development dependencies
pip install pylint

# Lint the source
pylint src/

# Run a quick action dry-run
python3 src/main.py --list
```

Runtime requirements: `python3-gi`, `gir1.2-nautilus-3.0` or equivalent file manager GObject bindings.

## Contributing

1. Fork this repository and create a feature branch.
2. Follow PEP 8 style guidelines; run `pylint src/` before submitting.
3. Each new action should ship with a `.action` descriptor and a matching test invocation example in `docs/`.
4. Open a pull request against `main` with a clear description of the change.
5. All contributions must be licensed under the MIT License.
