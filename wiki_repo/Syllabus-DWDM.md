# Data Warehousing & Mining → SigmaViz Analytics Layer

> Maps the DWDM syllabus to `SigmaWarehouse` + `SigmaViz` — the enterprise analytics and data mining backbone of SigmaOS.

---

## Unit I: Data Warehousing & Preprocessing

### Why Preprocess Data?

Raw enterprise data is inherently incomplete, noisy, inconsistent, and redundant. Preprocessing transforms raw data shards into clean, silicon-direct feature matrices.

```python
from sigma.analytics import DataPreprocessor

dp = DataPreprocessor()

# Data Cleaning & Imputation

df = dp.fill_missing(df, strategy='mean')     # Replace NaN with column mean

df = dp.remove_outliers(df, method='zscore', threshold=3.0)
df = dp.drop_duplicates(df)

# Data Integration

df_merged = dp.merge([df_telemetry, df_events], on='timestamp', how='inner')

# Data Reduction

df_reduced = dp.pca(df, n_components=10)      # PCA: keep top 10 components

df_sampled = dp.sample(df, n=10000, strategy='stratified')

# Data Transformation (Normalization & Standardization)

df_norm = dp.normalize(df, method='min-max')  # Scale to [0, 1]

df_std  = dp.standardize(df, method='zscore') # Mean=0, Std=1

df_enc  = dp.encode_categorical(df, columns=['os_type'], method='onehot')
```

### Data Warehouse Concepts & Schema Design

A Data Warehouse is a centralized, consolidated repository designed for analytical query processing (OLAP) rather than daily transaction processing (OLTP).

**Unique Selling Point (USP):** Efficient storage, horizontal scalability, and rapid retrieval for enterprise-grade analytics.

```
OLTP (Transactional DB)       →  ETL  →  Data Warehouse  →  Data Marts
Real-time, normalized, CRUD          Denormalized, historical, read-heavy

SigmaDB (OLTP)  →  SigmaETL  →  SigmaWarehouse  →  SigmaViz Dashboards
```

### Star Schema vs. Snowflake Schema

Schema design dictates how multi-dimensional analytical data is structured within the data warehouse:

| Feature | Star Schema | Snowflake Schema |
| :--- | :--- | :--- |
| **Structure** | Central fact table connected directly to denormalized dimension tables. | Central fact table connected to normalized, branching dimension tables. |
| **Normalization** | Denormalized (dimensions contain redundant data). | Normalized (dimensions are split into sub-dimensions E.g., City $\rightarrow$ State $\rightarrow$ Country). |
| **Query Complexity**| Simple, requiring fewer `JOIN` operations; highly optimized for fast read aggregations. | Complex, requiring extensive multi-table `JOIN` operations. |
| **Storage Space** | Higher storage footprint due to data redundancy. | Minimal storage footprint due to strict normalization. |
| **Maintenance** | Prone to update anomalies if dimension attributes change. | Easy to maintain and update due to centralized dimension definitions. |

### Data Cube & OLAP Operations

OLAP Cubes provide multi-dimensional conceptual views of data, enabling rapid analytical aggregations across axes E.g., time, geography, and product lines.

```python
from sigma.warehouse import DataCube

# Create OLAP cube: CPU/Memory metrics by time, host, app

cube = DataCube(
    facts=['cpu_pct', 'mem_mb', 'io_mbps'],
    dimensions=['timestamp', 'hostname', 'app_name']
)
cube.load('/sigma/data/metrics.parquet')

# OLAP Operations

cube.roll_up('timestamp', 'month')       # Aggregate by month

cube.drill_down('timestamp', 'hour')     # Granular view

cube.slice(app_name='sigma-ui')          # Fix one dimension

cube.dice(hostname='node1', month='May') # Filter 2+ dimensions

cube.pivot(rows='app_name', cols='month', values='cpu_pct')
```

### ETL (Extract, Transform, Load) Pipeline

```python
from sigma.warehouse import ETLPipeline

pipeline = ETLPipeline(
    extract=SigmaDB.query("SELECT * FROM system_metrics WHERE date > '2026-01-01'"),
    transform=[
        lambda df: df.dropna(),
        lambda df: df.rename(columns={'ts': 'timestamp'}),
        lambda df: df.assign(month=df['timestamp'].dt.month)
    ],
    load=SigmaWarehouse.table('metrics_warehouse')
)
pipeline.run(schedule='daily')  # Runs via SigmaOS task scheduler

```

### Tools & Enterprise Ecosystem Parity

SigmaWarehouse provides drop-in compatibility and bridging with major enterprise data warehousing platforms:

- **SQL Server & Oracle:** Direct ODBC/JDBC query translation and stored procedure execution.

- **Amazon Redshift:** Columnar Parquet/ORC file ingestion and distributed query sharding.

- **Snowflake:** Cloud-native virtual warehouse scaling and zero-copy data cloning emulation.

---

## Unit II: Data Mining Algorithms

### Association Rule Mining

```python
from sigma.analytics.mining import Apriori, FPGrowth

# Find frequent itemsets in process co-occurrence

rules = Apriori(
    transactions=proc_cooccurrence_data,
    min_support=0.3,
    min_confidence=0.7,
    min_lift=1.2
)

# Rule format: {sigma-ui} → {sigma-net} [support=0.45, conf=0.82, lift=1.6]

# Meaning: When sigma-ui runs, sigma-net runs 82% of the time

for rule in rules:
    print(f"{rule.antecedent} → {rule.consequent}")
    print(f"  Support: {rule.support:.2f}, Confidence: {rule.confidence:.2f}")
```

### Classification

```python
from sigma.analytics.mining import DecisionTree, NaiveBayes, RandomForest

# Classify log entries as normal / anomalous

clf = RandomForest(n_estimators=200, max_depth=10)
clf.fit(X_train_logs, y_train_labels)

# Decision Tree visualization

dt = DecisionTree(max_depth=5)
dt.fit(X_train, y_train)
sigma.viz.plot_tree(dt, feature_names=X.columns)

# Naive Bayes for text classification

nb = NaiveBayes(type='multinomial')
nb.fit(tfidf_matrix, y_labels)
```

### Clustering

```python
from sigma.analytics.mining import KMeans, DBSCAN, Hierarchical

# Cluster system processes by resource usage pattern

kmeans = KMeans(n_clusters=4, random_state=42)
cluster_labels = kmeans.fit_predict(resource_matrix)

# DBSCAN for anomaly detection (outliers = -1)

dbscan = DBSCAN(eps=0.5, min_samples=5)
labels = dbscan.fit_predict(process_metrics)
anomalies = process_metrics[labels == -1]

# Hierarchical clustering

hc = Hierarchical(n_clusters=3, linkage='ward')
dendogram = hc.fit_predict(X)
sigma.viz.dendrogram(hc)
```

---

## Debugging & Problem-Solving in Data Warehousing

### Common Issues & Fix Strategies

- **Issue - Incorrect Indexing in Databases:** Missing or fragmented B+ Tree indices cause full table scans, degrading analytical query performance.
- *Fix Strategy:* Run `EXPLAIN QUERY PLAN` to identify unindexed scans, create composite covering indices for frequent `WHERE` and `JOIN` clauses, and periodically rebuild fragmented index trees.

- **Issue - Database Deadlocks:** Concurrent ETL write transactions lock identical tables in reverse order, causing circular wait states.
- *Fix Strategy:* Enforce strict two-phase locking (2PL) protocols, acquire table locks in a globally uniform hierarchical order, and implement automatic deadlock detection with exponential backoff retries.

- **Issue - Data Corruption & Missing Values:** Sensor dropouts or network failures inject `NULL` values into OLAP fact tables.
- *Fix Strategy:* Use automated ETL data preprocessors to execute k-NN or mean imputation, ensuring analytical hypercubes remain fully populated.

- **Issue - Algorithmic Complexity in Mining Scans:** Apriori candidate generation ($C_k$) scales exponentially ($O(2^d)$) with unique item counts.
- *Fix Strategy:* Migrate from Apriori to FP-Growth tree traversal, compressing transactional databases into in-memory prefix trees to eliminate candidate generation entirely.

---

## SigmaWarehouse Architecture

```
SigmaWarehouse Stack
├── Source Systems: SigmaDB, Log Files, HAL Sensors
├── ETL Layer: SigmaETL (Extract-Transform-Load pipelines)
├── Storage: SovereignFS Parquet/ORC + SigmaDB columnstore
├── OLAP Engine: DataCube with roll-up/drill-down/slice/dice
├── Data Marts: CPU, Memory, Network, Security, App-specific
├── Mining Engine: Apriori, Decision Trees, Clustering
└── Visualization: SigmaViz dashboards
```

### Files

- `userland/apps/SigmaWarehouse/sigma_warehouse.cpp`

- `userland/apps/SigmaAnalytics/data_mining.cpp`

- `userland/apps/SigmaViz/olap_dashboard.cpp`

### Last updated: 2026-05-19 | SigmaOS Zenith v15.2
