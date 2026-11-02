# SigmaOS-chatroom

Built-in IRC and chat client providing multi-server connections, channel management, and notifications. Modeled after the Hexchat client architecture.

**Upstream inspiration:** https://github.com/hexchat/hexchat

**Dependencies:** `openssl`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-chatroom /usr/bin/sigma-chatroom
```
