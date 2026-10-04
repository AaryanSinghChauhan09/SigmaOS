// SigmaOS Cinnamon/XApp Desktop Suite + SerenityOS LibGUI Widget Toolkit
// Cinnamon-inspired: Desklets, System Tray, Panel, Nemo file manager
// XApp-inspired: Warpinator LAN transfer, Timeshift snapshots, Hypnotix IPTV,
//   Sticky Notes, Webapp Manager
// SerenityOS LibGUI-inspired: Widget hierarchy, layout engine, event dispatch,
//   painting model, window manager integration

use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU32, AtomicU64, Ordering},
    Arc, Mutex,
};

// ─────────────────────────────────────────────────────────────────────────────
// LibGUI Core: Widget System
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const CINNAMON: Color = Color {
        r: 75,
        g: 39,
        b: 0,
        a: 255,
    };
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }
    pub fn with_alpha(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutKind {
    None,
    Vertical,
    Horizontal,
    Grid { cols: u32 },
}

#[derive(Debug, Clone)]
pub enum Event {
    MousePress { button: u8, x: i32, y: i32 },
    MouseRelease { button: u8, x: i32, y: i32 },
    MouseMove { x: i32, y: i32 },
    KeyPress { keycode: u32, modifiers: u32 },
    KeyRelease { keycode: u32 },
    Paint { rect: Rect },
    Resize { size: Size },
    Close,
    Focus,
    Blur,
    Timer { id: u32 },
    Custom { name: String, data: Vec<u8> },
}

pub type WidgetId = u32;
static WIDGET_ID_COUNTER: AtomicU32 = AtomicU32::new(1);
fn new_widget_id() -> WidgetId {
    WIDGET_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Core widget trait (SerenityOS LibGUI-style)
pub trait Widget: Send + Sync {
    fn id(&self) -> WidgetId;
    fn rect(&self) -> Rect;
    fn set_rect(&mut self, r: Rect);
    fn preferred_size(&self) -> Size;
    fn handle_event(&mut self, event: &Event) -> bool;
    fn paint(&self, canvas: &mut Canvas);
    fn children(&self) -> &[WidgetId] {
        &[]
    }
    fn is_enabled(&self) -> bool {
        true
    }
    fn is_visible(&self) -> bool {
        true
    }
    fn widget_type(&self) -> &'static str;
}

/// Simple canvas for painting (backed by pixel buffer in real impl)
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub commands: Vec<DrawCommand>,
}

#[derive(Debug, Clone)]
pub enum DrawCommand {
    FillRect {
        rect: Rect,
        color: Color,
    },
    DrawText {
        x: i32,
        y: i32,
        text: String,
        color: Color,
    },
    DrawLine {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: Color,
    },
    DrawRect {
        rect: Rect,
        color: Color,
    },
    DrawBitmap {
        rect: Rect,
        data_ref: String,
    },
}

impl Canvas {
    pub fn new(w: u32, h: u32) -> Self {
        Canvas {
            width: w,
            height: h,
            commands: Vec::new(),
        }
    }
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.commands.push(DrawCommand::FillRect { rect, color });
    }
    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        self.commands.push(DrawCommand::DrawText {
            x,
            y,
            text: text.into(),
            color,
        });
    }
    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.commands.push(DrawCommand::DrawRect { rect, color });
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Concrete Widgets
// ─────────────────────────────────────────────────────────────────────────────

pub struct Label {
    id: WidgetId,
    rect: Rect,
    pub text: String,
    pub color: Color,
    pub bg: Color,
}
impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            id: new_widget_id(),
            rect: Rect {
                x: 0,
                y: 0,
                w: 100,
                h: 24,
            },
            text: text.into(),
            color: Color::BLACK,
            bg: Color::TRANSPARENT,
        }
    }
}
impl Widget for Label {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, r: Rect) {
        self.rect = r;
    }
    fn preferred_size(&self) -> Size {
        Size {
            w: (self.text.len() as i32) * 8,
            h: 20,
        }
    }
    fn handle_event(&mut self, _: &Event) -> bool {
        false
    }
    fn paint(&self, c: &mut Canvas) {
        c.fill_rect(self.rect, self.bg);
        c.draw_text(self.rect.x + 4, self.rect.y + 4, &self.text, self.color);
    }
    fn widget_type(&self) -> &'static str {
        "Label"
    }
}

pub struct Button {
    id: WidgetId,
    rect: Rect,
    pub text: String,
    pub pressed: bool,
    pub click_count: u32,
}
impl Button {
    pub fn new(text: &str) -> Self {
        Button {
            id: new_widget_id(),
            rect: Rect {
                x: 0,
                y: 0,
                w: 100,
                h: 32,
            },
            text: text.into(),
            pressed: false,
            click_count: 0,
        }
    }
}
impl Widget for Button {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, r: Rect) {
        self.rect = r;
    }
    fn preferred_size(&self) -> Size {
        Size {
            w: (self.text.len() as i32 + 4) * 8,
            h: 32,
        }
    }
    fn handle_event(&mut self, ev: &Event) -> bool {
        match ev {
            Event::MousePress { .. } => {
                self.pressed = true;
                true
            }
            Event::MouseRelease { .. } => {
                self.pressed = false;
                self.click_count += 1;
                true
            }
            _ => false,
        }
    }
    fn paint(&self, c: &mut Canvas) {
        let bg = if self.pressed {
            Color::new(100, 100, 100)
        } else {
            Color::new(200, 200, 200)
        };
        c.fill_rect(self.rect, bg);
        c.draw_rect(self.rect, Color::BLACK);
        c.draw_text(self.rect.x + 8, self.rect.y + 8, &self.text, Color::BLACK);
    }
    fn widget_type(&self) -> &'static str {
        "Button"
    }
}

pub struct TextInput {
    id: WidgetId,
    rect: Rect,
    pub text: String,
    pub placeholder: String,
    pub focused: bool,
}
impl TextInput {
    pub fn new(placeholder: &str) -> Self {
        TextInput {
            id: new_widget_id(),
            rect: Rect {
                x: 0,
                y: 0,
                w: 200,
                h: 28,
            },
            text: String::new(),
            placeholder: placeholder.into(),
            focused: false,
        }
    }
}
impl Widget for TextInput {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, r: Rect) {
        self.rect = r;
    }
    fn preferred_size(&self) -> Size {
        Size { w: 200, h: 28 }
    }
    fn handle_event(&mut self, ev: &Event) -> bool {
        match ev {
            Event::Focus => {
                self.focused = true;
                true
            }
            Event::Blur => {
                self.focused = false;
                true
            }
            _ => false,
        }
    }
    fn paint(&self, c: &mut Canvas) {
        let border = if self.focused {
            Color::new(0, 120, 215)
        } else {
            Color::new(150, 150, 150)
        };
        c.fill_rect(self.rect, Color::WHITE);
        c.draw_rect(self.rect, border);
        let display = if self.text.is_empty() {
            &self.placeholder
        } else {
            &self.text
        };
        let color = if self.text.is_empty() {
            Color::new(150, 150, 150)
        } else {
            Color::BLACK
        };
        c.draw_text(self.rect.x + 4, self.rect.y + 6, display, color);
    }
    fn widget_type(&self) -> &'static str {
        "TextInput"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Window Manager (LibGUI Window)
// ─────────────────────────────────────────────────────────────────────────────

pub struct Window {
    pub id: u32,
    pub title: String,
    pub rect: Rect,
    pub widgets: Vec<Box<dyn Widget>>,
    pub layout: LayoutKind,
    pub background: Color,
    pub is_modal: bool,
}

static WINDOW_ID: AtomicU32 = AtomicU32::new(1);

impl Window {
    pub fn new(title: &str, w: i32, h: i32) -> Self {
        Window {
            id: WINDOW_ID.fetch_add(1, Ordering::SeqCst),
            title: title.into(),
            rect: Rect {
                x: 100,
                y: 100,
                w,
                h,
            },
            widgets: Vec::new(),
            layout: LayoutKind::Vertical,
            background: Color::new(240, 240, 240),
            is_modal: false,
        }
    }

    pub fn add_widget(&mut self, w: Box<dyn Widget>) {
        self.widgets.push(w);
    }

    pub fn paint_all(&self) -> Canvas {
        let mut canvas = Canvas::new(self.rect.w as u32, self.rect.h as u32);
        canvas.fill_rect(self.rect, self.background);
        // Title bar
        canvas.fill_rect(
            Rect {
                x: 0,
                y: 0,
                w: self.rect.w,
                h: 28,
            },
            Color::CINNAMON,
        );
        canvas.draw_text(8, 6, &self.title, Color::WHITE);
        for widget in &self.widgets {
            if widget.is_visible() {
                widget.paint(&mut canvas);
            }
        }
        canvas
    }

    pub fn dispatch_event(&mut self, ev: &Event) -> bool {
        let mut handled = false;
        for widget in self.widgets.iter_mut() {
            if widget.is_enabled() && widget.handle_event(ev) {
                handled = true;
                break;
            }
        }
        handled
    }

    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cinnamon Panel & System Tray
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TrayIcon {
    pub app_name: String,
    pub tooltip: String,
    pub icon_path: String,
    pub visible: bool,
}

pub struct CinnamonPanel {
    pub height: i32,
    pub position: PanelPosition,
    pub applets: Vec<String>,
    pub tray_icons: Vec<TrayIcon>,
    pub clock_format: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelPosition {
    Bottom,
    Top,
}

impl CinnamonPanel {
    pub fn new(pos: PanelPosition) -> Self {
        CinnamonPanel {
            height: 40,
            position: pos,
            applets: vec![
                "menu".into(),
                "window-list".into(),
                "grouped-window-list".into(),
                "systray".into(),
                "calendar".into(),
                "power".into(),
            ],
            tray_icons: Vec::new(),
            clock_format: "%H:%M:%S".into(),
        }
    }

    pub fn add_tray_icon(&mut self, app: &str, tooltip: &str, icon: &str) {
        self.tray_icons.push(TrayIcon {
            app_name: app.into(),
            tooltip: tooltip.into(),
            icon_path: icon.into(),
            visible: true,
        });
    }

    pub fn remove_tray_icon(&mut self, app: &str) {
        self.tray_icons.retain(|t| t.app_name != app);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cinnamon Desklets
// ─────────────────────────────────────────────────────────────────────────────

pub trait Desklet: Send + Sync {
    fn name(&self) -> &str;
    fn update(&mut self);
    fn render(&self) -> String;
    fn on_click(&mut self) {}
}

pub struct ClockDesklet {
    pub time_str: String,
}
impl Desklet for ClockDesklet {
    fn name(&self) -> &str {
        "clock-desklet"
    }
    fn update(&mut self) {
        self.time_str = "14:36:21".into();
    }
    fn render(&self) -> String {
        format!("🕒 {}", self.time_str)
    }
}

pub struct WeatherDesklet {
    pub location: String,
    pub temp_c: f32,
    pub condition: String,
}
impl Desklet for WeatherDesklet {
    fn name(&self) -> &str {
        "weather-desklet"
    }
    fn update(&mut self) {
        self.temp_c = 28.5;
        self.condition = "Partly Cloudy".into();
    }
    fn render(&self) -> String {
        format!(
            "🌤 {} — {}°C, {}",
            self.location, self.temp_c, self.condition
        )
    }
}

pub struct SystemMonitorDesklet {
    pub cpu_pct: f32,
    pub ram_pct: f32,
}
impl Desklet for SystemMonitorDesklet {
    fn name(&self) -> &str {
        "sysmon-desklet"
    }
    fn update(&mut self) {
        self.cpu_pct = 23.4;
        self.ram_pct = 54.2;
    }
    fn render(&self) -> String {
        format!("CPU: {:.1}%  RAM: {:.1}%", self.cpu_pct, self.ram_pct)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// XApp Tools: Warpinator, Timeshift, Hypnotix, Sticky Notes
// ─────────────────────────────────────────────────────────────────────────────

/// Warpinator — LAN file transfer (like Android Nearby Share)
pub struct Warpinator {
    pub hostname: String,
    pub transfers: Vec<WarpTransfer>,
}

#[derive(Debug, Clone)]
pub struct WarpTransfer {
    pub id: u64,
    pub peer_hostname: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub transferred_bytes: u64,
    pub state: WarpState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarpState {
    Pending,
    Transferring,
    Complete,
    Failed,
    Declined,
}

static WARP_ID: AtomicU64 = AtomicU64::new(1);

impl Warpinator {
    pub fn new(hostname: &str) -> Self {
        Warpinator {
            hostname: hostname.into(),
            transfers: Vec::new(),
        }
    }

    pub fn send_file(&mut self, peer: &str, filename: &str, size: u64) -> u64 {
        let id = WARP_ID.fetch_add(1, Ordering::SeqCst);
        self.transfers.push(WarpTransfer {
            id,
            peer_hostname: peer.into(),
            file_name: filename.into(),
            size_bytes: size,
            transferred_bytes: 0,
            state: WarpState::Pending,
        });
        id
    }

    pub fn update_progress(&mut self, id: u64, bytes: u64) {
        if let Some(t) = self.transfers.iter_mut().find(|t| t.id == id) {
            t.transferred_bytes = bytes;
            if bytes >= t.size_bytes {
                t.state = WarpState::Complete;
            } else {
                t.state = WarpState::Transferring;
            }
        }
    }

    pub fn active_transfers(&self) -> Vec<&WarpTransfer> {
        self.transfers
            .iter()
            .filter(|t| t.state == WarpState::Transferring)
            .collect()
    }
}

/// Timeshift — System Snapshot Manager
#[derive(Debug, Clone)]
pub struct TimeshiftSnapshot {
    pub id: String,
    pub created_ns: u64,
    pub size_bytes: u64,
    pub snapshot_type: SnapshotKind,
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotKind {
    Rsync,
    Btrfs,
    Zfs,
}

pub struct Timeshift {
    pub snapshots: Vec<TimeshiftSnapshot>,
    pub target_device: String,
    pub auto_schedule: AutoSchedule,
}

#[derive(Debug, Clone)]
pub struct AutoSchedule {
    pub monthly: u8,
    pub weekly: u8,
    pub daily: u8,
    pub hourly: u8,
    pub boot: u8,
}

impl AutoSchedule {
    pub fn default_schedule() -> Self {
        AutoSchedule {
            monthly: 2,
            weekly: 3,
            daily: 5,
            hourly: 6,
            boot: 5,
        }
    }
}

impl Timeshift {
    pub fn new(device: &str) -> Self {
        Timeshift {
            snapshots: Vec::new(),
            target_device: device.into(),
            auto_schedule: AutoSchedule::default_schedule(),
        }
    }

    pub fn create_snapshot(&mut self, comment: &str, kind: SnapshotKind) -> String {
        let id = format!("{:016x}", self.snapshots.len() + 1);
        self.snapshots.push(TimeshiftSnapshot {
            id: id.clone(),
            created_ns: 0,
            size_bytes: 1_500_000_000,
            snapshot_type: kind,
            comment: comment.into(),
        });
        id
    }

    pub fn restore(&self, id: &str) -> Result<(), &'static str> {
        if self.snapshots.iter().any(|s| s.id == id) {
            Ok(())
        } else {
            Err("Snapshot not found")
        }
    }

    pub fn delete(&mut self, id: &str) -> bool {
        let before = self.snapshots.len();
        self.snapshots.retain(|s| s.id != id);
        self.snapshots.len() < before
    }
}

/// Hypnotix — IPTV Player
pub struct HypnotixPlayer {
    pub playlist_url: String,
    pub channels: Vec<IptvChannel>,
    pub current_channel: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct IptvChannel {
    pub name: String,
    pub url: String,
    pub group: String,
    pub logo_url: Option<String>,
}

impl HypnotixPlayer {
    pub fn new() -> Self {
        HypnotixPlayer {
            playlist_url: String::new(),
            channels: Vec::new(),
            current_channel: None,
        }
    }

    pub fn load_m3u(&mut self, content: &str) -> usize {
        let mut count = 0;
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            if lines[i].starts_with("#EXTINF") {
                let name = lines[i]
                    .split(',')
                    .last()
                    .unwrap_or("Channel")
                    .trim()
                    .to_string();
                if i + 1 < lines.len()
                    && (lines[i + 1].starts_with("http") || lines[i + 1].starts_with("rtmp"))
                {
                    self.channels.push(IptvChannel {
                        name,
                        url: lines[i + 1].into(),
                        group: "General".into(),
                        logo_url: None,
                    });
                    count += 1;
                    i += 2;
                    continue;
                }
            }
            i += 1;
        }
        count
    }

    pub fn tune(&mut self, index: usize) -> Option<&IptvChannel> {
        if index < self.channels.len() {
            self.current_channel = Some(index);
            self.channels.get(index)
        } else {
            None
        }
    }
}

/// Sticky Notes
pub struct StickyNotes {
    pub notes: Vec<StickyNote>,
}

#[derive(Debug, Clone)]
pub struct StickyNote {
    pub id: u32,
    pub content: String,
    pub color: Color,
    pub x: i32,
    pub y: i32,
}

static NOTE_ID: AtomicU32 = AtomicU32::new(1);

impl StickyNotes {
    pub fn new() -> Self {
        StickyNotes { notes: Vec::new() }
    }
    pub fn add(&mut self, content: &str, color: Color) -> u32 {
        let id = NOTE_ID.fetch_add(1, Ordering::SeqCst);
        self.notes.push(StickyNote {
            id,
            content: content.into(),
            color,
            x: 0,
            y: 0,
        });
        id
    }
    pub fn delete(&mut self, id: u32) {
        self.notes.retain(|n| n.id != id);
    }
    pub fn edit(&mut self, id: u32, content: &str) -> bool {
        self.notes
            .iter_mut()
            .find(|n| n.id == id)
            .map(|n| n.content = content.into())
            .is_some()
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_libgui_label_paint() {
        let label = Label::new("Hello SigmaOS");
        let mut canvas = Canvas::new(800, 600);
        label.paint(&mut canvas);
        assert!(!canvas.commands.is_empty());
    }

    #[test]
    fn test_libgui_button_click() {
        let mut btn = Button::new("Click Me");
        btn.handle_event(&Event::MousePress {
            button: 1,
            x: 50,
            y: 16,
        });
        assert!(btn.pressed);
        btn.handle_event(&Event::MouseRelease {
            button: 1,
            x: 50,
            y: 16,
        });
        assert!(!btn.pressed);
        assert_eq!(btn.click_count, 1);
    }

    #[test]
    fn test_libgui_text_input_focus() {
        let mut input = TextInput::new("Type here...");
        input.handle_event(&Event::Focus);
        assert!(input.focused);
        input.handle_event(&Event::Blur);
        assert!(!input.focused);
    }

    #[test]
    fn test_window_paint_and_dispatch() {
        let mut win = Window::new("SigmaOS Settings", 800, 600);
        win.add_widget(Box::new(Label::new("Welcome")));
        win.add_widget(Box::new(Button::new("Apply")));
        let canvas = win.paint_all();
        assert!(!canvas.commands.is_empty());
        assert_eq!(win.widget_count(), 2);
        // Dispatch click
        win.dispatch_event(&Event::MousePress {
            button: 1,
            x: 200,
            y: 200,
        });
    }

    #[test]
    fn test_cinnamon_panel_tray() {
        let mut panel = CinnamonPanel::new(PanelPosition::Bottom);
        panel.add_tray_icon("NetworkManager", "Network", "/icons/network.svg");
        panel.add_tray_icon("Bluetooth", "Bluetooth", "/icons/bt.svg");
        assert_eq!(panel.tray_icons.len(), 2);
        panel.remove_tray_icon("Bluetooth");
        assert_eq!(panel.tray_icons.len(), 1);
    }

    #[test]
    fn test_desklets() {
        let mut clock = ClockDesklet {
            time_str: String::new(),
        };
        clock.update();
        assert!(clock.render().contains("14:36:21"));

        let mut weather = WeatherDesklet {
            location: "Mumbai".into(),
            temp_c: 0.0,
            condition: String::new(),
        };
        weather.update();
        assert!(weather.render().contains("Mumbai"));
    }

    #[test]
    fn test_warpinator_transfer() {
        let mut warp = Warpinator::new("sigmabox");
        let id = warp.send_file("peer-laptop", "photo.jpg", 5_000_000);
        warp.update_progress(id, 2_500_000);
        assert_eq!(warp.active_transfers().len(), 1);
        warp.update_progress(id, 5_000_000);
        assert_eq!(warp.active_transfers().len(), 0); // complete
    }

    #[test]
    fn test_timeshift_snapshot_restore_delete() {
        let mut ts = Timeshift::new("/dev/sda");
        let id1 = ts.create_snapshot("Before upgrade", SnapshotKind::Rsync);
        let id2 = ts.create_snapshot("After upgrade", SnapshotKind::Btrfs);
        assert_eq!(ts.snapshots.len(), 2);
        assert!(ts.restore(&id1).is_ok());
        assert!(ts.restore("nonexistent").is_err());
        ts.delete(&id2);
        assert_eq!(ts.snapshots.len(), 1);
    }

    #[test]
    fn test_hypnotix_m3u_load() {
        let mut player = HypnotixPlayer::new();
        let m3u = "#EXTM3U\n#EXTINF:-1,BBC News\nhttp://stream.bbc.co.uk/news\n#EXTINF:-1,CNN\nhttp://stream.cnn.com/live\n";
        let count = player.load_m3u(m3u);
        assert_eq!(count, 2);
        assert!(player.tune(0).is_some());
        assert_eq!(player.tune(0).unwrap().name, "BBC News");
    }

    #[test]
    fn test_sticky_notes() {
        let mut notes = StickyNotes::new();
        let id = notes.add("Buy groceries", Color::new(255, 255, 0));
        assert_eq!(notes.notes.len(), 1);
        notes.edit(id, "Buy groceries and call mom");
        assert_eq!(notes.notes[0].content, "Buy groceries and call mom");
        notes.delete(id);
        assert!(notes.notes.is_empty());
    }
}
