# Statistics → SigmaStats Toolkit

> Maps the Statistics syllabus to `SigmaStats` — SigmaOS's built-in statistical analysis library, integrated with SigmaViz, SigmaAI, and silicon-direct NPU sharding.

---

## Unit I: Introduction & Data Collection

### What is Statistics?

Statistics is the mathematical science of collecting, organizing, analyzing, interpreting, and presenting empirical data. In sovereign computing, statistics provides the quantitative rigor required for deterministic decision-making, predictive caching, and hardware failure modeling.

**Unique Selling Point (USP):** Turning raw data into actionable insights with absolute quantitative rigor for decision-making.

**SigmaOS Integration:** `SigmaStats` provides freestanding, kernel-level statistical primitives used by:

- `SigmaViz` — direct framebuffer chart rendering
- `SigmaAI` — ML feature engineering and normalization
- `SigmaSheets` — spreadsheet calculation engine
- `SigmaDB` — columnar aggregate query functions

### Primary vs Secondary Data

| Type | Source | SigmaOS Analogy | 
| :--- | :--- | :--- | 
| **Primary** | Collected firsthand (surveys, hardware sensors) | Live telemetry from SigmaOS HAL thermal/voltage sensors | 
| **Secondary** | Pre-collected (databases, historical logs) | Historical time-series metrics from SigmaDB | 

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

### Central Tendency & Mathematical Formulas

The foundational formulas for population mean ($\mu$) and sample arithmetic mean ($\bar{x}$) represent the center of mass for arbitrary data distributions:

$$\mu = \frac{\sum x_i}{n}, \quad \bar{x} = \frac{\sum_{i=1}^n x_i}{n}$$

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

### Measures of Dispersion & Variance Formula

Dispersion quantifies the spread or variability of data around the central tendency. The foundational formula for population variance ($\sigma^2$) and standard deviation ($\sigma$) is defined as:

$$\sigma^2 = \frac{\sum (x_i - \mu)^2}{n}, \quad \sigma = \sqrt{\frac{\sum (x_i - \mu)^2}{n}}$$

```cpp
namespace Sigma::Stats {

    double range(const double* data, size_t n);  // max - min

    double mean_deviation(const double* data, size_t n);
    // MD = Σ | xi - mean | / n

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

## Unit III: Hypothesis Testing, Regression & Bayesian Inference

### Hypothesis Testing & Probability Distributions

Hypothesis testing provides a rigorous mathematical framework for validating assumptions regarding population parameters using sample data.

* **Null Hypothesis ($H_0$):** Assumes no effect or no difference between groups.
* **Alternative Hypothesis ($H_1$):** Assumes a statistically significant effect or difference exists.
* **Probability Distributions:** Evaluates test statistics against established theoretical distributions including Normal ($Z$), Student's $t$, Chi-Square ($\chi^2$), and $F$-distributions.

```cpp
namespace Sigma::Stats {
    // Independent Two-Sample T-Test
    double t_test_independent(const double* group1, size_t n1, const double* group2, size_t n2);

    // Chi-Square Test of Independence
    double chi_square_test(const double* observed, const double* expected, size_t k);

    // One-Way ANOVA F-Test
    double anova_one_way(const double* g1, const double* g2, const double* g3, size_t n);
}
```

### Bayesian Inference

Bayesian inference updates the probability of a hypothesis ($H$) as more evidence or information ($E$) becomes available, combining prior beliefs with current likelihoods:

$$P(H | E) = \frac{P(E | H) \cdot P(H)}{P(E)}$$

```cpp
namespace Sigma::Stats {
    // Bayesian Posterior Probability Calculator
    double bayesian_posterior(double prior, double likelihood, double marginal_likelihood) {
        return (marginal_likelihood > 0.0) ? ((likelihood * prior) / marginal_likelihood) : prior;
    }
}
```

### Correlation & Regression Analysis

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

    struct RegressionLine {
        double slope;      // b
        double intercept;  // a
        // y = a + bx  (regression of y on x)
    };

    RegressionLine regression_y_on_x(const double* x, const double* y, size_t n);
    // b = [nΣxy - ΣxΣy] / [nΣx² - (Σx)²]
    // a = ȳ - b*x̄
}
```

### Interpretation

| r value | Correlation | 
| :--- | :--- | 
| +1.0 | Perfect positive | 
| +0.7 to +0.9 | Strong positive | 
| +0.4 to +0.6 | Moderate positive | 
| 0 | No correlation | 
| -0.4 to -0.6 | Moderate negative | 
| -1.0 | Perfect negative | 

---

## Unit IV: Interpolation, Time Series & Tools

### Interpolation & Extrapolation

```cpp
namespace Sigma::Stats {

    // Newton's Forward Difference for equal intervals
    double interpolate_newton_forward(const double* x, const double* y, int n, double xi);

    // Lagrange interpolation for unequal intervals
    double interpolate_lagrange(const double* x, const double* y, int n, double xi);

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
}
```

### Tools & Ecosystem Parity

SigmaStats provides seamless bridging to industry-standard data science and statistical toolkits:

* **R Environment:** Full compatibility with R dataframe structures and CRAN analytical packages.
* **Python Ecosystem:** Direct interoperability with `NumPy` array buffers, `Pandas` series, and `Matplotlib` plotting backends.
* **Jupyter Notebooks:** Interactive kernel execution for real-time exploratory data analysis (EDA).

---

## Debugging & Problem-Solving in Statistics

### Common Issues & Fix Strategies

* **Issue - Overfitting in Statistical/ML Models:** Models capture noise rather than underlying distributions, leading to high training accuracy but poor generalization.
  * *Fix Strategy:* Apply $L_1$ (Lasso) or $L_2$ (Ridge) regularization penalties, increase k-fold cross-validation partitions, or prune decision tree depths.
* **Issue - Data Problems (Missing Values & Scaling):** Unclean datasets skew mean/variance calculations and cause gradient explosion.
  * *Fix Strategy:* Implement robust imputation (k-NN or median replacement) for missing values, and normalize/standardize features using Z-score transformations ($z = \frac{x - \mu}{\sigma}$).
* **Issue - Algorithmic Complexity Bottlenecks:** Naive sorting or pairwise distance calculations yield $O(n^2)$ complexity, stalling large dataset ingestion.
  * *Fix Strategy:* Optimize algorithmic complexity by replacing naive bubble/insertion sorts with QuickSort or MergeSort ($O(n \log n)$), and utilize spatial B+ Trees or k-d trees for searching.
* **Issue - Runtime Errors:** Unhandled exceptions during matrix inversion or floating-point division by zero.
  * *Fix Strategy:* Use kernel-level logging (`sigma_log`), exhaustive unit testing suites, and hardware profiling tools (DTrace/perf) to trace execution bottlenecks.

---

## SigmaStats Integration Map

```
SigmaStats Library
├── Central Tendency:  mean, median, mode, GM, HM
├── Dispersion:        range, MD, σ, σ², QD
├── Shape:             skewness, kurtosis, moments
├── Correlation:       Pearson r, Spearman r, PE
├── Regression:        y=a+bx, x=a+by, R²
├── Hypothesis:        T-Test, Chi-Square, ANOVA, Bayes
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
*Last updated: 2026-05-19 | SigmaOS Zenith v15.2*
