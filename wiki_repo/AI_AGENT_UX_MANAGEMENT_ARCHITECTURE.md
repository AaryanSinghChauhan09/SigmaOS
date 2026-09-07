# AI Agent UX Management Architecture in SigmaOS

## System Architecture

```
+---------------------------------------------------------------------------------+
|                         User Input / Voice Command                              |
|           (Natural Language Prompt, REPL CLI, Voice Speech-to-Text)              |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                               Adaptive UX Agent                                 |
|               (Intent Parser, Workload Classifier, User Profile)                 |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
|  CinnamonThemeEngine  |   | FolderColorSwitcher   |   |   CursorThemeEngine   |
| (Mint-Y, GTK, Adwaita)|   | (Hex, Emblems, Inherit|   | (Adwaita, Breeze, DPI)|
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                            SigmaOS Desktop Portals                              |
|        (Wayland Compositor, GTK Plymouth Splash, X11/XCursor Dispatcher)         |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Adaptive UX Agent Engine**:
   - Classifies real-time system workload (Developer, Gamer, Data Scientist, Daily Office).
   - Generates contextual workspace layouts (tiling grids, floating stacks, or dual-monitor focus).

2. **Customization Subsystems**:
   - **GTK & Cinnamon Theme Engine**: Direct IPC manipulation of Cinnamon desktop themes, panel transparency, applet icon sets, and sound schemes.
   - **Linux Mint / Papirus Folder Color Engine**: Subfolder palette inheritance, custom hex color rendering, and status emblems.
   - **Cursor Theme Engine**: XCursor hotspot translation, animated cursor frame rates, and automatic display scale adjustment.

3. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
