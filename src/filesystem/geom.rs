extern crate alloc;
// SigmaOS FreeBSD GEOM-Inspired Storage Transformation Framework
// Modular storage class, provider, consumer, and volume transformation topology
// Inspired by FreeBSD's GEOM storage architecture (sys/geom)


use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomAccessRights {
    Read,
    Write,
    Exclusive,
}

#[derive(Debug, Clone)]
pub struct GeomProvider {
    pub name: String,
    pub sector_size: usize,
    pub total_sectors: u64,
    pub consumers_count: usize,
}

impl GeomProvider {
    pub fn new(name: &str, sector_size: usize, total_sectors: u64) -> Self {
        Self {
            name: name.to_string(),
            sector_size,
            total_sectors,
            consumers_count: 0,
        }
    }

    pub fn total_capacity_bytes(&self) -> u64 {
        self.total_sectors * self.sector_size as u64
    }
}

pub struct GeomConsumer {
    pub provider_name: String,
    pub rights: GeomAccessRights,
}

pub struct GeomClass {
    pub name: String,
    pub providers: BTreeMap<String, GeomProvider>,
    pub consumers: Vec<GeomConsumer>,
}

impl GeomClass {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            providers: BTreeMap::new(),
            consumers: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, provider: GeomProvider) {
        self.providers.insert(provider.name.clone(), provider);
    }

    pub fn attach_consumer(&mut self, provider_name: &str, rights: GeomAccessRights) -> Result<(), &'static str> {
        let provider = self.providers.get_mut(provider_name).ok_or("Provider not found")?;
        provider.consumers_count += 1;
        self.consumers.push(GeomConsumer {
            provider_name: provider_name.to_string(),
            rights,
        });
        Ok(())
    }

    /// Transforms a base provider into striped (RAID0) or mirror (RAID1) providers (GEOM_STRIPE/GEOM_MIRROR parity)
    pub fn create_transformed_stripe(&mut self, new_provider_name: &str, provider1: &str, provider2: &str) -> Result<GeomProvider, &'static str> {
        let p1 = self.providers.get(provider1).ok_or("Source provider 1 not found")?;
        let p2 = self.providers.get(provider2).ok_or("Source provider 2 not found")?;

        let min_sectors = p1.total_sectors.min(p2.total_sectors);
        let stripe_provider = GeomProvider::new(new_provider_name, p1.sector_size, min_sectors * 2);

        self.attach_consumer(provider1, GeomAccessRights::Exclusive)?;
        self.attach_consumer(provider2, GeomAccessRights::Exclusive)?;

        self.register_provider(stripe_provider.clone());
        Ok(stripe_provider)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geom_storage_topology() {
        let mut geom_stripe_class = GeomClass::new("STRIPE");

        let disk1 = GeomProvider::new("ada0", 512, 1000000);
        let disk2 = GeomProvider::new("ada1", 512, 1000000);

        geom_stripe_class.register_provider(disk1);
        geom_stripe_class.register_provider(disk2);

        let stripe = geom_stripe_class
            .create_transformed_stripe("stripe/stripe0", "ada0", "ada1")
            .unwrap();

        assert_eq!(stripe.name, "stripe/stripe0");
        assert_eq!(stripe.total_sectors, 2000000);
        assert_eq!(stripe.total_capacity_bytes(), 1024000000);
        assert_eq!(geom_stripe_class.providers.get("ada0").unwrap().consumers_count, 1);
    }
}
