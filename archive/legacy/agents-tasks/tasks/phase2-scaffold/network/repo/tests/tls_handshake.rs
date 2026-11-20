//! TLS context creation and handshake stub tests.
//!
//! Validates the separation-of-mechanism-and-policy design: TlsContext
//! holds policy (cipher suite preferences, cert verification, SNI) while
//! TcpConnection holds mechanism (byte transport).

use sigmaos_network::{
    SocketAddr, TcpConnection, TlsContext, TlsHandshakeState, TlsVersion,
};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn make_conn() -> TcpConnection {
    TcpConnection::connect(
        SocketAddr::new("127.0.0.1", 0),
        SocketAddr::new("example.com", 443),
    )
    .expect("connect failed")
}

// ---------------------------------------------------------------------------
// TlsContext creation tests
// ---------------------------------------------------------------------------

#[test]
fn test_new_context_has_tls13_by_default() {
    // Arrange + Act
    let ctx = TlsContext::new("example.com");

    // Assert — secure default: TLS 1.3
    assert_eq!(ctx.version(), TlsVersion::Tls13);
}

#[test]
fn test_new_context_verifies_certs_by_default() {
    // Arrange + Act
    let ctx = TlsContext::new("example.com");

    // Assert — secure default: cert verification on
    assert!(ctx.verify_certs());
}

#[test]
fn test_context_initial_state() {
    // Arrange + Act
    let ctx = TlsContext::new("example.com");

    // Assert
    assert_eq!(ctx.state(), TlsHandshakeState::Initial);
}

#[test]
fn test_with_version_builder() {
    // Arrange
    let ctx = TlsContext::new("legacy.example.com")
        .with_version(TlsVersion::Tls12);

    // Act + Assert
    assert_eq!(ctx.version(), TlsVersion::Tls12);
}

#[test]
fn test_disable_cert_verification() {
    // Arrange — insecure, but required for self-signed dev environments
    let ctx = TlsContext::new("dev.local")
        .with_cert_verification(false);

    // Assert
    assert!(!ctx.verify_certs());
}

#[test]
fn test_new_no_sni() {
    // Arrange + Act — IP-address connections cannot carry SNI
    let ctx = TlsContext::new_no_sni();

    // Assert: still uses TLS 1.3 and verifies certs
    assert_eq!(ctx.version(), TlsVersion::Tls13);
    assert!(ctx.verify_certs());
}

// ---------------------------------------------------------------------------
// Handshake stub tests
// ---------------------------------------------------------------------------

#[test]
fn test_handshake_transitions_to_established() {
    // Arrange
    let mut ctx = TlsContext::new("example.com");
    let mut conn = make_conn();

    // Act
    ctx.handshake(&mut conn).expect("handshake failed");

    // Assert
    assert_eq!(ctx.state(), TlsHandshakeState::Established);
}

#[test]
fn test_handshake_idempotent_on_established() {
    // Arrange — call handshake twice (e.g. session resumption path)
    let mut ctx = TlsContext::new("example.com");
    let mut conn = make_conn();
    ctx.handshake(&mut conn).unwrap();

    // Act — second call (stub is idempotent)
    let result = ctx.handshake(&mut conn);

    // Assert
    assert!(result.is_ok());
    assert_eq!(ctx.state(), TlsHandshakeState::Established);
}

#[test]
fn test_handshake_with_tls12_policy() {
    // Arrange — policy override: TLS 1.2 for compatibility
    let mut ctx = TlsContext::new("legacy.example.com")
        .with_version(TlsVersion::Tls12);
    let mut conn = make_conn();

    // Act
    ctx.handshake(&mut conn).expect("TLS 1.2 handshake failed");

    // Assert — version policy is preserved after handshake
    assert_eq!(ctx.version(), TlsVersion::Tls12);
    assert_eq!(ctx.state(), TlsHandshakeState::Established);
}
