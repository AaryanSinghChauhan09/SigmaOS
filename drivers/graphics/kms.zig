// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Kernel Mode Setting (KMS) Graphics Interface (Zig, no stdlib)
//! Replaces: drivers/graphics/sigma_kms.cpp, sigma_kms.h
//! =========================================================================

pub const KmsMode = struct {
    width: u32,
    height: u32,
    refresh_rate: u32,
};

pub const KmsConnector = struct {
    id: u32,
    connected: bool,
    modes: [4]KmsMode,
    mode_count: usize,
};

pub const KmsDriver = struct {
    mmio_base: usize,
    active_mode: KmsMode,
    connector: KmsConnector,

    pub fn new(mmio: usize) KmsDriver {
        return KmsDriver{
            .mmio_base = mmio,
            .active_mode = KmsMode{ .width = 1920, .height = 1080, .refresh_rate = 60 },
            .connector = KmsConnector{
                .id = 1,
                .connected = true,
                .modes = [_]KmsMode{
                    KmsMode{ .width = 1920, .height = 1080, .refresh_rate = 60 },
                    KmsMode{ .width = 1280, .height = 720, .refresh_rate = 60 },
                    KmsMode{ .width = 1024, .height = 768, .refresh_rate = 60 },
                    KmsMode{ .width = 800, .height = 600, .refresh_rate = 60 },
                },
                .mode_count = 4,
            },
        };
    }

    pub fn set_mode(self: *KmsDriver, width: u32, height: u32) bool {
        var i: usize = 0;
        while (i < self.connector.mode_count) : (i += 1) {
            const m = self.connector.modes[i];
            if (m.width == width and m.height == height) {
                self.active_mode = m;
                return true;
            }
        }
        return false;
    }
};
