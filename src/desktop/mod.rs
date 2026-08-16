// SigmaOS Desktop Module
pub mod zenith_compositor;
pub mod notification;

pub use zenith_compositor::{
    DamageRegion, InputEvent, InputEventData, InputEventType, Output, Surface, SurfaceType,
    WindowGeometry, WindowState, ZenithCompositor, ZenithWindow,
};

pub use notification::{
    Notification, SimpleNotification, NotificationManager, SimpleNotificationManager,
    NotificationUrgency, NotificationError,
};
