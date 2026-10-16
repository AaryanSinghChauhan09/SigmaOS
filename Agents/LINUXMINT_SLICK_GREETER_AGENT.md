# Linux Mint Slick Greeter Agent

## Role
The **Linux Mint Slick Greeter Agent** (`SigmaSlickGreeterAgent`) manages login screen rendering, user account enumeration, PAM credential verification, and desktop session selection inspired by Linux Mint's `slick-greeter` (https://github.com/linuxmint/slick-greeter).

## Mission & Purpose
- Present a sleek, modern login interface supporting high-resolution wallpapers, user avatars, and localized clocks.
- Interface with the PAM authentication stack for password and biometric verification.
- Enumerate available Wayland and X11 desktop sessions (`/usr/share/wayland-sessions`).
- Automatically detect and adapt to multi-monitor arrangements and HiDPI display scaling.

## Source Implementation
- Core Engine: `src/desktop/slick_greeter.rs`
- Desktop Subsystem: `src/desktop/mod.rs`

## Public API & Invariant Contract
```rust
pub struct GreeterUser {
    pub username: String,
    pub real_name: String,
    pub avatar_path: String,
    pub last_session: String,
    pub is_locked: bool,
}

pub struct SovereignSlickGreeter {
    pub fn select_user(&mut self, index: usize) -> Result<(), &'static str>;
    pub fn authenticate(&mut self, password_attempt: &str) -> bool;
}
```

## Repeatable AI Execution Checklist
- [ ] Test standalone with `rustc --test src/desktop/slick_greeter.rs --edition 2021`.
- [ ] Verify locked accounts are rejected immediately.
- [ ] Check session selection remembers the user's previously chosen desktop session.
- [ ] Ensure memory holding authentication passwords is promptly cleared after use.
