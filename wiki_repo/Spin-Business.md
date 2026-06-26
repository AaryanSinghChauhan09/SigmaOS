# SigmaOS Business Spin — Productivity & Enterprise Edition

The **SigmaOS Business** spin delivers a sovereign, privacy-respecting workplace environment — combining enterprise-grade office productivity, encrypted communications, and project management tools. Inspired by Ubuntu's enterprise focus but without vendor lock-in.

---

## 📧 Email & Calendar

| Tool | Purpose |
|------|---------|
| Thunderbird | Full-featured email client (IMAP/SMTP/PGP) |
| Evolution | GNOME-native email + calendar + contacts |
| Sovereign Mail Client | Lightweight encrypted sovereign alternative |
| KOrganizer | Calendar & task management |

## 📊 Office & Documents

- **LibreOffice** — full office suite (Writer, Calc, Impress, Base)
- **OnlyOffice Desktop** — Microsoft Office-compatible editing
- **Okular** — PDF reader with form filling & digital signatures
- **Pandoc** — universal document format converter

## 📋 Project Management

- **ProjectLibre** — Microsoft Project-compatible Gantt charts
- **Planka** — sovereign self-hosted Kanban board (Trello alternative)
- **Taiga** — agile project management (Flatpak)
- **Timewarrior + Taskwarrior** — CLI-based sovereign time & task tracking

## 💬 Collaboration & Messaging

| Tool | Protocol |
|------|---------|
| Element | Matrix (end-to-end encrypted) |
| Rocket.Chat | Self-hosted Slack alternative |
| Jitsi Meet | Self-hosted video conferencing |
| Nextcloud Talk | Integrated with Nextcloud suite |

## 🏦 Finance & Accounting

- **GnuCash** — double-entry accounting (SMB-ready)
- **KMyMoney** — personal & small business finance
- **Sovereign ERP Module** — SigmaOS-native invoicing & ledger stubs

## ☁ Cloud & Sync

- **Nextcloud Desktop** — sovereign file sync & collaboration
- **Rclone** — bridge to S3, Backblaze, or sovereign object storage
- **Syncthing** — peer-to-peer encrypted file synchronization

## 🔒 Security & Compliance

- **KeePassXC** — sovereign password manager
- **VeraCrypt** — encrypted container volumes
- **GnuPG** — email/file signing & encryption
- **AppArmor** business profiles for browser & mail sandboxing

## 📱 Device Management

- MDM-compatible policies via sovereign `sigma-policy` daemon
- Active Directory / LDAP integration (via SSSD)
- Printer & scanner support via CUPS + SANE

---

## 🚀 Installation

```bash
sigma-spin install business
```

## 📚 See Also

- [Encrypted Volume Management](Security-Architecture.md)
- [Sovereign Sandbox](Sovereign-Sandbox.md)
