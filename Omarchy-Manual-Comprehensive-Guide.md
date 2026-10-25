# The Complete Omarchy 4.0.5 User Manual

This comprehensive guide brings together all 51 chapters of the authoritative **Omarchy 4.0.5 Manual** (by DHH and contributors), fully integrated and cross-referenced with SigmaOS architecture and Zenith desktop equivalents.

---

## 📑 Table of Contents

### Part I: Foundations & Navigation
- **Chapter 01**: [[Welcome to Omarchy!|Omarchy-Manual-01-Welcome-To-Omarchy]]
- **Chapter 02**: [[Getting Started|Omarchy-Manual-02-Getting-Started]]
- **Chapter 03**: [[Coming From Mac or Windows|Omarchy-Manual-03-Coming-From-Mac-Or-Windows]]
- **Chapter 04**: [[Navigation|Omarchy-Manual-04-Navigation]]
- **Chapter 05**: [[The Top Bar|Omarchy-Manual-05-The-Top-Bar]]
- **Chapter 06**: [[Themes|Omarchy-Manual-06-Themes]]
- **Chapter 07**: [[Hotkeys|Omarchy-Manual-07-Hotkeys]]

### Part II: Productivity, Audio & System Utilities
- **Chapter 08**: [[Unified Clipboard & History|Omarchy-Manual-08-Unified-Clipboard-History]]
- **Chapter 09**: [[Reminders|Omarchy-Manual-09-Reminders]]
- **Chapter 10**: [[Notices|Omarchy-Manual-10-Notices]]
- **Chapter 11**: [[Text Extraction & Dictation|Omarchy-Manual-11-Text-Extraction-Dictation]]
- **Chapter 12**: [[Screenshots & Recording|Omarchy-Manual-12-Screenshots-Recording]]
- **Chapter 13**: [[Toggles, Idle & the Screensaver|Omarchy-Manual-13-Toggles-Idle-Screensaver]]

### Part III: CLI, Terminal & Development Tools
- **Chapter 14**: [[Omarchy CLI|Omarchy-Manual-14-Omarchy-Cli]]
- **Chapter 15**: [[Terminal|Omarchy-Manual-15-Terminal]]
- **Chapter 16**: [[Neovim|Omarchy-Manual-16-Neovim]]
- **Chapter 17**: [[AI|Omarchy-Manual-17-Ai]]
- **Chapter 18**: [[Development Tools|Omarchy-Manual-18-Development-Tools]]
- **Chapter 19**: [[Shell Tools|Omarchy-Manual-19-Shell-Tools]]
- **Chapter 20**: [[Shell Functions|Omarchy-Manual-20-Shell-Functions]]

### Part IV: Applications, Gaming & Web
- **Chapter 21**: [[TUIs|Omarchy-Manual-21-Tuis]]
- **Chapter 22**: [[GUIs|Omarchy-Manual-22-Guis]]
- **Chapter 23**: [[Browsers|Omarchy-Manual-23-Browsers]]
- **Chapter 24**: [[Commercial apps/services|Omarchy-Manual-24-Commercial-Apps-Services]]
- **Chapter 25**: [[Web Apps|Omarchy-Manual-25-Web-Apps]]
- **Chapter 26**: [[Gaming|Omarchy-Manual-26-Gaming]]
- **Chapter 27**: [[Filling out PDFs|Omarchy-Manual-27-Filling-Out-Pdfs]]
- **Chapter 28**: [[Windows VM|Omarchy-Manual-28-Windows-Vm]]
- **Chapter 29**: [[Other Packages|Omarchy-Manual-29-Other-Packages]]

### Part V: System Management, Hardware & Display
- **Chapter 30**: [[Updates|Omarchy-Manual-30-Updates]]
- **Chapter 31**: [[Dotfiles|Omarchy-Manual-31-Dotfiles]]
- **Chapter 32**: [[Shell Plugins|Omarchy-Manual-32-Shell-Plugins]]
- **Chapter 33**: [[Monitors|Omarchy-Manual-33-Monitors]]
- **Chapter 34**: [[Keyboard, Mouse, Trackpad|Omarchy-Manual-34-Keyboard-Mouse-Trackpad]]
- **Chapter 35**: [[Networking|Omarchy-Manual-35-Networking]]
- **Chapter 36**: [[System sleep|Omarchy-Manual-36-System-Sleep]]
- **Chapter 37**: [[Hardware authentication|Omarchy-Manual-37-Hardware-Authentication]]
- **Chapter 38**: [[Fonts|Omarchy-Manual-38-Fonts]]
- **Chapter 39**: [[Backgrounds|Omarchy-Manual-39-Backgrounds]]
- **Chapter 40**: [[Prompt|Omarchy-Manual-40-Prompt]]

### Part VI: Theming, Customization & Troubleshooting
- **Chapter 41**: [[Branding|Omarchy-Manual-41-Branding]]
- **Chapter 42**: [[Common tweaks|Omarchy-Manual-42-Common-Tweaks]]
- **Chapter 43**: [[Making your own theme|Omarchy-Manual-43-Making-Your-Own-Theme]]
- **Chapter 44**: [[Mac support|Omarchy-Manual-44-Mac-Support]]
- **Chapter 45**: [[Troubleshooting|Omarchy-Manual-45-Troubleshooting]]
- **Chapter 46**: [[FAQ|Omarchy-Manual-46-Faq]]

### Part VII: Advanced Administration & Deployment
- **Chapter 47**: [[System snapshots|Omarchy-Manual-47-System-Snapshots]]
- **Chapter 48**: [[Security|Omarchy-Manual-48-Security]]
- **Chapter 49**: [[Omarchy on...|Omarchy-Manual-49-Omarchy-On]]
- **Chapter 50**: [[Dual Boot Install|Omarchy-Manual-50-Dual-Boot-Install]]
- **Chapter 51**: [[Unattended Installs|Omarchy-Manual-51-Unattended-Installs]]

---

## 🚀 SigmaOS Architectural Supremacy Over Omarchy
While Omarchy relies on a collection of Arch packages, external Python scripts, and Hyprland/Wayland wrappers, SigmaOS implements these capabilities in native `#![no_std]` Rust with zero runtime memory bloat and microsecond latency:
- **Zero-Cost IPC**: Direct kernel message channels replacing Unix sockets and shell script wrappers.
- **Native Compositor**: Built-in hardware-accelerated Zenith compositor replacing Hyprland/Quickshell dependencies.
- **Unified Package Engine**: Content-addressable transactional `SigPkg` package manager replacing pacman/AUR fragmentation.
