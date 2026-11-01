# SigmaOS-player

SigmaOS-player is the media player application for SigmaOS, supporting wide-format video and audio playback via GStreamer with a clean, minimal interface, playlist management, chapter navigation, subtitle loading and rendering, and network stream support. It integrates with the SigmaOS media stack and respects system audio and MPRIS2 media keys for a consistent desktop media experience.

**Upstream reference projects:** [xplayer](https://github.com/linuxmint/xplayer)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-player.git
cd SigmaOS-player

# Install Python dependencies
pip install -r requirements.txt

# Run the player
python3 src/main.py
```

Dependencies: `python3 >= 3.10`, `python3-gi`, `gstreamer1.0-plugins-good`, `gstreamer1.0-plugins-bad`
