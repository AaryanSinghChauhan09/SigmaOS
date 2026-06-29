# Advanced Python & Data Science → SigmaAI Data Pipeline

> Maps the Advanced Python syllabus (NumPy, Pandas, Matplotlib, Seaborn, Scikit-Learn) to SigmaOS analytics components, emphasizing high-performance data wrangling, out-of-core processing, and robust ML pipelines.

---

## Unit I: Data Science Lifecycle, Setup & NumPy Core

### Data Science Lifecycle in SigmaOS

```
Problem Definition → Data Collection (SigmaDB / SovereignFS)
→ Data Cleaning (SigmaETL) → EDA (SigmaViz / Matplotlib)
→ Feature Engineering (Pandas / NumPy) → Model Training (SigmaAI / Scikit-Learn)
→ Evaluation & Diagnostics → Deployment (SigmaOS Microservice)
```

**Unique Selling Point (USP):** High-performance vectorization, out-of-core data wrangling, and end-to-end machine learning automation with seamless bare-metal C++ bridging.

### Tools Setup (SigmaOS)

```bash

# SigmaOS ships with an Anaconda-compatible Python environment

sigma pkg install sigma-datascience  # installs numpy, pandas, matplotlib, seaborn, sklearn
sigma-jupyter start                  # Launch Jupyter-compatible notebook server
sigma-py --repl                      # Interactive Python REPL
```

### NumPy

```python
import numpy as np

# ndarray creation

arr = np.array([1, 2, 3, 4, 5])
zeros = np.zeros((3, 4))
ones  = np.ones((2, 3, 4))
eye   = np.eye(4)                  # Identity matrix
rand  = np.random.randn(100, 10)   # Normal distribution
arange = np.arange(0, 100, 5)      # Like range(), returns array
linspace = np.linspace(0, 1, 100)  # 100 evenly spaced points

# Data types

arr_int = np.array([1,2,3], dtype=np.int32)
arr_f64 = np.array([1.0, 2.0], dtype=np.float64)
arr_bool = np.array([True, False, True])

# Array operations (vectorized — no loops needed)

a = np.array([1, 2, 3])
b = np.array([4, 5, 6])
print(a + b)        # [5, 7, 9]
print(a * b)        # [4, 10, 18]
print(a ** 2)       # [1, 4, 9]
print(np.sqrt(a))   # [1.0, 1.414, 1.732]

# Indexing and slicing

arr2d = np.arange(24).reshape(4, 6)
arr2d[1, 3]          # Row 1, Col 3
arr2d[0:2, 1:4]      # Sub-matrix
arr2d[:, -1]         # Last column
arr2d[arr2d > 10]    # Boolean mask

# Transposing

arr2d.T              # Transpose
arr3d = np.ones((2, 3, 4))
np.swapaxes(arr3d, 0, 1)

# Universal functions (ufuncs)

np.sqrt(arr); np.exp(arr); np.log(arr); np.sin(arr)
np.maximum(arr, 0)  # ReLU activation!

# Math and stats

np.mean(arr); np.median(arr); np.std(arr); np.var(arr)
np.sum(arr); np.cumsum(arr); np.prod(arr)
np.min(arr); np.max(arr); np.argmin(arr); np.argmax(arr)

# Sorting

np.sort(arr)           # Sorted copy
np.argsort(arr)        # Indices that would sort
np.unique(arr)         # Unique values

# Reshaping and concatenation

arr.reshape(2, -1)             # -1 auto-infers dimension
np.concatenate([a, b], axis=0)
np.vstack([a, b])              # Vertical stack
np.hstack([a, b])              # Horizontal stack
np.split(arr, [2])             # Split at index 2
np.tile(a, (2, 3))             # Repeat 2 rows, 3 cols
np.repeat(a, 3)                # Repeat each element 3x

# File I/O

np.save('/sigma/data/array.npy', arr)
loaded = np.load('/sigma/data/array.npy')
np.savetxt('/sigma/data/matrix.csv', arr2d, delimiter=',')
loaded_txt = np.loadtxt('/sigma/data/matrix.csv', delimiter=',')
```

---

## Unit II: Pandas, Time Series & Group Operations

```python
import pandas as pd

# Series

s = pd.Series([10, 20, 30], index=['a', 'b', 'c'])
s['a']     # 10
s[s > 15]  # b=20, c=30

# DataFrame

df = pd.DataFrame({
    'process': ['init', 'sigma-ui', 'sigma-net', 'sigma-ai'],
    'pid':     [1, 42, 43, 99],
    'cpu_pct': [0.1, 15.4, 2.3, 45.0],
    'mem_mb':  [10, 512, 128, 2048]
})

# Index operations

df.index = df['pid']
df = df.reindex([1, 42, 43, 99, 100])  # 100 becomes NaN
df.reindex(method='ffill')             # forward fill
df.reindex(method='bfill')             # backward fill

# Selection and filtering

df['process']                          # column
df[['pid', 'cpu_pct']]                 # multiple columns
df.loc[42]                             # by label
df.iloc[0]                             # by integer position
df.loc[df['cpu_pct'] > 10]            # boolean filter
df.query('cpu_pct > 10 and mem_mb > 100')

# Arithmetic alignment

df['total'] = df['cpu_pct'] + df['mem_mb'] / 1024
df['cpu_norm'] = df['cpu_pct'].apply(lambda x: x / 100)
df.sort_values('cpu_pct', ascending=False)
df.rank()

# Missing data

df.dropna()
df.fillna(0)
df.fillna(df.mean(numeric_only=True))
df['cpu_pct'].fillna(df.groupby('process')['cpu_pct'].transform('mean'))

# Group operations

df.groupby('process').agg({'cpu_pct': 'mean', 'mem_mb': 'sum'})
df.groupby('process').apply(lambda g: g.nlargest(1, 'cpu_pct'))

# Time series

ts = pd.Series(range(100), index=pd.date_range('2026-01-01', periods=100, freq='h'))
ts.resample('D').mean()           # Daily average
ts.rolling(window=7).mean()       # 7-period moving average
ts.shift(1)                       # Lag by 1
ts.diff()                         # First difference

# Reading/writing

df = pd.read_csv('/sigma/data/metrics.csv')
df = pd.read_json('/sigma/data/config.json')
df.to_csv('/sigma/data/output.csv', index=False)
df.to_parquet('/sigma/data/metrics.parquet')
```

---

## Unit III: Data Wrangling & Visualization

```python
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# Merging datasets

df_procs = pd.DataFrame({'pid': [1, 42], 'name': ['init', 'sigma-ui']})
df_metrics = pd.DataFrame({'pid': [1, 42], 'cpu': [0.1, 15.4]})
pd.merge(df_procs, df_metrics, on='pid', how='inner')
pd.merge(df_procs, df_metrics, on='pid', how='left')

df1 = pd.DataFrame({'A': [1, 2]})
df2 = pd.DataFrame({'A': [3, 4]})
pd.concat([df1, df2], axis=0)     # vertical stack
pd.concat([df1, df2], axis=1)     # horizontal join

# Reshaping

df_reset = df.reset_index(drop=True)
df_wide = pd.DataFrame({'date': ['2026-05-18'], 'init': [0.1], 'sigma-ui': [15.4]})
df_melt = df_wide.melt(id_vars=['date'], var_name='process', value_name='cpu_pct')

# Transformations

df.drop_duplicates(subset=['pid'])
df['cpu_pct'] = df['cpu_pct'].replace({-1: 0, None: 0})
df = df.rename(columns={'cpu_pct': 'cpu_percent'})
df['cpu_percent'] = df['cpu_percent'].clip(lower=0, upper=100)   # Remove outliers
df = pd.get_dummies(df, columns=['process'])  # One-hot encoding

# Matplotlib

fig, axes = plt.subplots(2, 2, figsize=(12, 8))
fig.suptitle('SigmaOS System Dashboard')

axes[0,0].plot(ts.index, ts.values, color='#6C63FF', linewidth=2)
axes[0,0].set_title('CPU Over Time')

axes[0,1].bar(df['pid'], df['cpu_percent'], color='#4CAF50')
axes[0,1].set_title('CPU by Process')

axes[1,0].hist(df['mem_mb'], bins=20, color='#FF6B6B', edgecolor='white')
axes[1,0].set_title('Memory Distribution')

axes[1,1].scatter(df['cpu_percent'], df['mem_mb'], c='#00BCD4', s=100, alpha=0.7)
axes[1,1].set_title('CPU vs Memory')

plt.tight_layout()
plt.savefig('/sigma/reports/dashboard.png', dpi=150)

# Seaborn

sns.set_theme(style='darkgrid', palette='deep')
ts_df = pd.DataFrame({'time': ts.index, 'cpu': ts.values})
sns.lineplot(data=ts_df, x='time', y='cpu')
sns.boxplot(data=df, x='pid', y='cpu_percent')
```

---

## Unit IV: Machine Learning with Scikit-Learn

```python
from sklearn.model_selection import train_test_split, cross_val_score
from sklearn.preprocessing import StandardScaler, LabelEncoder
from sklearn.pipeline import Pipeline
from sklearn.ensemble import RandomForestClassifier, GradientBoostingClassifier
from sklearn.linear_model import LinearRegression, Ridge
from sklearn.cluster import KMeans
from sklearn.metrics import (accuracy_score, classification_report,
                              mean_squared_error, r2_score)
import joblib

# Data prep

df_ml = pd.DataFrame({
    'cpu': [10, 20, 80, 90],
    'mem': [100, 200, 800, 900],
    'anomaly': [0, 0, 1, 1]
})
X, y = df_ml.drop('anomaly', axis=1), df_ml['anomaly']
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.5, random_state=42)

# Pipeline (preprocessing + model in one)

pipe = Pipeline([
    ('scaler', StandardScaler()),
    ('clf',    RandomForestClassifier(n_estimators=100, max_depth=5))
])
pipe.fit(X_train, y_train)
y_pred = pipe.predict(X_test)

# Evaluation

print(f"Accuracy:  {accuracy_score(y_test, y_pred):.4f}")
print(classification_report(y_test, y_pred))

# Regression

reg = Ridge(alpha=1.0)
reg.fit(X_train, y_train)
print(f"R²: {r2_score(y_test, reg.predict(X_test)):.4f}")
print(f"RMSE: {mean_squared_error(y_test, reg.predict(X_test), squared=False):.4f}")

# Overfitting detection

train_score = pipe.score(X_train, y_train)
test_score  = pipe.score(X_test, y_test)
if train_score - test_score > 0.1:
    print("WARNING: Overfitting detected")

# Cross-validation

cv_scores = cross_val_score(pipe, X, y, cv=2)
print(f"CV Mean: {cv_scores.mean():.4f} ± {cv_scores.std():.4f}")

# Save / load model

joblib.dump(pipe, '/sigma/ai/models/anomaly_detector.pkl')
loaded = joblib.load('/sigma/ai/models/anomaly_detector.pkl')
```

---

## Debugging & Problem-Solving in Advanced Python

### Common Issues & Fix Strategies

* **Issue - Out-of-Memory (OOM) Crashes during Batch ETL:** Trying to load multi-gigabyte Parquet or CSV files entirely into Pandas RAM triggers fatal kernel OOM terminations.
  * *Fix Strategy:* Use `pd.read_csv(chunksize=10000)` to stream and aggregate chunks iteratively, or migrate to Dask / PySpark for distributed out-of-core lazy execution.
* **Issue - Data Leakage during Preprocessing Pipelines:** Fitting `StandardScaler` or `SimpleImputer` on the entire dataset before `train_test_split` leaks future test distribution metrics into the training phase.
  * *Fix Strategy:* Always encapsulate scaling and imputation steps within a Scikit-Learn `Pipeline`, ensuring `fit_transform` executes solely on the active training fold during cross-validation.
* **Issue - SettingWithCopyWarning in Pandas:** Modifying DataFrame slices (`df[df['cpu'] > 50]['mem'] = 0`) triggers ambiguous CoW assignment warnings and fails to update the original frame.
  * *Fix Strategy:* Utilize explicit `.loc` indexing (`df.loc[df['cpu'] > 50, 'mem'] = 0`) or create an explicit deep copy (`df_sub = df[df['cpu'] > 50].copy()`).

---

## SigmaOS Integration Map

| Component | Role | 
| :--- | :--- | 
| **NumPy** | Tensor/matrix ops in `SigmaAI::NPURuntime` | 
| **Pandas** | ETL data frames in `SigmaWarehouse` | 
| **Matplotlib / Seaborn** | Charts rendered by `SigmaViz` | 
| **Scikit-Learn** | ML models in `SigmaAI::MLEngine` | 
| **Joblib** | Model serialization to `SovereignFS` | 
| **Jupyter-like** | `sigma-jupyter` notebook server | 

*Last updated: 2026-05-19 | SigmaOS Zenith v15.2*
