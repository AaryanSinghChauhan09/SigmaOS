// SigmaOS Desktop Module
pub mod zenith_compositor;
pub mod notification;

pub mod terminal;

pub use terminal::{
    ShellIntegration, SimpleShellIntegration, SimpleTerminal, SimpleTerminalManager, Terminal,
    TerminalError, TerminalID, TerminalManager,
};

pub use notification::{
    Notification, SimpleNotification, NotificationManager, SimpleNotificationManager, DoNotDisturb, SimpleDoNotDisturb,
    NotificationUrgency, NotificationError,
};
