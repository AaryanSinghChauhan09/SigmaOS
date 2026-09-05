// SPDX-License-Identifier: MIT
// SigmaOS Fedora & Red Hat-Inspired Data Commerce & Subscriptions Engine (`src/finance/data_commerce.rs`)
// Inspired by Red Hat Subscription Manager (RHSM), Red Hat Marketplace,
// Flathub Commercial Apps, Telemetry Metering, and Data Loss Prevention (DLP) Data Security.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};

// =========================================================================
// 1. RED HAT SUBSCRIPTION MANAGER (RHSM) ENTITLEMENT & LICENSING ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlaLevel {
    SelfSupport,
    Standard8x5,
    Premium24x7,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntitlementCertificate {
    pub cert_id: String,
    pub product_name: String,
    pub activation_key: String,
    pub sla: SlaLevel,
    pub quantity_pools: u32,
    pub expires_timestamp: u64,
}

pub struct RhsmEntitlementEngine {
    pub certificates: BTreeMap<String, EntitlementCertificate>, // cert_id -> cert
    pub active_key_mappings: BTreeMap<String, String>,            // activation_key -> cert_id
}

impl RhsmEntitlementEngine {
    pub fn new() -> Self {
        Self {
            certificates: BTreeMap::new(),
            active_key_mappings: BTreeMap::new(),
        }
    }

    pub fn register_subscription(
        &mut self,
        cert_id: &str,
        product_name: &str,
        activation_key: &str,
        sla: SlaLevel,
        pools: u32,
        expires_timestamp: u64,
    ) {
        let cert = EntitlementCertificate {
            cert_id: cert_id.to_string(),
            product_name: product_name.to_string(),
            activation_key: activation_key.to_string(),
            sla,
            quantity_pools: pools,
            expires_timestamp,
        };
        self.certificates.insert(cert_id.to_string(), cert);
        self.active_key_mappings
            .insert(activation_key.to_string(), cert_id.to_string());
    }

    pub fn attach_subscription_key(
        &self,
        activation_key: &str,
        current_time: u64,
    ) -> Result<&EntitlementCertificate, &'static str> {
        let cert_id = self
            .active_key_mappings
            .get(activation_key)
            .ok_or("RHSM: Activation key not registered")?;

        let cert = self
            .certificates
            .get(cert_id)
            .ok_or("RHSM: Entitlement certificate missing")?;

        if current_time >= cert.expires_timestamp {
            return Err("RHSM: Entitlement certificate has expired");
        }

        if cert.quantity_pools == 0 {
            return Err("RHSM: Subscription pool exhausted");
        }

        Ok(cert)
    }
}

impl Default for RhsmEntitlementEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. FEDORA & RED HAT MARKETPLACE CATALOG & PRICING ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommercialPricingModel {
    Free,
    FlatFeeUsd(u32),
    PayPerMbUsd(u32), // Micro-cents per MB
    SubscriptionMonthlyUsd(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataProductListing {
    pub product_id: String,
    pub title: String,
    pub publisher: String,
    pub pricing: CommercialPricingModel,
    pub total_sales_count: u64,
}

pub struct FedoraDataMarketplaceEngine {
    pub listings: BTreeMap<String, DataProductListing>,
    pub developer_revenue_share_pct: u32, // Default 85% developer share
}

impl FedoraDataMarketplaceEngine {
    pub fn new() -> Self {
        Self {
            listings: BTreeMap::new(),
            developer_revenue_share_pct: 85,
        }
    }

    pub fn add_listing(&mut self, product_id: &str, title: &str, publisher: &str, pricing: CommercialPricingModel) {
        self.listings.insert(
            product_id.to_string(),
            DataProductListing {
                product_id: product_id.to_string(),
                title: title.to_string(),
                publisher: publisher.to_string(),
                pricing,
                total_sales_count: 0,
            },
        );
    }

    pub fn purchase_product(&mut self, product_id: &str) -> Result<(u32, u32), &'static str> {
        let listing = self
            .listings
            .get_mut(product_id)
            .ok_or("Marketplace: Product listing not found")?;

        listing.total_sales_count += 1;

        let gross = match listing.pricing {
            CommercialPricingModel::Free => 0,
            CommercialPricingModel::FlatFeeUsd(price) => price,
            CommercialPricingModel::SubscriptionMonthlyUsd(price) => price,
            CommercialPricingModel::PayPerMbUsd(price) => price,
        };

        let dev_payout = (gross * self.developer_revenue_share_pct) / 100;
        let platform_fee = gross.saturating_sub(dev_payout);

        Ok((dev_payout, platform_fee))
    }
}

impl Default for FedoraDataMarketplaceEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. DATA CONSUMPTION METERING & TELEMETRY BILLING ENGINE
// =========================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageRecord {
    pub user_id: String,
    pub processed_mb: u64,
    pub api_calls: u64,
    pub execution_sec: u64,
}

pub struct DataCommerceTelemetryMeter {
    pub usage_by_user: BTreeMap<String, UsageRecord>,
}

impl DataCommerceTelemetryMeter {
    pub fn new() -> Self {
        Self {
            usage_by_user: BTreeMap::new(),
        }
    }

    pub fn record_usage(&mut self, user_id: &str, bytes: u64, api_calls: u64, exec_sec: u64) {
        let mb = bytes / (1024 * 1024);
        let entry = self
            .usage_by_user
            .entry(user_id.to_string())
            .or_insert_with(|| UsageRecord {
                user_id: user_id.to_string(),
                processed_mb: 0,
                api_calls: 0,
                execution_sec: 0,
            });

        entry.processed_mb += mb.max(1);
        entry.api_calls += api_calls;
        entry.execution_sec += exec_sec;
    }

    pub fn calculate_metered_bill_microcents(&self, user_id: &str, rate_per_mb: u64) -> u64 {
        if let Some(record) = self.usage_by_user.get(user_id) {
            record.processed_mb * rate_per_mb + record.api_calls * 10
        } else {
            0
        }
    }
}

impl Default for DataCommerceTelemetryMeter {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. DATA SECURITY & DLP CLASSIFICATION ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataClassificationTag {
    Public,
    Internal,
    Confidential,
    Restricted,
    PiiSensitive,
}

pub struct DataCommerceDlpEngine {
    pub classification_rules: BTreeMap<String, DataClassificationTag>, // Field -> Tag
}

impl DataCommerceDlpEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            classification_rules: BTreeMap::new(),
        };
        engine.register_default_dlp_rules();
        engine
    }

    fn register_default_dlp_rules(&mut self) {
        self.add_rule("ssn", DataClassificationTag::PiiSensitive);
        self.add_rule("credit_card", DataClassificationTag::Restricted);
        self.add_rule("financial_audit", DataClassificationTag::Confidential);
    }

    pub fn add_rule(&mut self, field_name: &str, tag: DataClassificationTag) {
        self.classification_rules
            .insert(field_name.to_string(), tag);
    }

    pub fn classify_field(&self, field_name: &str) -> DataClassificationTag {
        self.classification_rules
            .get(field_name)
            .copied()
            .unwrap_or(DataClassificationTag::Public)
    }

    pub fn mask_sensitive_data(&self, field_name: &str, value: &str) -> String {
        let tag = self.classify_field(field_name);
        match tag {
            DataClassificationTag::PiiSensitive | DataClassificationTag::Restricted => {
                let char_count = value.chars().count();
                if char_count <= 4 {
                    "****".to_string()
                } else {
                    let suffix: String = value.chars().skip(char_count - 4).collect();
                    format!("****{}", suffix)
                }
            }
            _ => value.to_string(),
        }
    }
}

impl Default for DataCommerceDlpEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_rhsm_entitlement_engine() {
        let mut rhsm = RhsmEntitlementEngine::new();
        rhsm.register_subscription(
            "cert_101",
            "SigmaOS Enterprise Server",
            "key_abc123",
            SlaLevel::Premium24x7,
            10,
            1800000000,
        );

        let cert = rhsm.attach_subscription_key("key_abc123", 1700000000).unwrap();
        assert_eq!(cert.product_name, "SigmaOS Enterprise Server");
        assert_eq!(cert.sla, SlaLevel::Premium24x7);

        // Expiration check
        assert!(rhsm.attach_subscription_key("key_abc123", 1900000000).is_err());
    }

    #[test]
    fn test_fedora_data_marketplace() {
        let mut marketplace = FedoraDataMarketplaceEngine::new();
        marketplace.add_listing(
            "prod_analytics",
            "Enterprise Financial Dataset",
            "Sigma Analytics Inc",
            CommercialPricingModel::FlatFeeUsd(100),
        );

        let (dev_payout, platform_fee) = marketplace.purchase_product("prod_analytics").unwrap();
        assert_eq!(dev_payout, 85);
        assert_eq!(platform_fee, 15);
        assert_eq!(marketplace.listings.get("prod_analytics").unwrap().total_sales_count, 1);
    }

    #[test]
    fn test_data_commerce_telemetry_meter() {
        let mut meter = DataCommerceTelemetryMeter::new();
        meter.record_usage("user_alpha", 50 * 1024 * 1024, 100, 300); // 50MB, 100 calls

        let bill = meter.calculate_metered_bill_microcents("user_alpha", 50); // 50 * 50 + 100 * 10
        assert_eq!(bill, 2500 + 1000);
    }

    #[test]
    fn test_data_commerce_dlp_engine() {
        let dlp = DataCommerceDlpEngine::new();
        assert_eq!(dlp.classify_field("ssn"), DataClassificationTag::PiiSensitive);

        let masked = dlp.mask_sensitive_data("ssn", "123-45-6789");
        assert_eq!(masked, "****6789");

        let public_val = dlp.mask_sensitive_data("user_name", "Jules");
        assert_eq!(public_val, "Jules");
    }
}
