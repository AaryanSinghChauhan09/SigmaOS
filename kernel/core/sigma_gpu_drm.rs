// SigmaOS — GPU/DRM/KMS subsystem (sovereign, no external deps)
// Implements: DRM device, framebuffer, mode-setting, page-flipping
#![no_std]
#![allow(dead_code)]

pub const DRM_MAX_DEVICES:    usize = 4;
pub const DRM_MAX_CONNECTORS: usize = 8;
pub const DRM_MAX_CRTCS:      usize = 4;
pub const DRM_MAX_PLANES:     usize = 16;
pub const DRM_MAX_GEM_OBJECTS: usize = 1024;

// ─── Pixel Formats ───────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PixelFormat {
    XRGB8888, ARGB8888, XBGR8888, RGB565, RGBA1010102, NV12, YUV420,
}
impl PixelFormat {
    pub fn bpp(&self) -> u32 {
        match self {
            Self::RGB565 => 16,
            Self::NV12 | Self::YUV420 => 12,
            _ => 32,
        }
    }
    pub fn fourcc(&self) -> u32 {
        match self {
            Self::XRGB8888   => u32::from_le_bytes(*b"XR24"),
            Self::ARGB8888   => u32::from_le_bytes(*b"AR24"),
            Self::XBGR8888   => u32::from_le_bytes(*b"XB24"),
            Self::RGB565     => u32::from_le_bytes(*b"RG16"),
            Self::RGBA1010102=> u32::from_le_bytes(*b"AB30"),
            Self::NV12       => u32::from_le_bytes(*b"NV12"),
            Self::YUV420     => u32::from_le_bytes(*b"YU12"),
        }
    }
}

// ─── Display Mode ────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct DisplayMode {
    pub name:          [u8; 32],
    pub clock_khz:     u32,
    pub hdisplay:      u16,
    pub hsync_start:   u16,
    pub hsync_end:     u16,
    pub htotal:        u16,
    pub vdisplay:      u16,
    pub vsync_start:   u16,
    pub vsync_end:     u16,
    pub vtotal:        u16,
    pub flags:         u32,
}

impl DisplayMode {
    pub fn refresh_hz(&self) -> u32 {
        if self.htotal == 0 || self.vtotal == 0 { return 0; }
        self.clock_khz * 1000 / (self.htotal as u32 * self.vtotal as u32)
    }
    pub fn common_1080p() -> Self {
        let mut m = DisplayMode::zeroed();
        m.clock_khz = 148500;
        m.hdisplay = 1920; m.hsync_start = 2008; m.hsync_end = 2052; m.htotal = 2200;
        m.vdisplay = 1080; m.vsync_start = 1084; m.vsync_end = 1089; m.vtotal = 1125;
        m.name[..10].copy_from_slice(b"1920x1080p");
        m
    }
    pub fn common_1440p() -> Self {
        let mut m = DisplayMode::zeroed();
        m.clock_khz = 241500;
        m.hdisplay = 2560; m.hsync_start = 2608; m.hsync_end = 2640; m.htotal = 2720;
        m.vdisplay = 1440; m.vsync_start = 1443; m.vsync_end = 1448; m.vtotal = 1481;
        m.name[..10].copy_from_slice(b"2560x1440p");
        m
    }
    pub fn common_4k() -> Self {
        let mut m = DisplayMode::zeroed();
        m.clock_khz = 533250;
        m.hdisplay = 3840; m.hsync_start = 3888; m.hsync_end = 3920; m.htotal = 4000;
        m.vdisplay = 2160; m.vsync_start = 2163; m.vsync_end = 2168; m.vtotal = 2222;
        m.name[..8].copy_from_slice(b"3840x216");
        m
    }
    fn zeroed() -> Self {
        DisplayMode {
            name: [0u8; 32], clock_khz: 0,
            hdisplay: 0, hsync_start: 0, hsync_end: 0, htotal: 0,
            vdisplay: 0, vsync_start: 0, vsync_end: 0, vtotal: 0, flags: 0,
        }
    }
}

// ─── Connector ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ConnectorType { HDMI, DisplayPort, VGA, LVDS, EDP, DSI }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ConnectorStatus { Connected, Disconnected, Unknown }

#[derive(Clone, Copy)]
pub struct DrmConnector {
    pub id:       u32,
    pub ctype:    ConnectorType,
    pub status:   ConnectorStatus,
    pub modes:    [DisplayMode; 8],
    pub n_modes:  u8,
    pub cur_mode: u8,
    pub crtc_id:  u32,
    pub edid:     [u8; 256],
    pub edid_len: u16,
}

impl DrmConnector {
    pub const fn new(id: u32, ctype: ConnectorType) -> Self {
        const EMPTY_MODE: DisplayMode = DisplayMode {
            name: [0u8;32], clock_khz:0, hdisplay:0, hsync_start:0,
            hsync_end:0, htotal:0, vdisplay:0, vsync_start:0, vsync_end:0, vtotal:0, flags:0,
        };
        DrmConnector {
            id, ctype, status: ConnectorStatus::Unknown,
            modes: [EMPTY_MODE; 8], n_modes: 0, cur_mode: 0,
            crtc_id: 0, edid: [0u8; 256], edid_len: 0,
        }
    }
    pub fn add_mode(&mut self, mode: DisplayMode) -> bool {
        if self.n_modes as usize >= 8 { return false; }
        self.modes[self.n_modes as usize] = mode;
        self.n_modes += 1;
        true
    }
    pub fn preferred_mode(&self) -> Option<&DisplayMode> {
        if self.n_modes == 0 { None } else { Some(&self.modes[0]) }
    }
}

// ─── CRTC (display pipeline) ─────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct DrmCrtc {
    pub id:       u32,
    pub mode:     DisplayMode,
    pub enabled:  bool,
    pub fb_id:    u32,
    pub x:        u32,
    pub y:        u32,
    pub gamma_lut: [u16; 256],
}

impl DrmCrtc {
    pub const fn new(id: u32) -> Self {
        DrmCrtc {
            id, mode: DisplayMode {
                name:[0u8;32], clock_khz:0, hdisplay:0, hsync_start:0,
                hsync_end:0, htotal:0, vdisplay:0, vsync_start:0, vsync_end:0, vtotal:0, flags:0,
            },
            enabled: false, fb_id: 0, x: 0, y: 0,
            gamma_lut: [0u16; 256],
        }
    }
    pub fn set_gamma_linear(&mut self) {
        for i in 0..256 { self.gamma_lut[i] = (i as u16) << 8; }
    }
}

// ─── GEM Buffer Object (memory-mapped GPU buffer) ────────────────────────────
#[derive(Clone, Copy)]
pub struct GemObject {
    pub handle: u32,
    pub size:   usize,
    pub phys:   u64,   // physical address of backing pages
    pub mmap_offset: u64,
    pub tiling: u32,   // 0=linear, 1=X-tiled, 2=Y-tiled
    pub in_use: bool,
}

impl GemObject {
    pub const fn empty() -> Self {
        GemObject { handle: 0, size: 0, phys: 0, mmap_offset: 0, tiling: 0, in_use: false }
    }
}

// ─── Framebuffer ─────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct DrmFramebuffer {
    pub id:     u32,
    pub width:  u32,
    pub height: u32,
    pub pitch:  u32,
    pub fmt:    PixelFormat,
    pub gem:    u32,   // GEM handle
    pub in_use: bool,
}

// ─── DRM Device ──────────────────────────────────────────────────────────────
pub struct DrmDevice {
    pub mmio_base:    u64,
    pub vendor:       u16,
    pub device:       u16,
    pub connectors:   [DrmConnector; DRM_MAX_CONNECTORS],
    pub n_connectors: usize,
    pub crtcs:        [DrmCrtc; DRM_MAX_CRTCS],
    pub n_crtcs:      usize,
    pub fbs:          [DrmFramebuffer; 64],
    pub n_fbs:        usize,
    pub gems:         [GemObject; DRM_MAX_GEM_OBJECTS],
    pub n_gems:       usize,
    pub vblank_count: u64,
    pub initialized:  bool,
}

#[derive(Debug, Clone, Copy)]
pub enum DrmError { NoDevice, NoConnector, NoCrtc, NoMemory, InvalidMode, AlreadySet }

impl DrmDevice {
    pub const fn new(mmio_base: u64, vendor: u16, device: u16) -> Self {
        const EMPTY_CONN: DrmConnector = DrmConnector::new(0, ConnectorType::HDMI);
        const EMPTY_CRTC: DrmCrtc = DrmCrtc::new(0);
        const EMPTY_FB: DrmFramebuffer = DrmFramebuffer {
            id:0, width:0, height:0, pitch:0, fmt: PixelFormat::XRGB8888, gem:0, in_use:false
        };
        const EMPTY_GEM: GemObject = GemObject::empty();
        DrmDevice {
            mmio_base, vendor, device,
            connectors: [EMPTY_CONN; DRM_MAX_CONNECTORS], n_connectors: 0,
            crtcs: [EMPTY_CRTC; DRM_MAX_CRTCS], n_crtcs: 0,
            fbs: [EMPTY_FB; 64], n_fbs: 0,
            gems: [EMPTY_GEM; DRM_MAX_GEM_OBJECTS], n_gems: 0,
            vblank_count: 0, initialized: false,
        }
    }

    pub fn init(&mut self) {
        // Add two CRTCs
        for i in 0..2 {
            let mut c = DrmCrtc::new(i as u32 + 1);
            c.set_gamma_linear();
            self.crtcs[i] = c;
        }
        self.n_crtcs = 2;
        // Add HDMI + DP connectors
        let mut hdmi = DrmConnector::new(1, ConnectorType::HDMI);
        hdmi.status = ConnectorStatus::Connected;
        hdmi.add_mode(DisplayMode::common_1080p());
        hdmi.add_mode(DisplayMode::common_1440p());
        hdmi.add_mode(DisplayMode::common_4k());
        hdmi.crtc_id = 1;
        self.connectors[0] = hdmi;
        let mut dp = DrmConnector::new(2, ConnectorType::DisplayPort);
        dp.status = ConnectorStatus::Connected;
        dp.add_mode(DisplayMode::common_1440p());
        dp.add_mode(DisplayMode::common_4k());
        dp.crtc_id = 2;
        self.connectors[1] = dp;
        self.n_connectors = 2;
        self.initialized = true;
    }

    pub fn alloc_gem(&mut self, size: usize, phys: u64) -> Option<u32> {
        if self.n_gems >= DRM_MAX_GEM_OBJECTS { return None; }
        let idx = self.n_gems;
        self.gems[idx] = GemObject {
            handle: (idx + 1) as u32, size, phys,
            mmap_offset: phys, tiling: 0, in_use: true,
        };
        self.n_gems += 1;
        Some(self.gems[idx].handle)
    }

    pub fn create_fb(&mut self, w: u32, h: u32, fmt: PixelFormat, gem: u32) -> Option<u32> {
        if self.n_fbs >= 64 { return None; }
        let pitch = w * (fmt.bpp() / 8);
        let id = (self.n_fbs + 1) as u32;
        self.fbs[self.n_fbs] = DrmFramebuffer { id, width: w, height: h, pitch, fmt, gem, in_use: true };
        self.n_fbs += 1;
        Some(id)
    }

    pub fn set_crtc(&mut self, crtc_id: u32, fb_id: u32, mode: DisplayMode) -> Result<(), DrmError> {
        let crtc = self.crtcs[..self.n_crtcs].iter_mut()
            .find(|c| c.id == crtc_id).ok_or(DrmError::NoCrtc)?;
        crtc.mode    = mode;
        crtc.fb_id   = fb_id;
        crtc.enabled = true;
        Ok(())
    }

    pub fn page_flip(&mut self, crtc_id: u32, fb_id: u32) -> Result<(), DrmError> {
        let crtc = self.crtcs[..self.n_crtcs].iter_mut()
            .find(|c| c.id == crtc_id).ok_or(DrmError::NoCrtc)?;
        if !crtc.enabled { return Err(DrmError::NoDevice); }
        crtc.fb_id = fb_id;
        self.vblank_count += 1;
        Ok(())
    }

    pub fn vblank_count(&self) -> u64 { self.vblank_count }

    /// Write pixel to linear framebuffer (for VGA fallback path)
    pub fn write_pixel(&self, fb_phys: u64, pitch: u32, x: u32, y: u32, rgba: u32) {
        let offset = (y * pitch + x * 4) as u64;
        unsafe { ((fb_phys + offset) as *mut u32).write_volatile(rgba); }
    }

    /// Fill rect with solid color (used by compositor)
    pub fn fill_rect(&self, fb_phys: u64, pitch: u32, x: u32, y: u32, w: u32, h: u32, rgba: u32) {
        for row in y..y+h {
            for col in x..x+w {
                self.write_pixel(fb_phys, pitch, col, row, rgba);
            }
        }
    }
}

// ─── VESA Framebuffer Fallback ────────────────────────────────────────────────
pub struct VesaFb {
    pub addr:   u64,
    pub width:  u32,
    pub height: u32,
    pub pitch:  u32,
    pub bpp:    u8,
}

impl VesaFb {
    pub fn from_multiboot(addr: u64, w: u32, h: u32, pitch: u32, bpp: u8) -> Self {
        VesaFb { addr, width: w, height: h, pitch, bpp }
    }
    pub fn put_pixel(&self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height { return; }
        let offset = y as u64 * self.pitch as u64 + x as u64 * (self.bpp as u64 / 8);
        let color: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        unsafe { ((self.addr + offset) as *mut u32).write_volatile(color); }
    }
    pub fn clear(&self, r: u8, g: u8, b: u8) {
        for y in 0..self.height { for x in 0..self.width { self.put_pixel(x, y, r, g, b); } }
    }
    pub fn draw_rect(&self, x0: u32, y0: u32, w: u32, h: u32, r: u8, g: u8, b: u8) {
        for y in y0..y0+h { for x in x0..x0+w { self.put_pixel(x, y, r, g, b); } }
    }
    pub fn draw_char_1bpp(&self, x0: u32, y0: u32, bitmap: &[u8; 8], fg: (u8,u8,u8), bg: (u8,u8,u8)) {
        for row in 0..8u32 {
            let byte = bitmap[row as usize];
            for col in 0..8u32 {
                let set = byte & (0x80 >> col) != 0;
                let (r,g,b) = if set { fg } else { bg };
                self.put_pixel(x0 + col, y0 + row, r, g, b);
            }
        }
    }
}
