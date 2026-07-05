// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SigmaPersonalizer (Rust, no_std)
//! Declarative configuration and user environment parser.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

pub const MAX_PACKAGES: usize = 32;
pub const NAME_MAX: usize = 32;

#[derive(Copy, Clone)]
pub struct DeclarativePackage {
    pub name: [u8; NAME_MAX],
    pub name_len: usize,
    pub enable: bool,
}

pub struct SigmaPersonalizer {
    active: bool,
    theme_dark: bool,
    font_size: u32,
    packages: [Option<DeclarativePackage>; MAX_PACKAGES],
    package_count: usize,
}

impl SigmaPersonalizer {
    pub const fn new() -> Self {
        SigmaPersonalizer {
            active: false,
            theme_dark: true,
            font_size: 12,
            packages: [None; MAX_PACKAGES],
            package_count: 0,
        }
    }

    pub fn init(&mut self) -> SigmaStatus {
        self.active = true;
        SIGMA_OK
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn apply_theme(&mut self, dark_mode: bool, size: u32) {
        self.theme_dark = dark_mode;
        self.font_size = size;
    }

    pub fn add_package(&mut self, pkg_name: &[u8]) -> SigmaStatus {
        if self.package_count >= MAX_PACKAGES {
            return SIGMA_ERROR;
        }

        let mut name = [0u8; NAME_MAX];
        let len = core::cmp::min(pkg_name.len(), NAME_MAX);
        for i in 0..len {
            name[i] = pkg_name[i];
        }

        self.packages[self.package_count] = Some(DeclarativePackage {
            name,
            name_len: len,
            enable: true,
        });
        self.package_count += 1;
        SIGMA_OK
    }
}

static mut G_INSTANCE: SigmaPersonalizer = SigmaPersonalizer::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_personalizer_init() -> SigmaStatus {
    G_INSTANCE.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_personalizer_active() -> u8 {
    G_INSTANCE.is_active() as u8
}

#[no_mangle]
pub unsafe extern "C" fn sigma_personalizer_configure(dark_mode: u8, font_size: u32) {
    G_INSTANCE.apply_theme(dark_mode != 0, font_size);
}