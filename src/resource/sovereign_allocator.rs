extern crate alloc;
// Sovereign Multi-Resource Allocator Engine (Linux & BSD Inspired)
// Combines Linux cgroups v2 resource weight allocation with OpenBSD racct/rctl limits
// and FreeBSD GEOM storage provider resource budgeting.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllocatorResourceType {
    CpuShares,
    MemoryPages,
    IoBandwidthBytesPerSec,
    MaxProcessCount,
    NetworkSocketSlots,
}

#[derive(Debug, Clone)]
pub struct ResourceAllocationLimit {
    pub soft_limit: u64,
    pub hard_limit: u64,
    pub current_usage: u64,
}

#[derive(Debug, Clone)]
pub struct ResourceDomainGroup {
    pub domain_id: usize,
    pub name: String,
    pub parent_domain_id: Option<usize>,
    pub cpu_weight: u32,
    pub resource_limits: BTreeMap<AllocatorResourceType, ResourceAllocationLimit>,
}

impl ResourceDomainGroup {
    pub fn new(domain_id: usize, name: &str, cpu_weight: u32) -> Self {
        Self {
            domain_id,
            name: name.to_string(),
            parent_domain_id: None,
            cpu_weight,
            resource_limits: BTreeMap::new(),
        }
    }

    pub fn set_limit(&mut self, res_type: AllocatorResourceType, soft: u64, hard: u64) {
        self.resource_limits.insert(
            res_type,
            ResourceAllocationLimit {
                soft_limit: soft,
                hard_limit: hard,
                current_usage: 0,
            },
        );
    }

    pub fn request_allocation(&mut self, res_type: AllocatorResourceType, amount: u64) -> Result<u64, String> {
        let limit = self
            .resource_limits
            .get_mut(&res_type)
            .ok_or_else(|| format!("Resource type {:?} not configured for domain {}", res_type, self.name))?;

        if limit.current_usage + amount > limit.hard_limit {
            return Err(format!(
                "Hard limit exceeded for {:?} in domain {}: requested {}, currently {}, hard limit {}",
                res_type, self.name, amount, limit.current_usage, limit.hard_limit
            ));
        }

        limit.current_usage += amount;
        Ok(limit.current_usage)
    }

    pub fn release_allocation(&mut self, res_type: AllocatorResourceType, amount: u64) -> Result<u64, String> {
        let limit = self
            .resource_limits
            .get_mut(&res_type)
            .ok_or_else(|| format!("Resource type {:?} not configured for domain {}", res_type, self.name))?;

        if amount > limit.current_usage {
            limit.current_usage = 0;
        } else {
            limit.current_usage -= amount;
        }

        Ok(limit.current_usage)
    }
}

pub struct SovereignMultiResourceAllocator {
    pub domain_groups: BTreeMap<usize, ResourceDomainGroup>,
    next_domain_id: usize,
}

impl SovereignMultiResourceAllocator {
    pub fn new() -> Self {
        let mut allocator = Self {
            domain_groups: BTreeMap::new(),
            next_domain_id: 1,
        };

        // Initialize root domain group
        let mut root = ResourceDomainGroup::new(0, "root", 100);
        root.set_limit(AllocatorResourceType::CpuShares, 10000, 10000);
        root.set_limit(AllocatorResourceType::MemoryPages, 1048576, 2097152);
        root.set_limit(AllocatorResourceType::MaxProcessCount, 1024, 4096);
        allocator.domain_groups.insert(0, root);

        allocator
    }

    pub fn create_domain_group(&mut self, name: &str, cpu_weight: u32, parent_id: Option<usize>) -> usize {
        let domain_id = self.next_domain_id;
        self.next_domain_id += 1;

        let mut domain = ResourceDomainGroup::new(domain_id, name, cpu_weight);
        domain.parent_domain_id = parent_id;

        self.domain_groups.insert(domain_id, domain);
        domain_id
    }

    pub fn allocate_resource(&mut self, domain_id: usize, res_type: AllocatorResourceType, amount: u64) -> Result<u64, String> {
        let domain = self
            .domain_groups
            .get_mut(&domain_id)
            .ok_or_else(|| format!("Domain group {} not found", domain_id))?;

        domain.request_allocation(res_type, amount)
    }

    pub fn free_resource(&mut self, domain_id: usize, res_type: AllocatorResourceType, amount: u64) -> Result<u64, String> {
        let domain = self
            .domain_groups
            .get_mut(&domain_id)
            .ok_or_else(|| format!("Domain group {} not found", domain_id))?;

        domain.release_allocation(res_type, amount)
    }
}

impl Default for SovereignMultiResourceAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_multi_resource_allocator() {
        let mut allocator = SovereignMultiResourceAllocator::new();
        let user_domain = allocator.create_domain_group("user_workload", 50, Some(0));

        let domain = allocator.domain_groups.get_mut(&user_domain).unwrap();
        domain.set_limit(AllocatorResourceType::MemoryPages, 1000, 2000);

        assert_eq!(allocator.allocate_resource(user_domain, AllocatorResourceType::MemoryPages, 500).unwrap(), 500);
        assert_eq!(allocator.allocate_resource(user_domain, AllocatorResourceType::MemoryPages, 1000).unwrap(), 1500);
        assert!(allocator.allocate_resource(user_domain, AllocatorResourceType::MemoryPages, 1000).is_err());

        assert_eq!(allocator.free_resource(user_domain, AllocatorResourceType::MemoryPages, 500).unwrap(), 1000);
    }
}
