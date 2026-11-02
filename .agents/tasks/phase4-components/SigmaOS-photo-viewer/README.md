# SigmaOS-photo-viewer

SigmaOS-photo-viewer is a fast, lightweight image viewer for SigmaOS supporting all common image formats including JPEG, PNG, WebP, HEIC, and RAW files, with a clean single-window interface, basic editing tools such as crop and rotate, colour adjustment sliders, and a full-screen slideshow mode. It integrates with the SigmaOS file manager for seamless browsing and is built with accessibility and keyboard navigation in mind.

**Upstream reference projects:** [pix](https://github.com/linuxmint/pix), [xviewer](https://github.com/linuxmint/xviewer)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-photo-viewer.git
cd SigmaOS-photo-viewer

# Install Python dependencies
pip install -r requirements.txt

# Run the photo viewer
python3 src/main.py
```

Dependencies: `python3 >= 3.10`, `python3-gi`, `libgtk-4-dev`, `libheif-dev`
