//! TCP connection stub tests — Arrange-Act-Assert pattern.
//!
//! These tests verify the public API surface of [`TcpConnection`] and
//! [`TcpStack`] without requiring a real network; the stubs always succeed,
//! giving deterministic, fast, isolated results (FIRST principles).

use sigmaos_network::{
    NetworkError, SocketAddr, TcpConnection, TcpStack, TcpStackConfig, TcpState,
};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn loopback(port: u16) -> SocketAddr {
    SocketAddr::new("127.0.0.1", port)
}

// ---------------------------------------------------------------------------
// TcpConnection tests
// ---------------------------------------------------------------------------

#[test]
fn test_connect_returns_established() {
    // Arrange
    let local = loopback(0);
    let remote = loopback(8080);

    // Act
    let conn = TcpConnection::connect(local, remote).expect("connect failed");

    // Assert
    assert_eq!(conn.state(), TcpState::Established);
}

#[test]
fn test_send_returns_byte_count() {
    // Arrange
    let mut conn =
        TcpConnection::connect(loopback(0), loopback(9000)).expect("connect failed");
    let payload = b"hello sigmaos";

    // Act
    let sent = conn.send(payload).expect("send failed");

    // Assert
    assert_eq!(sent, payload.len());
}

#[test]
fn test_recv_returns_injected_bytes() {
    // Arrange
    let mut conn =
        TcpConnection::connect(loopback(0), loopback(9001)).expect("connect failed");
    conn.inject_rx(b"response data");
    let mut buf = [0u8; 64];

    // Act
    let n = conn.recv(&mut buf).expect("recv failed");

    // Assert
    assert_eq!(n, 13);
    assert_eq!(&buf[..n], b"response data");
}

#[test]
fn test_recv_partial_buffer() {
    // Arrange — buffer smaller than injected data (zero-copy drain semantics)
    let mut conn =
        TcpConnection::connect(loopback(0), loopback(9002)).expect("connect failed");
    conn.inject_rx(b"abcdefghij");
    let mut buf = [0u8; 4];

    // Act
    let n = conn.recv(&mut buf).expect("recv failed");

    // Assert: only as many bytes as fit in buf
    assert_eq!(n, 4);
    assert_eq!(&buf[..n], b"abcd");
}

#[test]
fn test_send_after_close_returns_error() {
    // Arrange
    let mut conn =
        TcpConnection::connect(loopback(0), loopback(9003)).expect("connect failed");
    conn.close();

    // Act
    let result = conn.send(b"too late");

    // Assert
    assert!(matches!(result, Err(NetworkError::SendFailed(_))));
}

#[test]
fn test_close_transitions_to_closing() {
    // Arrange
    let mut conn =
        TcpConnection::connect(loopback(0), loopback(9004)).expect("connect failed");

    // Act
    conn.close();

    // Assert
    assert_eq!(conn.state(), TcpState::Closing);
}

// ---------------------------------------------------------------------------
// TcpStack tests
// ---------------------------------------------------------------------------

#[test]
fn test_stack_connect_increments_count() {
    // Arrange
    let mut stack = TcpStack::new(TcpStackConfig::default());

    // Act
    let idx = stack.connect(loopback(0), loopback(7000)).expect("stack connect failed");

    // Assert
    assert_eq!(idx, 0);
    assert_eq!(stack.connection_count(), 1);
}

#[test]
fn test_stack_connection_by_index() {
    // Arrange
    let mut stack = TcpStack::new(TcpStackConfig::default());
    stack.connect(loopback(0), loopback(7001)).unwrap();

    // Act
    let conn = stack.connection_mut(0);

    // Assert
    assert!(conn.is_some());
    assert_eq!(conn.unwrap().state(), TcpState::Established);
}

#[test]
fn test_stack_connection_table_full_error() {
    // Arrange — max 1 connection
    let cfg = TcpStackConfig {
        max_connections: 1,
        ..Default::default()
    };
    let mut stack = TcpStack::new(cfg);
    stack.connect(loopback(0), loopback(8000)).unwrap();

    // Act
    let result = stack.connect(loopback(0), loopback(8001));

    // Assert
    assert!(matches!(result, Err(NetworkError::Io(_))));
}
