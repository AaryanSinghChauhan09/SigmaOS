# 📊 Data Science Deep Dive

SigmaOS embeds a native **Data Science Shard** that runs statistical analysis, visualization, and ML preprocessing entirely on-device — no Jupyter, no Python runtime, no cloud backend.

---

## Architecture

```text
Raw Data (VFS / Upload)
    └─► DS Shard (kernel/SovereignML.c + sigma_std.c)
            ├─► Ingestion (CSV/binary parsing)
            ├─► Statistical Engine
            │       ├─► Mean, Variance, Std Dev
            │       ├─► Pearson Correlation
            │       └─► Histogram Bucketing
            ├─► ML Preprocessing
            │       ├─► Normalization (Min-Max / Z-Score)
            │       ├─► One-hot encoding
            │       └─► Train/Test split
            └─► Visualization Engine (browser UI canvas)
```

---

## Statistical Functions (Native C11)

All stat functions in `sigma_std.c` and `SovereignML.c`:

```c
// Mean
sigma_f64 sigma_mean(sigma_f64* arr, sigma_u32 n);

// Variance
sigma_f64 sigma_variance(sigma_f64* arr, sigma_u32 n);

// Standard Deviation
sigma_f64 sigma_stddev(sigma_f64* arr, sigma_u32 n);

// Pearson Correlation
sigma_f64 sigma_pearson(sigma_f64* x, sigma_f64* y, sigma_u32 n);

// Histogram
void sigma_histogram(sigma_f64* arr, sigma_u32 n, sigma_u32 bins, sigma_u32* out);
```

---

## ML Preprocessing Pipeline

```c
// Normalize array to [0, 1]
void sigma_normalize_minmax(sigma_f64* arr, sigma_u32 n);

// Z-Score standardization
void sigma_standardize(sigma_f64* arr, sigma_u32 n);

// Train/test split
void sigma_train_test_split(sigma_f64* data, sigma_u32 n, sigma_f64 ratio,
                             sigma_f64* train, sigma_f64* test);
```

---

## Browser UI: Data Science Shard Panel

The `SigmaSystem` class in `index.js` renders an interactive DS dashboard:

- **Upload CSV**: Parse and visualize datasets from the VFS
- **Chart Types**: Bar, line, scatter, histogram rendered on HTML Canvas
- **Live Statistics**: Running mean/variance display as data streams in
- **Correlation Matrix**: Color-coded heatmap of feature correlations
- **Export Report**: Generate a `.sigma` data report to VFS

---

## Comparison: SigmaOS vs Traditional DS Tools

| Feature | Jupyter / Python | SigmaOS DS Shard |
| --------- | ----------------- | ----------------- |
| **Runtime** | Python interpreter required | Native C11, zero interpreter |
| **Dependencies** | NumPy, pandas, matplotlib | Zero external libraries |
| **Privacy** | Cloud notebooks possible | Always local, zero telemetry |
| **Performance** | GIL-bound Python | Direct memory access, cache-friendly |
| **Integration** | External installation | Built into kernel, always available |

---

## Roadmap

- [ ] Native CSV parser with schema inference
- [ ] Interactive scatter plot with zoom/pan
- [ ] PCA (Principal Component Analysis) in pure C11
- [ ] K-Means clustering visualization
- [ ] Time-series decomposition (trend + seasonality + residual)
