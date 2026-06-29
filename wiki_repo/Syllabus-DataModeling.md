# Data Modeling & Visualization → SigmaModeler + SigmaViz

> Maps the Data Modeling & Visualization syllabus to `SigmaModeler` — SigmaOS's ER/schema design tool, and `SigmaViz` — the visual analytics dashboard engine.

---

## Unit I: Introduction to Data Modeling & Sovereignty

### What is Data Modeling?

Data modeling defines the mathematical structure, relational dependencies, and domain constraints of enterprise data, establishing a rigorous architectural blueprint prior to database implementation.

**Unique Selling Point (USP):** Guarantees absolute data integrity, eliminates redundancy, and accelerates analytical query performance through mathematically proven schema normalization and B+ Tree indexing.

**SigmaOS Parallel:** Every SigmaDB schema is defined via `SigmaModeler`, which generates validated SQL DDL and interactive ER diagrams.

### Levels of Data Abstraction

```
Physical Level     — How data is stored (S-ZFS blocks, B+ Tree indexes)
Logical Level      — Tables, columns, relationships (SigmaDB schema)
View Level         — What users see (SigmaDB Views, SigmaDocs reports)
Conceptual Level   — ER Diagrams (SigmaModeler)
```

### Data Modeling Process

```
Requirements → Conceptual Model (ER) → Logical Model (Relational / OO)
→ Physical Model (DDL / Storage) → Implementation (SigmaDB)
→ Versioning (Git-backed) → Migration (SigmaModeler Migration Tool)
```

### Normalization in SigmaModeler

| Normal Form | Rule | SigmaModeler Check | 
| :--- | :--- | :--- | 
| **1NF** | Atomic scalar values, no repeating groups | ✅ Auto-detected | 
| **2NF** | No partial dependencies (full PK functional deps) | ✅ Enforced | 
| **3NF** | No transitive functional dependencies | ✅ Enforced | 
| **BCNF** | Every non-trivial determinant is a candidate key | ✅ Enforced | 
| **4NF** | No multi-valued dependencies | ✅ Warning | 
| **5NF** | No join dependencies | ⚠️ Manual review | 

---

## Unit II: Advanced Data Modeling Techniques & Schema Design

### Data Model Types

| Type | Structure | SigmaOS Use | 
| :--- | :--- | :--- | 
| **Hierarchical** | Tree (parent-child) | File system tree, org charts | 
| **Network** | Graph (many-to-many) | Process dependency graph | 
| **Relational** | Tables + Foreign Keys | SigmaDB primary model | 
| **Object-Oriented** | Classes + Inheritance | SigmaDB JSON/BLOB types | 
| **Dimensional** | Fact + Dimension tables | SigmaWarehouse (star schema) | 
| **NoSQL** | Document / Key-Value / Graph | SigmaDB JSON storage mode | 

### Entity-Relationship Diagram (ERD)

```
SigmaDB ERD — System Metrics Schema

┌─────────────────┐         ┌─────────────────┐
│    PROCESS      │         │    CPU_METRIC   │
├─────────────────┤         ├─────────────────┤
│ PK pid (INT)    │◄────────│ FK pid (INT)    │
│ name (VARCHAR)  │  1   N  │ PK metric_id    │
│ state (ENUM)    │         │ timestamp (TS)  │
│ start_time (TS) │         │ cpu_pct (FLOAT) │
│ FK parent_pid   │         │ mem_mb (INT)    │
└─────────────────┘         └─────────────────┘
         │
         │ N
         ▼
┌─────────────────┐
│    THREAD       │
├─────────────────┤
│ PK thread_id    │
│ FK pid (INT)    │
│ state (ENUM)    │
│ stack_addr(BIGINT)│
└─────────────────┘
```

### Dimensional Modeling (Star Schema)

```
             FACT_METRICS
             ┌──────────────┐
  DIM_TIME──►│ time_id (FK) │◄──DIM_HOST
             │ host_id (FK) │
  DIM_APP───►│ app_id (FK)  │◄──DIM_APP
             │ cpu_pct      │
             │ mem_mb       │
             │ io_mbps      │
             └──────────────┘
```

### NoSQL in SigmaDB

```python

# SigmaDB supports JSON document mode alongside relational mode

from sigma.db import SigmaDocStore

store = SigmaDocStore('/sigma/data/nosql')

# Document insert

doc = {
    "pid": 42,
    "name": "sigma-ui",
    "metadata": { "theme": "dark", "resolution": "4K" },
    "plugins": ["sigma-web", "sigma-ai"]
}
store.insert("processes", doc)

# Query

results = store.find("processes", {"name": {"$regex": "sigma-*"}})
```

---

## Unit III: Data Visualization & EDA

### Principles of Effective Visualization

1. **Clarity** — minimize chartjunk; maximize data-ink ratio.
2. **Accuracy** — use proportional scales; avoid truncated axes.
3. **Context** — always label axes, titles, and measurement units.
4. **Appropriate Encoding** — bars for comparison, lines for continuous trends, scatter plots for correlation.

### SigmaViz Chart Gallery

```python
import sigmaviz as sv

# Line chart — trends over time

sv.line(data=cpu_ts, x='timestamp', y='cpu_pct',
        title='CPU Usage Over 24h', color='#6C63FF')

# Bar chart — comparison

sv.bar(data=df, x='process', y='mem_mb',
       title='Memory by Process', color_scheme='blues')

# Histogram — distribution

sv.histogram(data=latencies, bins=50, title='Request Latency Distribution',
             color='#FF6B6B', kde=True)

# Scatter plot — correlation

sv.scatter(data=df, x='cpu_pct', y='mem_mb', hue='process',
           title='CPU vs Memory', size='io_mbps')

# Heatmap — correlation matrix

sv.heatmap(data=df.corr(), title='Feature Correlation',
           cmap='coolwarm', annotate=True)

# Interactive dashboard (renders in SigmaWeb runtime)

dashboard = sv.Dashboard(title="SigmaOS System Health")
dashboard.add_widget(sv.KPI("Active Processes", len(df)))
dashboard.add_widget(sv.LineChart(cpu_ts))
dashboard.add_widget(sv.Gauge("System Load", value=cpu_avg, max=100))
dashboard.add_widget(sv.Table(df.head(10)))
dashboard.render('/sigma/dashboard/health.html')
```

### EDA Techniques

```python
import pandas as pd
import sigmaviz as sv

df = pd.read_csv('/sigma/data/system_metrics.csv')

# Understand data shape

print(df.shape)
print(df.dtypes)
print(df.describe())
print(df.isnull().sum())

# Distribution analysis

for col in df.select_dtypes(include='float64').columns:
    sv.histogram(df[col], title=f"Distribution: {col}")

# Correlation

sv.heatmap(df.corr(), title="Correlation Matrix")

# Outlier detection

Q1, Q3 = df['cpu_pct'].quantile([0.25, 0.75])
IQR = Q3 - Q1
outliers = df[(df['cpu_pct'] < Q1 - 1.5*IQR) | (df['cpu_pct'] > Q3 + 1.5*IQR)]
sv.boxplot(df, x='process', y='cpu_pct', title="Outlier Detection")
```

---

## Unit IV: Visual Analytics for Decision Making

### SigmaOS Decision Dashboard

```python

# Interactive visual analytics in SigmaWeb runtime

from sigma.viz import DecisionDashboard

dash = DecisionDashboard("SigmaOS Executive View")

# KPI summary cards

dash.kpi_card("Total Processes", value=42, delta="+3 since yesterday")
dash.kpi_card("Avg CPU", value=f"{cpu_avg:.1f}%", status="warning" if cpu_avg>80 else "ok")
dash.kpi_card("Memory Used", value=f"{mem_pct:.0f}%", trend="up")

# Drill-down capability (click a bar to see sub-processes)

dash.interactive_bar(data=df, x='app', y='cpu_pct',
                     drilldown_col='subprocess',
                     on_click='show_process_detail')

# Filter panel

dash.add_filter('Time Range', type='daterange', default='last_24h')
dash.add_filter('Process', type='multiselect', options=df['process'].unique())

# Data storytelling narrative

dash.narrative("""

## System Health Summary — May 2026

The system processed **42 active applications** today. CPU usage peaked at **87%** at 14:30 IST, driven by the `sigma-ai` training job.
Memory remains stable at **62%** utilization. No anomalies detected.
""")

# Render to SigmaWeb

dash.render('/sigma/dashboard/executive.html', auto_refresh=30)  # refresh every 30s
```

### Integrating Data Models with Visualization

```python

# SigmaModeler → SigmaViz pipeline

from sigma.modeler import ERDiagram
from sigma.viz import SchemaVisualizer

erd = ERDiagram.load('/sigma/schemas/sigmaos.erd')
SchemaVisualizer.render(erd, output='/sigma/docs/schema.svg',
                        layout='hierarchical', theme='dark')

# Visual analytics for model performance

from sigma.viz import MLDashboard
ml_dash = MLDashboard(model=trained_model, test_data=(X_test, y_test))
ml_dash.confusion_matrix()
ml_dash.roc_curve()
ml_dash.feature_importance()
ml_dash.learning_curve()
ml_dash.render('/sigma/ai/reports/model_report.html')
```

---

## Debugging & Problem-Solving in Data Modeling

### Common Issues & Fix Strategies

* **Issue - Incorrect Indexing & B+ Tree Fragmentation:** Unindexed foreign keys or highly fragmented B+ Tree indices cause heavy sequential table scans ($O(N)$), degrading OLAP query performance.
  * *Fix Strategy:* Run `EXPLAIN QUERY PLAN` to identify unindexed joins, create composite covering B+ Tree indices (`CREATE INDEX idx_fk ON child_table(parent_id)`), and execute periodic index defragmentation (`REINDEX`).
* **Issue - Normalization Anomalies & Redundancy:** Storing unnormalized data (1NF/2NF) causes severe update, insertion, and deletion anomalies, leading to inconsistent database states.
  * *Fix Strategy:* Run `SigmaModeler` automated normalizer (`normalizer.cpp`) to decompose monolithic tables into strict 3NF/BCNF schemas, eliminating transitive functional dependencies.
* **Issue - Database Deadlocks in Transactional ER Models:** Mutually dependent transactions acquire row locks across parent-child ER tables in conflicting orders, triggering circular wait states.
  * *Fix Strategy:* Enforce strict two-phase locking (2PL) protocols, acquire table/row locks in a globally uniform hierarchical order, and implement automated deadlock detection with exponential backoff retries.

---

## SigmaModeler + SigmaViz Architecture

```
SigmaModeler
├── ER Diagram Editor (visual, drag-drop)
├── Schema Validator (NF checker)
├── Migration Generator (versioned ALTER scripts)
├── DDL Generator (SigmaDB-compatible SQL)
└── Documentation Export (PDF / HTML / SVG)

SigmaViz
├── Chart Types: Line, Bar, Scatter, Pie, Heatmap, Histogram, Boxplot
├── Interactive Dashboards (rendered in SigmaWeb)
├── OLAP Drill-down Integration (SigmaWarehouse)
├── Narrative / Storytelling Layer
├── ML Model Visualization
└── Export: PNG, SVG, PDF, HTML
```

### Files

- `userland/apps/SigmaModeler/erd_engine.cpp`
- `userland/apps/SigmaModeler/schema_validator.cpp`
- `userland/apps/SigmaViz/chart_engine.cpp`
- `userland/apps/SigmaViz/dashboard_renderer.cpp`

*Last updated: 2026-05-19 | SigmaOS Zenith v15.2*
