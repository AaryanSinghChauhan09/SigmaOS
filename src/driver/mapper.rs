use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
// SigmaOS Legacy Driver API Mapper (DriverMapper)
// Maps legacy driver APIs directly to modern equivalents to bypass heavy emulation overhead

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapperCategory {
    Storage,
    Network,
    Graphics,
}

pub struct DriverMapper {
    pub category: MapperCategory,
    pub api_translations: HashMap<String, String>,
}

impl DriverMapper {
    pub fn new(cat: MapperCategory) -> Self {
        let mut translations = HashMap::new();
        match cat {
            MapperCategory::Storage => {
                translations.insert("ide_read_sector".to_string(), "nvme_read_block".to_string());
                translations.insert(
                    "ide_write_sector".to_string(),
                    "nvme_write_block".to_string(),
                );
            }
            MapperCategory::Network => {
                translations.insert(
                    "slip_tx_packet".to_string(),
                    "ethernet_tx_packet".to_string(),
                );
            }
            MapperCategory::Graphics => {
                translations.insert(
                    "vga_set_mode_13h".to_string(),
                    "vesa_set_linear_modebar".to_string(),
                );
            }
        }
        DriverMapper {
            category: cat,
            api_translations: translations,
        }
    }

    pub fn map_legacy_api(&self, legacy_api_name: &str) -> Option<&String> {
        self.api_translations.get(legacy_api_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_mapper_api_resolving() {
        let mapper = DriverMapper::new(MapperCategory::Storage);
        let mapped = mapper.map_legacy_api("ide_read_sector").unwrap();
        assert_eq!(mapped, "nvme_read_block");

        let missing = mapper.map_legacy_api("ide_format_track");
        assert!(missing.is_none());
    }
}
