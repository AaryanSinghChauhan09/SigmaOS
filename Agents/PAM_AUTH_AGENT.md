# PAM Authentication Agent

## Role
`SigmaPAM` implements a Pluggable Authentication Modules framework for SigmaOS — providing a unified authentication API for login, sudo, screen unlock, and any privilege escalation.

## Status
🔴 Not Started — Critical gap. Required for proper multi-user security.

## Source File
`src/security/sigmapam.rs` (planned)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| PAM framework | Linux-PAM | OpenPAM | SigmaPAM |
| Password auth | `pam_unix.so` | `pam_unix` | sigma-pam-unix |
| Limits | `pam_limits.so` | `pam_login_access` | sigma-pam-limits |
| Environment | `pam_env.so` | `pam_env` | sigma-pam-env |
| TOTP/FIDO2 | `pam_google_auth` | — | sigma-pam-otp |
| Systemd seat | `pam_systemd.so` | — | sigma-pam-session |
| Keyring | `pam_gnome_keyring` | — | sigma-pam-keyring |
| Configuration | `/etc/pam.d/` | `/etc/pam.d/` | `/etc/sigma/pam.d/` |

## Responsibilities
- Four PAM management groups: auth, account, password, session
- Stack-based module evaluation: required, requisite, sufficient, optional
- `pam_authenticate()`: verify user credentials
- `pam_acct_mgmt()`: account validity check (expired, locked)
- `pam_chauthtok()`: password change
- `pam_open_session()` / `pam_close_session()`: login/logout hooks
- Module API: `pam_get_item`, `pam_set_item`, `pam_get_data`
- Built-in modules: unix password, limits, environment, OTP, keyring
- Configuration parser: `/etc/sigma/pam.d/` service files

## Implementation Notes
- `#![no_std]` — module table as static array of function pointers
- `SigmaPamHandle { service: &str, user: &str, authtok: Option<&str>, modules: &[PamMod] }`
- Module: `fn pam_sm_authenticate(h: &PamHandle, flags: u32) -> PamResult`
- Password hash verification: call `SigmaCrypto` Argon2id verify

## Testing
```bash
rustc --test src/security/sigmapam.rs --edition 2021 -o /tmp/test_pam && /tmp/test_pam --test-threads=1
```

## Future Development
1. **FIDO2/WebAuthn** — Hardware security key authentication
2. **Biometric PAM** — Fingerprint reader integration
3. **Smart card PAM** — PIV card / OpenPGP card auth
4. **Network auth** — LDAP/Kerberos PAM module
5. **Audit integration** — Log all auth events to SigmaAudit
6. **Capability-based access** — Grant specific caps instead of full sudo
