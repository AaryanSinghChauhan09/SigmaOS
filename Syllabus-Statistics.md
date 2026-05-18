# Statistics → SigmaStats Toolkit

> Maps the Statistics syllabus to `SigmaStats` — SigmaOS's built-in statistical analysis library, integrated with SigmaViz and SigmaAI.

---

## Unit I: Introduction & Data Collection

### What is Statistics?
Statistics is the science of collecting, organizing, analyzing, interpreting, and presenting data.

**SigmaOS Integration:** `SigmaStats` provides kernel-level statistical primitives used by:
- `SigmaViz` — chart rendering
- `SigmaAI` — ML feature engineering
- `SigmaSheets` — spreadsheet formulas
- `SigmaDB` — aggregate query functions

### Primary vs Secondary Data

| Type | Source | SigmaOS Analogy |
|---|---|---|
| **Primary** | Collected firsthand (surveys, sensors) | Live telemetry from SigmaOS sensors |
| **Secondary** | Pre-collected (databases, reports) | Historical data from SigmaDB |

### Diagrammatic Representation — SigmaViz Charts

```python
# SigmaViz Python API (SigmaPy integration)
import sigmaviz as sv

data = [23, 45, 12, 67, 34, 89, 55]
labels = ["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]

sv.bar_chart(data, labels, title="Weekly CPU Usage")
sv.pie_chart(data, labels, title="Resource Distribution")
sv.histogram(data, bins=5, title="Latency Distribution")
sv.frequency_polygon(data, title="Frequency Polygon")
sv.frequency_curve(data, title="Ogive / Frequency Curve")
```

---

## Unit II: Measures of Central Tendency & Dispersion

### Central Tendency

```cpp
// kernel/math/SigmaStats.h
namespace Sigma::Stats {

    double arithmetic_mean(const double* data, size_t n) {
        double sum = 0;
        for (size_t i = 0; i < n; i++) sum += data[i];
        return sum / n;
    }

    double median(double* data, size_t n);  // Sort then pick middle
    double mode(const double* data, size_t n);  // Most frequent value

    // GM = (x1 * x2 * ... * xn)^(1/n)
    double geometric_mean(const double* data, size_t n);

    // HM = n / (1/x1 + 1/x2 + ... + 1/xn)
    double harmonic_mean(const double* data, size_t n);

    // Relation: AM >= GM >= HM (for positive values)
}
```

### Measures of Dispersion

```cpp
namespace Sigma::Stats {

    double range(const double* data, size_t n);  // max - min

    double mean_deviation(const double* data, size_t n);
    // MD = Σ|xi - mean| / n

    double variance(const double* data, size_t n);
    // σ² = Σ(xi - mean)² / n

    double std_deviation(const double* data, size_t n);
    // σ = sqrt(variance)

    double quartile(const double* sorted, size_t n, int q);
    // Q1 = 25th percentile, Q2 = median, Q3 = 75th percentile

    double quartile_deviation(const double* data, size_t n);
    // QD = (Q3 - Q1) / 2

    // Coefficient of variation: CV = (σ / mean) × 100
    double coeff_variation(const double* data, size_t n);
}
```

### Skewness, Moments & Kurtosis

```cpp
namespace Sigma::Stats {
    // Skewness: measure of asymmetry
    // Positive skew: tail on right; Negative skew: tail on left
    double skewness_pearson(const double* data, size_t n);
    // = 3(mean - median) / stddev

    // Moments about mean (r-th central moment)
    double central_moment(const double* data, size_t n, int r);
    // μ_r = Σ(xi - mean)^r / n

    // Kurtosis: measure of "peakedness"
    double kurtosis(const double* data, size_t n);
    // β2 = μ4 / σ4  (normal dist = 3, mesokurtic)
    // Leptokurtic: β2 > 3 (heavy tails)
    // Platykurtic: β2 < 3 (light tails)
}
```

---

## Unit III: Correlation, Regression & Index Numbers

### Correlation Analysis

```cpp
namespace Sigma::Stats {

    // Pearson's correlation coefficient: r ∈ [-1, 1]
    double pearson_r(const double* x, const double* y, size_t n);
    // r = Σ[(xi-x̄)(yi-ȳ)] / sqrt[Σ(xi-x̄)² × Σ(yi-ȳ)²]

    // Spearman's rank correlation
    double spearman_r(const double* x, const double* y, size_t n);

    // Probable error of r
    double probable_error(double r, size_t n);
    // PE = 0.6745 × (1-r²) / sqrt(n)

    // r²: coefficient of determination
    double coeff_determination(double r);  // = r * r
}
```

**Interpretation:**

| r value | Correlation |
|---|---|
| +1.0 | Perfect positive |
| +0.7 to +0.9 | Strong positive |
| +0.4 to +0.6 | Moderate positive |
| 0 | No correlation |
| -0.4 to -0.6 | Moderate negative |
| -1.0 | Perfect negative |

### Regression Analysis

```cpp
namespace Sigma::Stats {

    struct RegressionLine {
        double slope;      // b
        double intercept;  // a
        // y = a + bx  (regression of y on x)
    };

    RegressionLine regression_y_on_x(const double* x, const double* y, size_t n);
    // b = [nΣxy - ΣxΣy] / [nΣx² - (Σx)²]
    // a = ȳ - b*x̄

    RegressionLine regression_x_on_y(const double* x, const double* y, size_t n);
    // Both lines pass through (x̄, ȳ)
    // byx * bxy = r²
}
```

### Index Numbers

| Type | Formula | SigmaOS Use |
|---|---|---|
| Simple Price Index | (Pn / P0) × 100 | Performance index vs baseline |
| Laspeyre's | ΣP1Q0 / ΣP0Q0 × 100 | Weighted by base year quantities |
| Paasche's | ΣP1Q1 / ΣP0Q1 × 100 | Weighted by current year quantities |
| Fisher's Ideal | √(Laspeyre × Paasche) | Best of both |

---

## Unit IV: Interpolation, Time Series & Probability

### Interpolation & Extrapolation

```cpp
namespace Sigma::Stats {

    // Newton's Forward Difference for equal intervals
    double interpolate_newton_forward(
        const double* x, const double* y, int n, double xi);

    // Lagrange interpolation for unequal intervals
    double interpolate_lagrange(
        const double* x, const double* y, int n, double xi);

    // Extrapolation: predict beyond known range (use with caution)
    double extrapolate(RegressionLine reg, double x_future);
}
```

### Time Series Analysis

```
Time Series = Trend (T) + Seasonal (S) + Cyclical (C) + Irregular (I)
```

```cpp
namespace Sigma::Stats {

    // Moving Average (trend isolation)
    std::vector<double> moving_average(const double* data, size_t n, int period);

    // Least Squares method for trend line
    RegressionLine trend_least_squares(const double* y, size_t n);

    // Seasonal Index
    double seasonal_index(const double* data, size_t n, int season_period);

    // SigmaOS use: CPU/memory usage forecasting
    ForecastResult forecast_next(const double* history, size_t n, int steps);
}
```

### Probability

```cpp
namespace Sigma::Math {

    // Classical: P(A) = favorable outcomes / total outcomes
    double classical_prob(int favorable, int total);

    // Addition theorem: P(A∪B) = P(A) + P(B) - P(A∩B)
    double prob_union(double pA, double pB, double pAB);

    // Multiplication theorem: P(A∩B) = P(A) × P(B|A)
    double prob_intersection(double pA, double pB_given_A);

    // Independent events: P(A∩B) = P(A) × P(B)
    double prob_independent(double pA, double pB);

    // Bayes' Theorem
    double bayes(double pA, double pB_given_A, double pB);
    // P(A|B) = P(B|A)×P(A) / P(B)

    // Used in SigmaAI: Naive Bayes classifier, anomaly detection
}
```

---

## SigmaStats Integration Map

```
SigmaStats Library
├── Central Tendency:  mean, median, mode, GM, HM
├── Dispersion:        range, MD, σ, σ², QD
├── Shape:             skewness, kurtosis, moments
├── Correlation:       Pearson r, Spearman r, PE
├── Regression:        y=a+bx, x=a+by, R²
├── Index Numbers:     Laspeyre, Paasche, Fisher
├── Interpolation:     Newton, Lagrange
├── Time Series:       MA, LSM, Seasonal Index
└── Probability:       Classical, Addition, Multiplication, Bayes

Consumers:
├── SigmaSheets (spreadsheet formulas)
├── SigmaViz (chart rendering)
├── SigmaAI (ML feature engineering)
└── SigmaDB (aggregate SQL functions)
```

**Files:** `userland/apps/SigmaStats/sigma_stats.cpp`, `sigma_stats.h`

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
