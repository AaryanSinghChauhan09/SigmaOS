//! SigmaOS Compositor — Wayland server implementation.
//!
//! Provides the core Wayland protocol server, surface management, and output
//! management for the SigmaOS desktop layer.
//!
//! Upstream inspiration:
//! - wlroots: <https://gitlab.freedesktop.org/wlroots/wlroots>
//! - smithay: <https://github.com/Smithay/smithay>

pub mod buffer;
pub mod output;
pub mod surface;

use std::fmt;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Errors that can occur during compositor initialisation or operation.
#[derive(Debug)]
pub enum CompositorError {
    /// Failed to bind to the Wayland socket.
    SocketBind(String),
    /// A required backend (DRM, KMS, …) is unavailable.
    BackendUnavailable(String),
    /// An internal protocol violation was detected.
    ProtocolError(String),
}

impl fmt::Display for CompositorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompositorError::SocketBind(msg) => write!(f, "socket bind error: {msg}"),
            CompositorError::BackendUnavailable(msg) => {
                write!(f, "backend unavailable: {msg}")
            }
            CompositorError::ProtocolError(msg) => write!(f, "protocol error: {msg}"),
        }
    }
}

impl std::error::Error for CompositorError {}

// ---------------------------------------------------------------------------
// Client connection
// ---------------------------------------------------------------------------

/// Represents a single Wayland client connection.
#[derive(Debug, Clone)]
pub struct ClientConnection {
    /// Unique identifier assigned at connection time.
    pub id: u64,
    /// Human-readable label (e.g. app name) for diagnostics.
    pub label: String,
}

impl ClientConnection {
    /// Create a new client connection record.
    pub fn new(id: u64, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Surface manager
// ---------------------------------------------------------------------------

/// Tracks all wl_surface objects across all clients.
#[derive(Debug, Default)]
pub struct SurfaceManager {
    surfaces: Vec<u64>,
}

impl SurfaceManager {
    /// Register a new surface by its opaque ID.
    pub fn register(&mut self, surface_id: u64) {
        self.surfaces.push(surface_id);
    }

    /// Remove a surface when the client destroys it.
    pub fn unregister(&mut self, surface_id: u64) {
        self.surfaces.retain(|&id| id != surface_id);
    }

    /// Returns a slice of all currently tracked surface IDs.
    pub fn surfaces(&self) -> &[u64] {
        &self.surfaces
    }
}

// ---------------------------------------------------------------------------
// Output manager
// ---------------------------------------------------------------------------

/// Describes a single physical or virtual output (monitor).
#[derive(Debug, Clone)]
pub struct Output {
    /// Connector name, e.g. `"HDMI-A-1"`.
    pub name: String,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Refresh rate in milli-Hz (e.g. 60000 for 60 Hz).
    pub refresh_mhz: u32,
}

/// Manages the set of outputs known to the compositor.
#[derive(Debug, Default)]
pub struct OutputManager {
    outputs: Vec<Output>,
}

impl OutputManager {
    /// Add an output to the managed set.
    pub fn add(&mut self, output: Output) {
        self.outputs.push(output);
    }

    /// Remove an output by connector name.
    pub fn remove(&mut self, name: &str) {
        self.outputs.retain(|o| o.name != name);
    }

    /// Returns a slice of all active outputs.
    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }
}

// ---------------------------------------------------------------------------
// Wayland server
// ---------------------------------------------------------------------------

/// Top-level Wayland server state.
///
/// Holds references to all compositor subsystems. Constructed via
/// [`init_compositor`].
#[derive(Debug)]
pub struct WaylandServer {
    /// Surface lifecycle tracker.
    pub surface_manager: SurfaceManager,
    /// Output configuration tracker.
    pub output_manager: OutputManager,
    clients: Vec<ClientConnection>,
}

impl WaylandServer {
    fn new() -> Self {
        Self {
            surface_manager: SurfaceManager::default(),
            output_manager: OutputManager::default(),
            clients: Vec::new(),
        }
    }

    /// Accept a new client connection.
    pub fn accept_client(&mut self, conn: ClientConnection) {
        self.clients.push(conn);
    }

    /// Disconnect and remove a client by ID.
    pub fn disconnect_client(&mut self, id: u64) {
        self.clients.retain(|c| c.id != id);
    }

    /// Returns a slice of all active client connections.
    pub fn clients(&self) -> &[ClientConnection] {
        &self.clients
    }

    /// Run a single event-loop iteration (placeholder for real dispatch).
    pub fn dispatch(&mut self) -> Result<(), CompositorError> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Public entry-point
// ---------------------------------------------------------------------------

/// Initialise the Wayland compositor and return a ready [`WaylandServer`].
///
/// # Errors
///
/// Returns [`CompositorError`] if the compositor cannot be set up.
pub fn init_compositor() -> Result<WaylandServer, CompositorError> {
    let server = WaylandServer::new();
    Ok(server)
}

// ---------------------------------------------------------------------------
// Unit tests (inline)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compositor_initialises_without_error() {
        let result = init_compositor();
        assert!(result.is_ok(), "init_compositor must succeed in test env");
    }

    #[test]
    fn surface_manager_register_unregister() {
        let mut sm = SurfaceManager::default();
        sm.register(1);
        sm.register(2);
        assert_eq!(sm.surfaces().len(), 2);
        sm.unregister(1);
        assert_eq!(sm.surfaces(), &[2u64]);
    }

    #[test]
    fn output_manager_add_remove() {
        let mut om = OutputManager::default();
        om.add(Output {
            name: "HDMI-A-1".into(),
            width: 1920,
            height: 1080,
            refresh_mhz: 60_000,
        });
        assert_eq!(om.outputs().len(), 1);
        om.remove("HDMI-A-1");
        assert!(om.outputs().is_empty());
    }
}
