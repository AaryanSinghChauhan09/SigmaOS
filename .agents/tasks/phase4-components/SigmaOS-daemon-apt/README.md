# SigmaOS-daemon-apt

Background APT daemon handling package updates, install queues, and transaction state via D-Bus. Provides the service layer behind the software manager UI.

**Upstream inspiration:** https://github.com/linuxmint/mintinstall

**Dependencies:** `apt`, `dbus`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-daemon-apt /usr/sbin/sigma-daemon-apt
sudo systemctl enable sigma-daemon-apt
```
