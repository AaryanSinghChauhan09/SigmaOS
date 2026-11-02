# SigmaOS-office

SigmaOS-office is the office productivity suite integration layer for SigmaOS, packaging and configuring LibreOffice and OnlyOffice with SigmaOS-native theming, default document templates, and deep desktop integration including file association handling, MIME type registration, and system font integration. It ensures a consistent, polished office experience that feels purpose-built for the SigmaOS desktop.

**Upstream reference projects:** [LibreOffice](https://www.libreoffice.org), [OnlyOffice](https://www.onlyoffice.com)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-office.git
cd SigmaOS-office

# Run the office integration setup
bash src/main.sh
```

Dependencies: `libreoffice`, `onlyoffice-desktopeditors`
