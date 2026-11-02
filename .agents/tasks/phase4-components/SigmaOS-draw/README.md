# SigmaOS-draw

## Overview

SigmaOS-draw is a vector drawing and screen annotation application for SigmaOS. It supports SVG output, freehand drawing, shape primitives, and presentation aids such as a laser-pointer mode and slide overlays. It integrates with the SigmaOS Wayland compositor for low-latency stylus input.

## Features

- **SVG export**: All drawings are stored and exported as standards-compliant SVG files.
- **Screen annotation**: Overlay annotations directly onto the desktop or running applications.
- **Presentation mode**: Spotlight, pointer, and highlight tools for live presentations.
- **Layer support**: Multiple drawing layers with independent opacity and blend modes.
- **Stylus pressure**: Pressure-sensitive line weight when used with a graphics tablet.
- **Shape library**: Built-in primitives — rectangles, ellipses, arrows, callouts.

## Upstream References

- [drawing](https://github.com/maoschanz/drawing) — Simple image editor for GNOME, used as a reference for GTK integration patterns.
- [xournalpp](https://xournalpp.github.io/) — Feature-rich handwriting note-taking and annotation tool.

## Building

```bash
pip install pylint
pylint src/ || true
python3 src/main.py
```

Requires Python 3.10+ and GTK 4 via PyGObject.

## Contributing

1. Fork and create a feature branch from `main`.
2. Follow PEP 8 and the SigmaOS Python coding guidelines.
3. Add or update tests for any new drawing primitives.
4. Open a pull request with a description of the change and screenshots where applicable.
5. All contributions are released under the MIT License.
