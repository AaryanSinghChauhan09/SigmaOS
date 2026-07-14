use sigma_net::{TcpStack, OnionRouter, SocketState};

#[test]
fn test_tcp_stack_bind_and_listen() {
    let mut stack = TcpStack::new();
    let id = stack.bind([127, 0, 0, 1], 8080);
    assert_eq!(id, 1);

    let sock = stack.get_socket(id).unwrap();
    assert_eq!(sock.local.ip, [127, 0, 0, 1]);
    assert_eq!(sock.local.port, 8080);
    assert_eq!(sock.state, SocketState::Listening);
}

#[test]
fn test_onion_router_circuit_creation() {
    let mut router = OnionRouter::new();
    
    let circuit = router.create_circuit(3).unwrap();
    assert_eq!(circuit.id, 1);
    assert_eq!(circuit.hops, 3);
    assert_eq!(router.circuits.len(), 1);

    assert!(router.create_circuit(0).is_err());
    assert!(router.create_circuit(9).is_err());
}
