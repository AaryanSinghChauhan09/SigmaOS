#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilitySet(pub u64);

impl CapabilitySet {
    pub const CAP_SYS_ADMIN: u64 = 1 << 0;
    pub const CAP_NET_ADMIN: u64 = 1 << 12;
    pub const CAP_KILL: u64 = 1 << 5;

    pub fn has_capability(&self, cap: u64) -> bool {
        (self.0 & cap) != 0
    }
}

pub struct NamespaceContext {
    pub pid_ns_id: u64,
    pub mount_ns_id: u64,
    pub net_ns_id: u64,
}

impl NamespaceContext {
    pub fn new(pid: u64, mount: u64, net: u64) -> Self {
        Self { pid_ns_id: pid, mount_ns_id: mount, net_ns_id: net }
    }

    pub fn can_cross_namespace_op(&self, target: &NamespaceContext, caps: CapabilitySet) -> bool {
        if self.pid_ns_id == target.pid_ns_id && self.net_ns_id == target.net_ns_id {
            return true;
        }
        caps.has_capability(CapabilitySet::CAP_SYS_ADMIN)
    }
}

#[test]
fn test_namespace_isolation_boundary() {
    let ns1 = NamespaceContext::new(1, 100, 1000);
    let ns2 = NamespaceContext::new(2, 200, 2000);

    let unprivileged_caps = CapabilitySet(0);
    let admin_caps = CapabilitySet(CapabilitySet::CAP_SYS_ADMIN);

    assert!(!ns1.can_cross_namespace_op(&ns2, unprivileged_caps));
    assert!(ns1.can_cross_namespace_op(&ns2, admin_caps));
}
