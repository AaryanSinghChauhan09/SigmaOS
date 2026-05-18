# Data Warehousing & Mining → SigmaViz Analytics Layer

> Maps the DWDM syllabus to `SigmaWarehouse` + `SigmaViz` — the analytics backbone of SigmaOS.

---

## Unit I: Data Preprocessing & Warehousing

### Why Preprocess Data?

Raw data is: incomplete, noisy, inconsistent, redundant.

```python
from sigma.analytics import DataPreprocessor

dp = DataPreprocessor()

# Data Cleaning
df = dp.fill_missing(df, strategy='mean')     # Replace NaN with column mean
df = dp.remove_outliers(df, method='zscore', threshold=3.0)
df = dp.drop_duplicates(df)

# Data Integration
df_merged = dp.merge([df_telemetry, df_events], on='timestamp', how='inner')

# Data Reduction
df_reduced = dp.pca(df, n_components=10)      # PCA: keep top 10 components
df_sampled = dp.sample(df, n=10000, strategy='stratified')

# Data Transformation
df_norm = dp.normalize(df, method='min-max')  # Scale to [0, 1]
df_std  = dp.standardize(df, method='zscore') # Mean=0, Std=1
df_enc  = dp.encode_categorical(df, columns=['os_type'], method='onehot')
```

### Data Warehouse Concepts

```
OLTP (Transactional DB)       →  ETL  →  Data Warehouse  →  Data Marts
Real-time, normalized, CRUD          Denormalized, historical, read-heavy

SigmaDB (OLTP)  →  SigmaETL  →  SigmaWarehouse  →  SigmaViz Dashboards
```

### Data Cube & OLAP Operations

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

### ETL Pipeline

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

---

## Unit II: Data Mining

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

**Files:**
- `userland/apps/SigmaWarehouse/sigma_warehouse.cpp`
- `userland/apps/SigmaAnalytics/data_mining.cpp`
- `userland/apps/SigmaViz/olap_dashboard.cpp`

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
