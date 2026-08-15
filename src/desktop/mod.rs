// SigmaOS Desktop Module
pub mod zenith_compositor;
pub mod notification;
pub mod moksha;
pub mod pantheon;
pub mod granite_contractor;

pub use granite_contractor::{
    GraniteUiToolkit, ToastNotification, AccentColor,
    SwitchboardSettingsHub, SwitchboardPlug, SwitchboardCategory,
    ContractorService, ContractorAction,
    ScreenTimeParentalGovernor, TimeQuota,
};

pub use pantheon::{
    GalaWindowManager, GalaTransitionStyle, Wingpanel, WingpanelIndicator, PlankDock,
    PlankDockItem, SlingshotLauncher, SlingshotApp, SlingshotCategory, AppCenter,
    AppCenterProduct, PantheonGreeter,
};

pub use zenith_compositor::{
    DamageRegion, InputEvent, InputEventData, InputEventType, Output, Surface, SurfaceType,
    WindowGeometry, WindowState, ZenithCompositor, ZenithWindow,
};

pub use notification::{
    Notification, SimpleNotification, NotificationManager, SimpleNotificationManager,
    NotificationUrgency, NotificationError,
};
