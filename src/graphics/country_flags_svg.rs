extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use crate::klib::BTreeMap;

/// Render modes for ISO country flags and emblems
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagRenderMode {
    Rectangular4x3,  // Standard 4:3 ratio (640x480)
    Square1x1,       // Icon/Square 1:1 ratio (500x500)
    Widescreen2x1,   // Banner/Widescreen 2:1 ratio (800x400)
    CircularBadge,   // Avatar/Circular badge with rounded clip path (500x500)
}

/// Global world regions for ISO country classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldRegion {
    Americas,
    Europe,
    AsiaPacific,
    Africa,
    MiddleEast,
    Global,
}

/// Metadata for ISO 3166-1 country flags
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountryFlagMetadata {
    pub alpha2: String,
    pub alpha3: String,
    pub numeric_code: u16,
    pub english_name: String,
    pub native_name: String,
    pub region: WorldRegion,
    pub capital: String,
    pub default_locale: String,
    pub primary_colors: Vec<String>,
}

/// Linux & BSD Distribution Emblems supported in the SVG Collection
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DistroEmblemKind {
    Debian,
    ArchLinux,
    Ubuntu,
    Fedora,
    Alpine,
    Gentoo,
    VoidLinux,
    FreeBSD,
    OpenBSD,
    NetBSD,
    DragonFlyBsd,
    SigmaOS,
}

/// Metadata for Linux & BSD distribution emblems
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistroEmblemMetadata {
    pub distro: DistroEmblemKind,
    pub name: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub slogan: String,
    pub release_tier: String,
}

/// Portage USE-flags & BSD pledge system flag metadata integration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemFeatureFlagMetadata {
    pub flag_name: String,
    pub origin_distro: String,
    pub description: String,
    pub icon_country_alpha2: String,
}

/// Sovereign ISO Country Flags SVG Collection & Linux/BSD Distro Flag Engine
#[derive(Debug, Clone)]
pub struct IsoCountryFlagsSvgCollection {
    countries: BTreeMap<String, CountryFlagMetadata>,
    distros: BTreeMap<String, DistroEmblemMetadata>,
    system_flags: BTreeMap<String, SystemFeatureFlagMetadata>,
}

impl IsoCountryFlagsSvgCollection {
    /// Initialize the ISO Country Flags SVG Collection with Linux & BSD inspirations
    pub fn new() -> Self {
        let mut collection = Self {
            countries: BTreeMap::new(),
            distros: BTreeMap::new(),
            system_flags: BTreeMap::new(),
        };

        collection.populate_default_countries();
        collection.populate_default_distros();
        collection.populate_default_system_flags();

        collection
    }

    /// Populates ISO 3166-1 country flags (24 major nations covering all global regions)
    fn populate_default_countries(&mut self) {
        let default_list = vec![
            CountryFlagMetadata {
                alpha2: "US".to_string(),
                alpha3: "USA".to_string(),
                numeric_code: 840,
                english_name: "United States of America".to_string(),
                native_name: "United States".to_string(),
                region: WorldRegion::Americas,
                capital: "Washington, D.C.".to_string(),
                default_locale: "en_US".to_string(),
                primary_colors: vec!["#B22234".to_string(), "#FFFFFF".to_string(), "#3C3B6E".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "IN".to_string(),
                alpha3: "IND".to_string(),
                numeric_code: 356,
                english_name: "India".to_string(),
                native_name: "Bharat".to_string(),
                region: WorldRegion::AsiaPacific,
                capital: "New Delhi".to_string(),
                default_locale: "en_IN".to_string(),
                primary_colors: vec!["#FF9933".to_string(), "#FFFFFF".to_string(), "#138808".to_string(), "#000080".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "GB".to_string(),
                alpha3: "GBR".to_string(),
                numeric_code: 826,
                english_name: "United Kingdom".to_string(),
                native_name: "United Kingdom".to_string(),
                region: WorldRegion::Europe,
                capital: "London".to_string(),
                default_locale: "en_GB".to_string(),
                primary_colors: vec!["#012169".to_string(), "#C8102E".to_string(), "#FFFFFF".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "DE".to_string(),
                alpha3: "DEU".to_string(),
                numeric_code: 276,
                english_name: "Germany".to_string(),
                native_name: "Deutschland".to_string(),
                region: WorldRegion::Europe,
                capital: "Berlin".to_string(),
                default_locale: "de_DE".to_string(),
                primary_colors: vec!["#000000".to_string(), "#DD0000".to_string(), "#FFCC00".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "FR".to_string(),
                alpha3: "FRA".to_string(),
                numeric_code: 250,
                english_name: "France".to_string(),
                native_name: "France".to_string(),
                region: WorldRegion::Europe,
                capital: "Paris".to_string(),
                default_locale: "fr_FR".to_string(),
                primary_colors: vec!["#0055A5".to_string(), "#FFFFFF".to_string(), "#EF4135".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "JP".to_string(),
                alpha3: "JPN".to_string(),
                numeric_code: 392,
                english_name: "Japan".to_string(),
                native_name: "Nihon".to_string(),
                region: WorldRegion::AsiaPacific,
                capital: "Tokyo".to_string(),
                default_locale: "ja_JP".to_string(),
                primary_colors: vec!["#FFFFFF".to_string(), "#BC002D".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "CA".to_string(),
                alpha3: "CAN".to_string(),
                numeric_code: 124,
                english_name: "Canada".to_string(),
                native_name: "Canada".to_string(),
                region: WorldRegion::Americas,
                capital: "Ottawa".to_string(),
                default_locale: "en_CA".to_string(),
                primary_colors: vec!["#FF0000".to_string(), "#FFFFFF".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "AU".to_string(),
                alpha3: "AUS".to_string(),
                numeric_code: 36,
                english_name: "Australia".to_string(),
                native_name: "Australia".to_string(),
                region: WorldRegion::AsiaPacific,
                capital: "Canberra".to_string(),
                default_locale: "en_AU".to_string(),
                primary_colors: vec!["#000085".to_string(), "#FFFFFF".to_string(), "#E00000".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "BR".to_string(),
                alpha3: "BRA".to_string(),
                numeric_code: 76,
                english_name: "Brazil".to_string(),
                native_name: "Brasil".to_string(),
                region: WorldRegion::Americas,
                capital: "Brasília".to_string(),
                default_locale: "pt_BR".to_string(),
                primary_colors: vec!["#009739".to_string(), "#FEDD00".to_string(), "#012169".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "CN".to_string(),
                alpha3: "CHN".to_string(),
                numeric_code: 156,
                english_name: "China".to_string(),
                native_name: "Zhongguo".to_string(),
                region: WorldRegion::AsiaPacific,
                capital: "Beijing".to_string(),
                default_locale: "zh_CN".to_string(),
                primary_colors: vec!["#EE1C25".to_string(), "#FFFF00".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "SE".to_string(),
                alpha3: "SWE".to_string(),
                numeric_code: 752,
                english_name: "Sweden".to_string(),
                native_name: "Sverige".to_string(),
                region: WorldRegion::Europe,
                capital: "Stockholm".to_string(),
                default_locale: "sv_SE".to_string(),
                primary_colors: vec!["#006AA7".to_string(), "#FECC00".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "CH".to_string(),
                alpha3: "CHE".to_string(),
                numeric_code: 756,
                english_name: "Switzerland".to_string(),
                native_name: "Schweiz".to_string(),
                region: WorldRegion::Europe,
                capital: "Bern".to_string(),
                default_locale: "de_CH".to_string(),
                primary_colors: vec!["#FF0000".to_string(), "#FFFFFF".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "IT".to_string(),
                alpha3: "ITA".to_string(),
                numeric_code: 380,
                english_name: "Italy".to_string(),
                native_name: "Italia".to_string(),
                region: WorldRegion::Europe,
                capital: "Rome".to_string(),
                default_locale: "it_IT".to_string(),
                primary_colors: vec!["#009246".to_string(), "#FFFFFF".to_string(), "#CE2B37".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "NL".to_string(),
                alpha3: "NLD".to_string(),
                numeric_code: 528,
                english_name: "Netherlands".to_string(),
                native_name: "Nederland".to_string(),
                region: WorldRegion::Europe,
                capital: "Amsterdam".to_string(),
                default_locale: "nl_NL".to_string(),
                primary_colors: vec!["#AE1C28".to_string(), "#FFFFFF".to_string(), "#21468B".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "ES".to_string(),
                alpha3: "ESP".to_string(),
                numeric_code: 724,
                english_name: "Spain".to_string(),
                native_name: "España".to_string(),
                region: WorldRegion::Europe,
                capital: "Madrid".to_string(),
                default_locale: "es_ES".to_string(),
                primary_colors: vec!["#AA1529".to_string(), "#F1BF00".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "FI".to_string(),
                alpha3: "FIN".to_string(),
                numeric_code: 246,
                english_name: "Finland".to_string(),
                native_name: "Suomi".to_string(),
                region: WorldRegion::Europe,
                capital: "Helsinki".to_string(),
                default_locale: "fi_FI".to_string(),
                primary_colors: vec!["#FFFFFF".to_string(), "#003580".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "NO".to_string(),
                alpha3: "NOR".to_string(),
                numeric_code: 578,
                english_name: "Norway".to_string(),
                native_name: "Norge".to_string(),
                region: WorldRegion::Europe,
                capital: "Oslo".to_string(),
                default_locale: "nb_NO".to_string(),
                primary_colors: vec!["#BA0C2F".to_string(), "#FFFFFF".to_string(), "#00205B".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "DK".to_string(),
                alpha3: "DNK".to_string(),
                numeric_code: 208,
                english_name: "Denmark".to_string(),
                native_name: "Danmark".to_string(),
                region: WorldRegion::Europe,
                capital: "Copenhagen".to_string(),
                default_locale: "da_DK".to_string(),
                primary_colors: vec!["#C8102E".to_string(), "#FFFFFF".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "IE".to_string(),
                alpha3: "IRL".to_string(),
                numeric_code: 372,
                english_name: "Ireland".to_string(),
                native_name: "Éire".to_string(),
                region: WorldRegion::Europe,
                capital: "Dublin".to_string(),
                default_locale: "en_IE".to_string(),
                primary_colors: vec!["#169B62".to_string(), "#FFFFFF".to_string(), "#FF883E".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "KR".to_string(),
                alpha3: "KOR".to_string(),
                numeric_code: 410,
                english_name: "South Korea".to_string(),
                native_name: "Daehan Minguk".to_string(),
                region: WorldRegion::AsiaPacific,
                capital: "Seoul".to_string(),
                default_locale: "ko_KR".to_string(),
                primary_colors: vec!["#FFFFFF".to_string(), "#CD2E3A".to_string(), "#0047A0".to_string(), "#000000".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "SG".to_string(),
                alpha3: "SGP".to_string(),
                numeric_code: 702,
                english_name: "Singapore".to_string(),
                native_name: "Singapore".to_string(),
                region: WorldRegion::AsiaPacific,
                capital: "Singapore".to_string(),
                default_locale: "en_SG".to_string(),
                primary_colors: vec!["#ED2939".to_string(), "#FFFFFF".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "ZA".to_string(),
                alpha3: "ZAF".to_string(),
                numeric_code: 710,
                english_name: "South Africa".to_string(),
                native_name: "South Africa".to_string(),
                region: WorldRegion::Africa,
                capital: "Pretoria".to_string(),
                default_locale: "en_ZA".to_string(),
                primary_colors: vec!["#007749".to_string(), "#E03C31".to_string(), "#001489".to_string(), "#FFB81C".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "MX".to_string(),
                alpha3: "MEX".to_string(),
                numeric_code: 484,
                english_name: "Mexico".to_string(),
                native_name: "México".to_string(),
                region: WorldRegion::Americas,
                capital: "Mexico City".to_string(),
                default_locale: "es_MX".to_string(),
                primary_colors: vec!["#006847".to_string(), "#FFFFFF".to_string(), "#CE1126".to_string()],
            },
            CountryFlagMetadata {
                alpha2: "AR".to_string(),
                alpha3: "ARG".to_string(),
                numeric_code: 32,
                english_name: "Argentina".to_string(),
                native_name: "Argentina".to_string(),
                region: WorldRegion::Americas,
                capital: "Buenos Aires".to_string(),
                default_locale: "es_AR".to_string(),
                primary_colors: vec!["#74ACDF".to_string(), "#FFFFFF".to_string(), "#F6B40E".to_string()],
            },
        ];

        for item in default_list {
            self.countries.insert(item.alpha2.clone(), item);
        }
    }

    /// Populates 12 Linux & BSD distribution emblems
    fn populate_default_distros(&mut self) {
        let distros = vec![
            DistroEmblemMetadata {
                distro: DistroEmblemKind::Debian,
                name: "Debian".to_string(),
                primary_color: "#D70A53".to_string(),
                secondary_color: "#FFFFFF".to_string(),
                slogan: "The Universal Operating System".to_string(),
                release_tier: "Stable".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::ArchLinux,
                name: "Arch Linux".to_string(),
                primary_color: "#1793D1".to_string(),
                secondary_color: "#333333".to_string(),
                slogan: "A simple, lightweight distribution".to_string(),
                release_tier: "Rolling".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::Ubuntu,
                name: "Ubuntu".to_string(),
                primary_color: "#E95420".to_string(),
                secondary_color: "#77216F".to_string(),
                slogan: "Linux for Human Beings".to_string(),
                release_tier: "LTS".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::Fedora,
                name: "Fedora".to_string(),
                primary_color: "#51A2DA".to_string(),
                secondary_color: "#294172".to_string(),
                slogan: "Freedom, Friends, Features, First".to_string(),
                release_tier: "Current".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::Alpine,
                name: "Alpine Linux".to_string(),
                primary_color: "#0D597F".to_string(),
                secondary_color: "#FFFFFF".to_string(),
                slogan: "Small, Simple, Secure".to_string(),
                release_tier: "Minimal".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::Gentoo,
                name: "Gentoo".to_string(),
                primary_color: "#54487A".to_string(),
                secondary_color: "#DDDDDD".to_string(),
                slogan: "Source-based meta-distribution".to_string(),
                release_tier: "Source".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::VoidLinux,
                name: "Void Linux".to_string(),
                primary_color: "#478061".to_string(),
                secondary_color: "#000000".to_string(),
                slogan: "Independent Linux Distribution".to_string(),
                release_tier: "Rolling".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::FreeBSD,
                name: "FreeBSD".to_string(),
                primary_color: "#AB2B28".to_string(),
                secondary_color: "#000000".to_string(),
                slogan: "The Power to Serve".to_string(),
                release_tier: "Production".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::OpenBSD,
                name: "OpenBSD".to_string(),
                primary_color: "#F2C200".to_string(),
                secondary_color: "#000000".to_string(),
                slogan: "Only two remote holes in 25+ years".to_string(),
                release_tier: "Hardened".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::NetBSD,
                name: "NetBSD".to_string(),
                primary_color: "#FF6600".to_string(),
                secondary_color: "#000000".to_string(),
                slogan: "Of course it runs NetBSD".to_string(),
                release_tier: "Portable".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::DragonFlyBsd,
                name: "DragonFly BSD".to_string(),
                primary_color: "#990000".to_string(),
                secondary_color: "#CCCCCC".to_string(),
                slogan: "HAMMER2 Distributed Filesystem".to_string(),
                release_tier: "Clustered".to_string(),
            },
            DistroEmblemMetadata {
                distro: DistroEmblemKind::SigmaOS,
                name: "SigmaOS Sovereign".to_string(),
                primary_color: "#00E5FF".to_string(),
                secondary_color: "#7C4DFF".to_string(),
                slogan: "Sovereign Desktop & Kernel Suite".to_string(),
                release_tier: "Apex".to_string(),
            },
        ];

        for d in distros {
            let key = format!("{:?}", d.distro);
            self.distros.insert(key, d);
        }
    }

    /// Populates Portage USE-flags and BSD pledge feature flag metadata
    fn populate_default_system_flags(&mut self) {
        let flags = vec![
            SystemFeatureFlagMetadata {
                flag_name: "wayland".to_string(),
                origin_distro: "Gentoo/Debian".to_string(),
                description: "Enable modern Wayland display compositor support".to_string(),
                icon_country_alpha2: "SE".to_string(),
            },
            SystemFeatureFlagMetadata {
                flag_name: "pqc".to_string(),
                origin_distro: "SigmaOS".to_string(),
                description: "Post-quantum cryptographic key exchange & VPN".to_string(),
                icon_country_alpha2: "IN".to_string(),
            },
            SystemFeatureFlagMetadata {
                flag_name: "pledge_stdio".to_string(),
                origin_distro: "OpenBSD".to_string(),
                description: "Restrict process syscall access to standard IO".to_string(),
                icon_country_alpha2: "CH".to_string(),
            },
            SystemFeatureFlagMetadata {
                flag_name: "geom_mirror".to_string(),
                origin_distro: "FreeBSD".to_string(),
                description: "GEOM storage class volume mirroring".to_string(),
                icon_country_alpha2: "US".to_string(),
            },
        ];

        for f in flags {
            self.system_flags.insert(f.flag_name.clone(), f);
        }
    }

    /// Returns total count of supported ISO country flags
    pub fn count(&self) -> usize {
        self.countries.len()
    }

    /// Returns list of all ISO alpha-2 codes in the collection
    pub fn list_alpha2(&self) -> Vec<String> {
        self.countries.keys().cloned().collect()
    }

    /// Lookup country metadata by ISO alpha-2 code
    pub fn get_by_alpha2(&self, alpha2: &str) -> Option<&CountryFlagMetadata> {
        self.countries.get(&alpha2.to_ascii_uppercase())
    }

    /// Lookup country metadata by ISO alpha-3 code
    pub fn get_by_alpha3(&self, alpha3: &str) -> Option<&CountryFlagMetadata> {
        let target = alpha3.to_ascii_uppercase();
        self.countries.values().find(|c| c.alpha3 == target)
    }

    /// Lookup country metadata from system locale string (e.g. "en_IN", "hi_IN", "ja_JP", "de_DE")
    pub fn lookup_by_locale(&self, locale_str: &str) -> Option<&CountryFlagMetadata> {
        let parts: Vec<&str> = locale_str.split(&['_', '.'][..]).collect();
        if parts.len() > 1 {
            let country_code = parts[1].to_ascii_uppercase();
            if let Some(meta) = self.get_by_alpha2(&country_code) {
                return Some(meta);
            }
        }
        // Fallback search by default locale match
        self.countries.values().find(|c| c.default_locale.eq_ignore_ascii_case(locale_str))
    }

    /// Search country flags by English or native name
    pub fn search_by_name(&self, query: &str) -> Vec<&CountryFlagMetadata> {
        let q = query.to_ascii_lowercase();
        self.countries
            .values()
            .filter(|c| c.english_name.to_ascii_lowercase().contains(&q) || c.native_name.to_ascii_lowercase().contains(&q))
            .collect()
    }

    /// Filter country flags by global world region
    pub fn filter_by_region(&self, region: WorldRegion) -> Vec<&CountryFlagMetadata> {
        self.countries.values().filter(|c| c.region == region).collect()
    }

    /// Register a new custom country flag into the SVG collection
    pub fn register_custom_flag(&mut self, metadata: CountryFlagMetadata) -> Result<(), &'static str> {
        if metadata.alpha2.len() != 2 {
            return Err("ISO alpha-2 code must be exactly 2 characters");
        }
        let code = metadata.alpha2.to_ascii_uppercase();
        self.countries.insert(code, metadata);
        Ok(())
    }

    /// Lookup distro emblem metadata
    pub fn get_distro_emblem(&self, distro: DistroEmblemKind) -> Option<&DistroEmblemMetadata> {
        let key = format!("{:?}", distro);
        self.distros.get(&key)
    }

    /// Associate a Portage/BSD system feature flag with metadata
    pub fn associate_system_flag(&mut self, meta: SystemFeatureFlagMetadata) {
        self.system_flags.insert(meta.flag_name.clone(), meta);
    }

    /// Retrieve metadata for a Portage/BSD system feature flag
    pub fn get_system_flag_icon(&self, flag_name: &str) -> Option<&SystemFeatureFlagMetadata> {
        self.system_flags.get(flag_name)
    }

    /// Generate clean, valid, high-performance SVG XML string for an ISO country flag
    pub fn generate_country_svg(&self, alpha2_or_alpha3: &str, mode: FlagRenderMode) -> Result<String, &'static str> {
        let meta = self
            .get_by_alpha2(alpha2_or_alpha3)
            .or_else(|| self.get_by_alpha3(alpha2_or_alpha3))
            .ok_or("Country code not found in SVG collection")?;

        let (width, height, view_box) = match mode {
            FlagRenderMode::Rectangular4x3 => (640, 480, "0 0 640 480"),
            FlagRenderMode::Square1x1 => (500, 500, "0 0 500 500"),
            FlagRenderMode::Widescreen2x1 => (800, 400, "0 0 800 400"),
            FlagRenderMode::CircularBadge => (500, 500, "0 0 500 500"),
        };

        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"{}\" id=\"flag-iso-{}\">\n",
            width, height, view_box, meta.alpha2
        ));

        if mode == FlagRenderMode::CircularBadge {
            svg.push_str("  <defs>\n");
            svg.push_str("    <clipPath id=\"badge-clip\">\n");
            svg.push_str("      <circle cx=\"250\" cy=\"250\" r=\"240\" />\n");
            svg.push_str("    </clipPath>\n");
            svg.push_str("  </defs>\n");
            svg.push_str("  <g clip-path=\"url(#badge-clip)\">\n");
        }

        // Generate flag geometry based on country palette & pattern type
        let c = &meta.primary_colors;
        match meta.alpha2.as_str() {
            "JP" => {
                let bg = c.first().map(|s| s.as_str()).unwrap_or("#FFFFFF");
                let sun = c.get(1).map(|s| s.as_str()).unwrap_or("#BC002D");
                svg.push_str(&format!("  <rect width=\"100%\" height=\"100%\" fill=\"{}\" />\n", bg));
                svg.push_str(&format!("  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n", width / 2, height / 2, height / 3, sun));
            }
            "FR" | "IT" | "IE" => {
                let w3 = width / 3;
                let c1 = c.first().map(|s| s.as_str()).unwrap_or("#0055A5");
                let c2 = c.get(1).map(|s| s.as_str()).unwrap_or("#FFFFFF");
                let c3 = c.get(2).map(|s| s.as_str()).unwrap_or("#EF4135");
                svg.push_str(&format!("  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", w3, height, c1));
                svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", w3, w3, height, c2));
                svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", w3 * 2, w3, height, c3));
            }
            "DE" | "NL" => {
                let h3 = height / 3;
                let c1 = c.first().map(|s| s.as_str()).unwrap_or("#000000");
                let c2 = c.get(1).map(|s| s.as_str()).unwrap_or("#DD0000");
                let c3 = c.get(2).map(|s| s.as_str()).unwrap_or("#FFCC00");
                svg.push_str(&format!("  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", width, h3, c1));
                svg.push_str(&format!("  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", h3, width, h3, c2));
                svg.push_str(&format!("  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", h3 * 2, width, h3, c3));
            }
            "IN" => {
                let h3 = height / 3;
                let saffron = c.first().map(|s| s.as_str()).unwrap_or("#FF9933");
                let white = c.get(1).map(|s| s.as_str()).unwrap_or("#FFFFFF");
                let green = c.get(2).map(|s| s.as_str()).unwrap_or("#138808");
                let navy = c.get(3).map(|s| s.as_str()).unwrap_or("#000080");

                svg.push_str(&format!("  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", width, h3, saffron));
                svg.push_str(&format!("  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", h3, width, h3, white));
                svg.push_str(&format!("  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", h3 * 2, width, h3, green));
                // Ashoka Chakra wheel emblem in center
                let cx = width / 2;
                let cy = height / 2;
                let r = h3 / 3;
                svg.push_str(&format!("  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"6\" />\n", cx, cy, r, navy));
                svg.push_str(&format!("  <circle cx=\"{}\" cy=\"{}\" r=\"6\" fill=\"{}\" />\n", cx, cy, navy));
            }
            "SE" | "FI" | "DK" | "NO" => {
                let bg = c.first().map(|s| s.as_str()).unwrap_or("#006AA7");
                let cross = c.get(1).map(|s| s.as_str()).unwrap_or("#FECC00");
                let bar_w = width / 8;
                let cross_x = width / 3;
                svg.push_str(&format!("  <rect width=\"100%\" height=\"100%\" fill=\"{}\" />\n", bg));
                svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", cross_x, bar_w, height, cross));
                svg.push_str(&format!("  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", (height - bar_w) / 2, width, bar_w, cross));
            }
            "CH" => {
                let bg = c.first().map(|s| s.as_str()).unwrap_or("#FF0000");
                let white = c.get(1).map(|s| s.as_str()).unwrap_or("#FFFFFF");
                let cx = width / 2;
                let cy = height / 2;
                svg.push_str(&format!("  <rect width=\"100%\" height=\"100%\" fill=\"{}\" />\n", bg));
                svg.push_str(&format!("  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", cx - 30, cy - 90, 60, 180, white));
                svg.push_str(&format!("  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n", cx - 90, cy - 30, 180, 60, white));
            }
            _ => {
                // Default multi-stripe / gradient fallback vector rendering
                let stripe_h = height / c.len().max(1) as u32;
                for (idx, color) in c.iter().enumerate() {
                    svg.push_str(&format!(
                        "  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                        idx as u32 * stripe_h, width, stripe_h, color
                    ));
                }
            }
        }

        if mode == FlagRenderMode::CircularBadge {
            svg.push_str("  </g>\n");
            svg.push_str("  <circle cx=\"250\" cy=\"250\" r=\"240\" fill=\"none\" stroke=\"rgba(255,255,255,0.4)\" stroke-width=\"8\" />\n");
        }

        svg.push_str("</svg>");
        Ok(svg)
    }

    /// Generate production vector SVG for Linux & BSD distribution emblems
    pub fn generate_distro_svg(&self, distro: DistroEmblemKind, width: u32, height: u32) -> String {
        let meta = self.get_distro_emblem(distro).cloned().unwrap_or(DistroEmblemMetadata {
            distro,
            name: "Generic Distro".to_string(),
            primary_color: "#00E5FF".to_string(),
            secondary_color: "#FFFFFF".to_string(),
            slogan: "Sovereign OS".to_string(),
            release_tier: "Release".to_string(),
        });

        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 500 500\" id=\"distro-emblem-{:?}\">\n",
            width, height, distro
        ));

        let p_color = &meta.primary_color;
        let s_color = &meta.secondary_color;

        match distro {
            DistroEmblemKind::Debian => {
                svg.push_str(&format!("  <rect width=\"500\" height=\"500\" rx=\"60\" fill=\"{}\" />\n", p_color));
                svg.push_str(&format!("  <path d=\"M 250 100 C 350 120 400 220 370 310 C 340 400 230 420 160 360 C 100 310 120 220 180 170\" fill=\"none\" stroke=\"{}\" stroke-width=\"28\" stroke-linecap=\"round\" />\n", s_color));
            }
            DistroEmblemKind::ArchLinux => {
                svg.push_str(&format!("  <rect width=\"500\" height=\"500\" rx=\"60\" fill=\"#1793D1\" />\n"));
                svg.push_str(&format!("  <path d=\"M 250 80 L 390 400 L 320 400 L 250 250 L 180 400 L 110 400 Z\" fill=\"{}\" />\n", s_color));
            }
            DistroEmblemKind::FreeBSD => {
                svg.push_str(&format!("  <rect width=\"500\" height=\"500\" rx=\"60\" fill=\"{}\" />\n", p_color));
                // Beastie devil horns SVG motif
                svg.push_str("  <path d=\"M 180 180 Q 140 100 200 120 Q 220 160 220 200 Z\" fill=\"#FFFFFF\" />\n");
                svg.push_str("  <path d=\"M 320 180 Q 360 100 300 120 Q 280 160 280 200 Z\" fill=\"#FFFFFF\" />\n");
                svg.push_str("  <circle cx=\"250\" cy=\"280\" r=\"100\" fill=\"#FFFFFF\" />\n");
            }
            DistroEmblemKind::OpenBSD => {
                svg.push_str(&format!("  <rect width=\"500\" height=\"500\" rx=\"60\" fill=\"{}\" />\n", p_color));
                // Puffy pufferfish circular motif
                svg.push_str("  <circle cx=\"250\" cy=\"250\" r=\"130\" fill=\"#000000\" />\n");
                svg.push_str("  <circle cx=\"210\" cy=\"220\" r=\"20\" fill=\"#FFFFFF\" />\n");
                svg.push_str("  <circle cx=\"290\" cy=\"220\" r=\"20\" fill=\"#FFFFFF\" />\n");
            }
            DistroEmblemKind::SigmaOS => {
                svg.push_str("  <defs>\n");
                svg.push_str("    <linearGradient id=\"sigmaGrad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\">\n");
                svg.push_str(&format!("      <stop offset=\"0%\" stop-color=\"{}\" />\n", p_color));
                svg.push_str(&format!("      <stop offset=\"100%\" stop-color=\"{}\" />\n", s_color));
                svg.push_str("    </linearGradient>\n");
                svg.push_str("  </defs>\n");
                svg.push_str("  <rect width=\"500\" height=\"500\" rx=\"100\" fill=\"url(#sigmaGrad)\" />\n");
                // Sovereign Greek Sigma Σ emblem motif
                svg.push_str("  <path d=\"M 150 140 L 350 140 L 240 250 L 350 360 L 150 360 L 150 310 L 270 310 L 190 250 L 270 190 L 150 190 Z\" fill=\"#FFFFFF\" />\n");
            }
            _ => {
                svg.push_str(&format!("  <rect width=\"500\" height=\"500\" rx=\"60\" fill=\"{}\" />\n", p_color));
                svg.push_str(&format!("  <circle cx=\"250\" cy=\"250\" r=\"120\" fill=\"{}\" />\n", s_color));
            }
        }

        svg.push_str(&format!("  <text x=\"250\" y=\"460\" font-family=\"sans-serif\" font-size=\"32\" font-weight=\"bold\" fill=\"#FFFFFF\" text-anchor=\"middle\">{}</text>\n", meta.name));
        svg.push_str("</svg>");
        svg
    }
}

impl Default for IsoCountryFlagsSvgCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_country_flags_svg_generation() {
        let collection = IsoCountryFlagsSvgCollection::new();

        assert!(collection.count() >= 24);
        assert!(collection.get_by_alpha2("US").is_some());
        assert!(collection.get_by_alpha2("IN").is_some());
        assert!(collection.get_by_alpha3("DEU").is_some());

        // Test SVG rendering for India (IN)
        let svg_in = collection.generate_country_svg("IN", FlagRenderMode::Rectangular4x3).unwrap();
        assert!(svg_in.contains("<svg"));
        assert!(svg_in.contains("id=\"flag-iso-IN\""));
        assert!(svg_in.contains("#FF9933")); // Saffron color
        assert!(svg_in.contains("#138808")); // Green color

        // Test SVG rendering for Japan (JP)
        let svg_jp = collection.generate_country_svg("JP", FlagRenderMode::Square1x1).unwrap();
        assert!(svg_jp.contains("id=\"flag-iso-JP\""));
        assert!(svg_jp.contains("<circle"));
    }

    #[test]
    fn test_locale_to_flag_lookup() {
        let collection = IsoCountryFlagsSvgCollection::new();

        let meta_in = collection.lookup_by_locale("en_IN").unwrap();
        assert_eq!(meta_in.alpha2, "IN");
        assert_eq!(meta_in.english_name, "India");

        let meta_jp = collection.lookup_by_locale("ja_JP.UTF-8").unwrap();
        assert_eq!(meta_jp.alpha2, "JP");

        let meta_de = collection.lookup_by_locale("de_DE").unwrap();
        assert_eq!(meta_de.alpha2, "DE");
    }

    #[test]
    fn test_distro_emblems_svg_rendering() {
        let collection = IsoCountryFlagsSvgCollection::new();

        let debian_svg = collection.generate_distro_svg(DistroEmblemKind::Debian, 256, 256);
        assert!(debian_svg.contains("Debian"));
        assert!(debian_svg.contains("#D70A53"));

        let sigma_svg = collection.generate_distro_svg(DistroEmblemKind::SigmaOS, 512, 512);
        assert!(sigma_svg.contains("SigmaOS Sovereign"));
        assert!(sigma_svg.contains("sigmaGrad"));
    }

    #[test]
    fn test_render_modes_aspect_ratios() {
        let collection = IsoCountryFlagsSvgCollection::new();

        let rect_svg = collection.generate_country_svg("US", FlagRenderMode::Rectangular4x3).unwrap();
        assert!(rect_svg.contains("width=\"640\""));

        let badge_svg = collection.generate_country_svg("US", FlagRenderMode::CircularBadge).unwrap();
        assert!(badge_svg.contains("badge-clip"));
        assert!(badge_svg.contains("clip-path"));
    }

    #[test]
    fn test_portage_bsd_flag_metadata_integration() {
        let mut collection = IsoCountryFlagsSvgCollection::new();

        let icon = collection.get_system_flag_icon("wayland").unwrap();
        assert_eq!(icon.icon_country_alpha2, "SE");

        collection.associate_system_flag(SystemFeatureFlagMetadata {
            flag_name: "chflags_simmut".to_string(),
            origin_distro: "FreeBSD".to_string(),
            description: "System immutable file attribute flag".to_string(),
            icon_country_alpha2: "GB".to_string(),
        });

        assert!(collection.get_system_flag_icon("chflags_simmut").is_some());
    }
}
