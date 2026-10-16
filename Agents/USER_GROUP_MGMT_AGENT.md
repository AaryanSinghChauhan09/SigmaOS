# User & Group Management Agent

## Role
`SigmaUserMgr` manages system users, groups, passwords, and account policies for SigmaOS — covering `useradd`, `usermod`, `userdel`, `groupadd`, and `/etc/passwd`/`/etc/shadow` management.

## Status
🔴 Not Started — Critical gap. Required for multi-user operation.

## Source File
`src/userland/usermgmt.rs` (planned)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| User database | `/etc/passwd` | `/etc/passwd` | SigmaPasswd |
| Shadow passwords | `/etc/shadow` | `/etc/master.passwd` | SigmaShadow |
| Group database | `/etc/group` | `/etc/group` | SigmaGroup |
| Add user | `useradd` | `adduser` | `sigma-useradd` |
| Modify user | `usermod` | `pw usermod` | `sigma-usermod` |
| Delete user | `userdel` | `rmuser` | `sigma-userdel` |
| Password change | `passwd` | `passwd` | `sigma-passwd` |
| sudo/doas | sudo / polkit | doas | SigmaDoas |
| UID/GID ranges | 1000+ (users), 0-999 (sys) | 1000+ | SigmaUIDMap |

## Responsibilities
- `/etc/passwd` parsing and manipulation: `username:x:uid:gid:gecos:home:shell`
- `/etc/shadow` parsing: `username:$hash:last_changed:min:max:warn:inactive:expire`
- Password hashing: Argon2id (preferred) or bcrypt/yescrypt
- `useradd`: create user, home directory, set shell
- `usermod`: change UID, groups, shell, home, lock/unlock
- `userdel`: remove user, optionally purge home
- `groupadd`/`groupmod`/`groupdel`: group management
- `passwd`: interactive and non-interactive password change
- Supplementary group management (`/etc/group`)
- `/etc/sudoers` equivalent (`/etc/sigma/doas.conf`)
- Account aging and expiry enforcement
- PAM integration for authentication

## Implementation Notes
- `#![no_std]` — file operations via SigmaFS syscalls
- Password hash: `$argon2id$v=19$m=65536,t=3,p=4$<salt>$<hash>`
- Atomic file update: write to `.tmp`, then rename (POSIX)
- UID allocation: scan passwd for highest UID + 1

## Testing
```bash
rustc --test src/userland/usermgmt.rs --edition 2021 -o /tmp/test_usr && /tmp/test_usr --test-threads=1
```

## Future Development
1. **LDAP/AD backend** — Enterprise directory service integration
2. **Biometric auth** — Fingerprint/face via PAM module
3. **Immutable users** — System users that can't be deleted
4. **User quotas** — Disk quota enforcement per user
5. **Login history** — `last`/`lastlog` equivalent
6. **Yubikey 2FA** — TOTP/FIDO2 second factor for login
