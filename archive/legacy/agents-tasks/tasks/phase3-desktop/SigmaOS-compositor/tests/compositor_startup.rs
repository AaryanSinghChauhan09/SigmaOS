//! Integration tests — compositor startup.

use sigmaos_compositor::{init_compositor, ClientConnection};

#[test]
fn server_starts_with_no_clients() {
    let server = init_compositor().expect("compositor init failed");
    assert!(
        server.clients().is_empty(),
        "fresh server must have zero clients"
    );
}

#[test]
fn server_accepts_and_disconnects_client() {
    let mut server = init_compositor().expect("compositor init failed");
    let conn = ClientConnection::new(42, "test-client");
    server.accept_client(conn);
    assert_eq!(server.clients().len(), 1);
    server.disconnect_client(42);
    assert!(server.clients().is_empty());
}

#[test]
fn dispatch_returns_ok() {
    let mut server = init_compositor().expect("compositor init failed");
    assert!(server.dispatch().is_ok());
}
