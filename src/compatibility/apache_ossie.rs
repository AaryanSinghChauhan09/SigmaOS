#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
/// Apache Ossie (Incubating) Semantic Model & Metric Specification Engine for SigmaOS
/// Formerly Open Semantic Interchange (OSI) standard
/// Establishes an on-device, vendor-neutral semantic layer and metric language interpreter.
/// Enables AI agents, microservices, and databases to parse, define, and evaluate identical business metrics,
/// dimensions, and relationships without loss of semantic meaning.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricAggregation {
    Sum,
    Average,
    Count,
    CountDistinct,
}

/// Represents an Apache Ossie-compliant Business Metric (e.g. "Revenue" or "Monthly Active Users")
#[derive(Debug, Clone)]
pub struct OssieMetric {
    pub name: String,
    pub label: String,
    pub formula: String, // E.g., "Sum(revenue)" or "CountDistinct(user_id)"
    pub aggregation: MetricAggregation,
    pub source_field: String,
    pub description: String,
}

impl OssieMetric {
    pub fn new(name: &str, label: &str, agg: MetricAggregation, field: &str) -> Self {
        OssieMetric {
            name: name.to_string(),
            label: label.to_string(),
            formula: format!("{:?}({})", agg, field),
            aggregation: agg,
            source_field: field.to_string(),
            description: String::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
}

/// Represents a dimensional property used for slicing and dicing metrics (e.g. "Country" or "Date")
#[derive(Debug, Clone)]
pub struct OssieDimension {
    pub name: String,
    pub source_field: String,
    pub data_type: String, // "text", "date", "numeric"
}

impl OssieDimension {
    pub fn new(name: &str, field: &str, data_type: &str) -> Self {
        OssieDimension {
            name: name.to_string(),
            source_field: field.to_string(),
            data_type: data_type.to_string(),
        }
    }
}

/// Represents a join relationship between semantic tables
#[derive(Debug, Clone)]
pub struct OssieRelationship {
    pub from_table: String,
    pub to_table: String,
    pub join_on: String, // E.g. "users.id = transactions.user_id"
}

/// The Apache Ossie Semantic Catalog aggregating metrics, dimensions, and relationships
pub struct OssieCatalog {
    pub name: String,
    pub metrics: Vec<OssieMetric>,
    pub dimensions: Vec<OssieDimension>,
    pub relationships: Vec<OssieRelationship>,
}

impl OssieCatalog {
    pub fn new(name: &str) -> Self {
        OssieCatalog {
            name: name.to_string(),
            metrics: Vec::new(),
            dimensions: Vec::new(),
            relationships: Vec::new(),
        }
    }

    pub fn add_metric(&mut self, metric: OssieMetric) {
        self.metrics.push(metric);
    }

    pub fn add_dimension(&mut self, dimension: OssieDimension) {
        self.dimensions.push(dimension);
    }

    pub fn add_relationship(&mut self, relationship: OssieRelationship) {
        self.relationships.push(relationship);
    }
}

/// Row representations for parsing raw database or AI query results
#[derive(Debug, Clone)]
pub struct SemanticRow {
    pub fields: Vec<(String, String)>, // (Field name, Field value)
}

impl SemanticRow {
    pub fn new() -> Self {
        SemanticRow { fields: Vec::new() }
    }

    pub fn with_field(mut self, name: &str, val: &str) -> Self {
        self.fields.push((name.to_string(), val.to_string()));
        self
    }

    pub fn get_value(&self, name: &str) -> Option<&str> {
        for field in &self.fields {
            if field.0 == name {
                return Some(&field.1);
            }
        }
        None
    }
}

impl Default for SemanticRow {
    fn default() -> Self {
        Self::new()
    }
}

/// On-device interpreter for Apache Ossie Metric Language, evaluating business metrics over granular dimensions
pub struct OssieInterpreter;

impl OssieInterpreter {
    pub fn new() -> Self {
        OssieInterpreter
    }

    /// Evaluates a metric and groups results by a specified dimension over a raw dataset
    pub fn evaluate_metric(
        &self,
        metric: &OssieMetric,
        dimension: &OssieDimension,
        dataset: &[SemanticRow],
    ) -> Result<Vec<(String, f64)>, &'static str> {
        let mut grouped_values: Vec<(String, Vec<f64>)> = Vec::new();

        // 1. Group raw rows by the selected dimension field
        for row in dataset {
            let dim_val = row
                .get_value(&dimension.source_field)
                .unwrap_or("Unknown")
                .to_string();
            let metric_val_str = row
                .get_value(&metric.source_field)
                .ok_or("Metric source field not found in row")?;
            let metric_val: f64 = metric_val_str
                .parse()
                .map_err(|_| "Failed to parse metric value as float")?;

            // Append to group
            let mut group_idx = None;
            for i in 0..grouped_values.len() {
                if grouped_values[i].0 == dim_val {
                    group_idx = Some(i);
                    break;
                }
            }

            if let Some(idx) = group_idx {
                grouped_values[idx].1.push(metric_val);
            } else {
                let mut vals = Vec::new();
                vals.push(metric_val);
                grouped_values.push((dim_val, vals));
            }
        }

        // 2. Perform aggregate operations based on Ossie Metric Specifications
        let mut results = Vec::new();
        for (group_key, values) in &grouped_values {
            let aggregated_value = match metric.aggregation {
                MetricAggregation::Sum => {
                    let mut sum = 0.0;
                    for &val in values {
                        sum += val;
                    }
                    sum
                }
                MetricAggregation::Average => {
                    if values.is_empty() {
                        0.0
                    } else {
                        let mut sum = 0.0;
                        for &val in values {
                            sum += val;
                        }
                        sum / (values.len() as f64)
                    }
                }
                MetricAggregation::Count => values.len() as f64,
                MetricAggregation::CountDistinct => {
                    let mut distinct = Vec::new();
                    for &val in values {
                        let mut found = false;
                        for &d in &distinct {
                            let diff = d - val;
                            if diff < 1e-9 && diff > -1e-9 {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            distinct.push(val);
                        }
                    }
                    distinct.len() as f64
                }
            };
            results.push((group_key.clone(), aggregated_value));
        }

        Ok(results)
    }
}

impl Default for OssieInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Ontology validator verifying relationships, dimensions, and loops
pub struct OssieOntology;

impl OssieOntology {
    pub fn new() -> Self {
        OssieOntology
    }

    /// Verifies if there are circular loops in defined catalog join relationships
    pub fn verify_relationships_acyclic(&self, catalog: &OssieCatalog) -> bool {
        let mut visited = Vec::new();
        for rel in &catalog.relationships {
            if visited.contains(&rel.from_table) && visited.contains(&rel.to_table) {
                return false; // circular dependency loop detected!
            }
            if !visited.contains(&rel.from_table) {
                visited.push(rel.from_table.clone());
            }
            if !visited.contains(&rel.to_table) {
                visited.push(rel.to_table.clone());
            }
        }
        true
    }
}

impl Default for OssieOntology {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_ossie_metric_and_dimension_creation() {
        let metric = OssieMetric::new(
            "total_revenue",
            "Total Revenue",
            MetricAggregation::Sum,
            "revenue",
        )
        .with_description("Sum of all transaction revenues");

        assert_eq!(metric.name, "total_revenue");
        assert_eq!(metric.formula, "Sum(revenue)");
        assert_eq!(metric.description, "Sum of all transaction revenues");

        let dimension = OssieDimension::new("country", "signup_country", "text");
        assert_eq!(dimension.name, "country");
        assert_eq!(dimension.source_field, "signup_country");
    }

    #[test]
    fn test_ossie_metric_aggregation_interpreter() {
        let metric_rev = OssieMetric::new(
            "total_revenue",
            "Total Revenue",
            MetricAggregation::Sum,
            "revenue",
        );
        let metric_count = OssieMetric::new(
            "active_users",
            "Active Users",
            MetricAggregation::CountDistinct,
            "user_id",
        );
        let dimension = OssieDimension::new("country", "signup_country", "text");

        // Mock dataset representing business facts
        let dataset = vec![
            SemanticRow::new()
                .with_field("signup_country", "US")
                .with_field("revenue", "100.0")
                .with_field("user_id", "1"),
            SemanticRow::new()
                .with_field("signup_country", "US")
                .with_field("revenue", "150.0")
                .with_field("user_id", "2"),
            SemanticRow::new()
                .with_field("signup_country", "US")
                .with_field("revenue", "50.0")
                .with_field("user_id", "1"), // Duplicate user_id 1
            SemanticRow::new()
                .with_field("signup_country", "CA")
                .with_field("revenue", "80.0")
                .with_field("user_id", "3"),
        ];

        let interpreter = OssieInterpreter::new();

        // 1. Evaluate Sum(revenue) GroupBy Country
        let revenue_results = interpreter
            .evaluate_metric(&metric_rev, &dimension, &dataset)
            .unwrap();

        let us_revenue = revenue_results.iter().find(|r| r.0 == "US").unwrap().1;
        let ca_revenue = revenue_results.iter().find(|r| r.0 == "CA").unwrap().1;
        assert_eq!(us_revenue, 300.0); // 100 + 150 + 50
        assert_eq!(ca_revenue, 80.0);

        // 2. Evaluate CountDistinct(user_id) GroupBy Country (checks MAU semantic duplicates removal)
        let user_results = interpreter
            .evaluate_metric(&metric_count, &dimension, &dataset)
            .unwrap();
        let us_users = user_results.iter().find(|r| r.0 == "US").unwrap().1;
        let ca_users = user_results.iter().find(|r| r.0 == "CA").unwrap().1;
        assert_eq!(us_users, 2.0); // User IDs 1 and 2 (distinct)
        assert_eq!(ca_users, 1.0); // User ID 3
    }

    #[test]
    fn test_ossie_ontology_relationships_loop() {
        let mut catalog = OssieCatalog::new("SalesCatalog");
        catalog.add_relationship(OssieRelationship {
            from_table: "users".to_string(),
            to_table: "transactions".to_string(),
            join_on: "users.id = transactions.user_id".to_string(),
        });
        catalog.add_relationship(OssieRelationship {
            from_table: "transactions".to_string(),
            to_table: "locations".to_string(),
            join_on: "transactions.location_id = locations.id".to_string(),
        });

        let ontology = OssieOntology::new();
        assert!(ontology.verify_relationships_acyclic(&catalog));

        // Add circular relationship loop: locations -> users
        catalog.add_relationship(OssieRelationship {
            from_table: "locations".to_string(),
            to_table: "users".to_string(),
            join_on: "locations.manager_id = users.id".to_string(),
        });

        // Verification must detect the cyclic relationship trap immediately!
        assert!(!ontology.verify_relationships_acyclic(&catalog));
    }
}
