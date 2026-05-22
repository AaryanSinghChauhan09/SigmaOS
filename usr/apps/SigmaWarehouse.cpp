/**
 * SigmaWarehouse.cpp â€” Data Warehouse & Mining Engine
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-DWDM (Data Preprocessing, OLAP, ETL, Data Mining)
 * Implements: ETL pipeline, DataCube, OLAP operations, Association Rule Mining
 */
#include "SigmaWarehouse.h"

namespace Sigma::Warehouse {

// â”€â”€â”€ Data Preprocessing â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

DataSet DataPreprocessor::fill_missing_mean(const DataSet& ds) {
    DataSet result = ds.clone();
    for (sigma_u32 f = 0; f < ds.n_features; f++) {
        double sum = 0; sigma_u32 cnt = 0;
        for (sigma_u32 r = 0; r < ds.n_samples; r++)
            if (!ds.is_missing(r, f)) { sum += ds.X[r][f]; cnt++; }
        double mean = cnt ? sum / cnt : 0;
        for (sigma_u32 r = 0; r < ds.n_samples; r++)
            if (result.is_missing(r, f)) result.X[r][f] = mean;
    }
    return result;
}

DataSet DataPreprocessor::remove_duplicates(const DataSet& ds) {
    DataSet result;
    result.n_features = ds.n_features;
    result.n_samples  = 0;
    bool* seen = new bool[ds.n_samples]();
    for (sigma_u32 i = 0; i < ds.n_samples; i++) {
        if (seen[i]) continue;
        result.add_row(ds.X[i]);
        for (sigma_u32 j = i+1; j < ds.n_samples; j++) {
            bool dup = true;
            for (sigma_u32 f = 0; f < ds.n_features; f++)
                if (ds.X[i][f] != ds.X[j][f]) { dup = false; break; }
            if (dup) seen[j] = true;
        }
    }
    delete[] seen;
    return result;
}

DataSet DataPreprocessor::remove_outliers_zscore(const DataSet& ds, double threshold) {
    // Z-score: remove rows where |z| > threshold for any feature
    double* means = new double[ds.n_features]();
    double* stds  = new double[ds.n_features]();
    for (sigma_u32 f = 0; f < ds.n_features; f++) {
        double sum = 0;
        for (sigma_u32 r = 0; r < ds.n_samples; r++) sum += ds.X[r][f];
        means[f] = sum / ds.n_samples;
        double var = 0;
        for (sigma_u32 r = 0; r < ds.n_samples; r++) {
            double d = ds.X[r][f] - means[f]; var += d*d;
        }
        stds[f] = sigma_sqrt(var / ds.n_samples) + 1e-9;
    }
    DataSet result; result.n_features = ds.n_features;
    for (sigma_u32 r = 0; r < ds.n_samples; r++) {
        bool outlier = false;
        for (sigma_u32 f = 0; f < ds.n_features; f++) {
            double z = (ds.X[r][f] - means[f]) / stds[f];
            if (z > threshold || z < -threshold) { outlier = true; break; }
        }
        if (!outlier) result.add_row(ds.X[r]);
    }
    delete[] means; delete[] stds;
    return result;
}

// â”€â”€â”€ ETL Pipeline â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

int ETLPipeline::run() {
    sigma_klog(sigma_log_info, "[SigmaWarehouse] ETL pipeline '%s' starting\n", m_name);
    // 1. EXTRACT: read from source
    DataSet raw = m_extractor->extract();
    sigma_klog(sigma_log_info, "[SigmaWarehouse] Extracted %u rows\n", raw.n_samples);

    // 2. TRANSFORM: apply all transform steps
    DataSet current = raw;
    for (sigma_u32 i = 0; i < m_transform_count; i++) {
        current = m_transforms[i]->transform(current);
        sigma_klog(sigma_log_info, "[SigmaWarehouse] Transform %u: %u rows\n", i, current.n_samples);
    }

    // 3. LOAD: write to destination
    int rc = m_loader->load(current);
    sigma_klog(sigma_log_info, "[SigmaWarehouse] ETL complete: %u rows loaded, rc=%d\n",
               current.n_samples, rc);
    return rc;
}

// â”€â”€â”€ DataCube & OLAP â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

void DataCube::build(const DataSet& ds) {
    m_ds = ds.clone();
    // Index unique values per dimension
    for (sigma_u32 d = 0; d < m_n_dims; d++) {
        m_dim_values[d].clear();
        for (sigma_u32 r = 0; r < ds.n_samples; r++) {
            double v = ds.X[r][m_dims[d]];
            bool found = false;
            for (sigma_u32 i = 0; i < m_dim_values[d].size(); i++)
                if (m_dim_values[d][i] == v) { found = true; break; }
            if (!found) m_dim_values[d].push(v);
        }
    }
}

// Roll up: aggregate to coarser granularity
DataSet DataCube::roll_up(sigma_u32 dim_idx, AggFunc agg) {
    DataSet result;
    // Group by all other dimensions, aggregate on dim_idx
    // (simplified: aggregate over entire dimension)
    sigma_klog(sigma_log_info, "[DataCube] ROLL UP dim=%u\n", dim_idx);
    return aggregate_by_dim(dim_idx, agg);
}

// Drill down: more detailed view
DataSet DataCube::drill_down(sigma_u32 dim_idx) {
    sigma_klog(sigma_log_info, "[DataCube] DRILL DOWN dim=%u\n", dim_idx);
    return m_ds; // Already at finest granularity
}

// Slice: fix one dimension to a value
DataSet DataCube::slice(sigma_u32 dim_idx, double value) {
    DataSet result; result.n_features = m_ds.n_features;
    for (sigma_u32 r = 0; r < m_ds.n_samples; r++)
        if (m_ds.X[r][m_dims[dim_idx]] == value)
            result.add_row(m_ds.X[r]);
    return result;
}

// Dice: filter on multiple dimensions
DataSet DataCube::dice(const DiceFilter* filters, sigma_u32 n_filters) {
    DataSet result; result.n_features = m_ds.n_features;
    for (sigma_u32 r = 0; r < m_ds.n_samples; r++) {
        bool match = true;
        for (sigma_u32 f = 0; f < n_filters; f++)
            if (m_ds.X[r][filters[f].dim] != filters[f].value) { match = false; break; }
        if (match) result.add_row(m_ds.X[r]);
    }
    return result;
}

// â”€â”€â”€ Association Rule Mining (Apriori algorithm) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

// Step 1: Generate frequent itemsets
FrequentItemsets Apriori::find_frequent_itemsets(
    const Transaction* transactions, sigma_u32 n_trans,
    double min_support) {

    FrequentItemsets result;
    sigma_u32 min_count = (sigma_u32)(min_support * n_trans);

    // Generate 1-itemsets
    ItemsetList candidates = generate_1_candidates(transactions, n_trans);
    ItemsetList freq = prune_infrequent(candidates, transactions, n_trans, min_count);

    while (!freq.empty()) {
        result.add_all(freq);
        ItemsetList next_candidates = apriori_gen(freq);
        freq = prune_infrequent(next_candidates, transactions, n_trans, min_count);
    }
    return result;
}

// Step 2: Generate rules from frequent itemsets
AssocRuleList Apriori::generate_rules(const FrequentItemsets& fis,
                                       double min_confidence) {
    AssocRuleList rules;
    for (sigma_u32 i = 0; i < fis.count; i++) {
        const Itemset& full = fis.itemsets[i];
        if (full.size < 2) continue;
        // Try all non-empty proper subsets as antecedent
        for (sigma_u32 mask = 1; mask < (1u << full.size) - 1; mask++) {
            Itemset ant, cons;
            split_by_mask(full, mask, ant, cons);
            double conf = support(full, fis) / support(ant, fis);
            if (conf >= min_confidence) {
                AssocRule rule;
                rule.antecedent = ant;
                rule.consequent = cons;
                rule.support    = fis.get_support(full);
                rule.confidence = conf;
                rule.lift       = conf / fis.get_support(cons);
                rules.push(rule);
            }
        }
    }
    return rules;
}

} // namespace Sigma::Warehouse

