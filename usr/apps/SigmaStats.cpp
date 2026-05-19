/**
 * SigmaStats.cpp — Statistical Analysis Toolkit
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-Statistics (Central Tendency, Dispersion,
 *          Correlation, Regression, Probability, Time Series)
 */
#include "SigmaStats.h"

namespace Sigma::Stats {

// ─── Internal kernel math (no libm) ──────────────────────────────────────────
static double sigma_sqrt(double x) {
    if (x <= 0.0) return 0.0;
    double g = x / 2.0;
    for (int i = 0; i < 64; i++) g = (g + x / g) / 2.0;
    return g;
}
static double sigma_fabs(double x) { return x < 0 ? -x : x; }
static double sigma_pow(double b, int e) {
    double r = 1.0; for (int i = 0; i < e; i++) r *= b; return r;
}

// ─── Unit II: Central Tendency ────────────────────────────────────────────────

double arithmetic_mean(const double* data, sigma_usize n) {
    if (!n) return 0.0;
    double sum = 0.0;
    for (sigma_usize i = 0; i < n; i++) sum += data[i];
    return sum / (double)n;
}

double median(double* data, sigma_usize n) {
    // Simple insertion sort (kernel — no qsort)
    for (sigma_usize i = 1; i < n; i++) {
        double key = data[i];
        sigma_usize j = i;
        while (j > 0 && data[j-1] > key) { data[j] = data[j-1]; j--; }
        data[j] = key;
    }
    if (n % 2 == 1) return data[n / 2];
    return (data[n/2 - 1] + data[n/2]) / 2.0;
}

double mode(const double* data, sigma_usize n) {
    double best_val = data[0];
    sigma_usize best_cnt = 0;
    for (sigma_usize i = 0; i < n; i++) {
        sigma_usize cnt = 0;
        for (sigma_usize j = 0; j < n; j++) if (data[j] == data[i]) cnt++;
        if (cnt > best_cnt) { best_cnt = cnt; best_val = data[i]; }
    }
    return best_val;
}

double geometric_mean(const double* data, sigma_usize n) {
    // GM = (x1*x2*...*xn)^(1/n) = exp(mean(log(xi)))
    // Compute via nth root using Newton
    double product = 1.0;
    for (sigma_usize i = 0; i < n; i++) product *= data[i];
    // nth root via Newton: x^(1/n) = x^(1/n)
    double r = product;
    for (int iter = 0; iter < 100; iter++) {
        double rn1 = 1.0; for (sigma_usize i = 1; i < n; i++) rn1 *= r;
        r = r - (rn1 * r - product) / ((double)n * rn1);
    }
    return r;
}

double harmonic_mean(const double* data, sigma_usize n) {
    if (!n) return 0.0;
    double sum_inv = 0.0;
    for (sigma_usize i = 0; i < n; i++) {
        if (data[i] == 0.0) return 0.0; // undefined
        sum_inv += 1.0 / data[i];
    }
    return (double)n / sum_inv;
}

// ─── Unit II: Dispersion ─────────────────────────────────────────────────────

double range_stat(const double* data, sigma_usize n) {
    if (!n) return 0.0;
    double mn = data[0], mx = data[0];
    for (sigma_usize i = 1; i < n; i++) {
        if (data[i] < mn) mn = data[i];
        if (data[i] > mx) mx = data[i];
    }
    return mx - mn;
}

double mean_deviation(const double* data, sigma_usize n) {
    double mean = arithmetic_mean(data, n);
    double sum = 0.0;
    for (sigma_usize i = 0; i < n; i++) sum += sigma_fabs(data[i] - mean);
    return sum / (double)n;
}

double variance(const double* data, sigma_usize n) {
    double mean = arithmetic_mean(data, n);
    double sum = 0.0;
    for (sigma_usize i = 0; i < n; i++) {
        double d = data[i] - mean;
        sum += d * d;
    }
    return sum / (double)n;
}

double std_deviation(const double* data, sigma_usize n) {
    return sigma_sqrt(variance(data, n));
}

double quartile(const double* sorted, sigma_usize n, int q) {
    // q=1 → Q1 (25th), q=2 → median, q=3 → Q3 (75th)
    double pos = (q * (n + 1)) / 4.0;
    sigma_usize lo = (sigma_usize)pos;
    double frac = pos - lo;
    if (lo == 0) return sorted[0];
    if (lo >= n) return sorted[n-1];
    return sorted[lo-1] + frac * (sorted[lo] - sorted[lo-1]);
}

double quartile_deviation(const double* data, sigma_usize n) {
    // Make a sorted copy (in-place sort on local copy)
    double buf[4096];
    if (n > 4096) n = 4096;
    for (sigma_usize i = 0; i < n; i++) buf[i] = data[i];
    // insertion sort
    for (sigma_usize i = 1; i < n; i++) {
        double key = buf[i]; sigma_usize j = i;
        while (j > 0 && buf[j-1] > key) { buf[j] = buf[j-1]; j--; }
        buf[j] = key;
    }
    double q1 = quartile(buf, n, 1);
    double q3 = quartile(buf, n, 3);
    return (q3 - q1) / 2.0;
}

// ─── Unit II: Skewness & Kurtosis ────────────────────────────────────────────

double central_moment(const double* data, sigma_usize n, int r) {
    double mean = arithmetic_mean(data, n);
    double sum = 0.0;
    for (sigma_usize i = 0; i < n; i++) sum += sigma_pow(data[i] - mean, r);
    return sum / (double)n;
}

double skewness_pearson(const double* data, sigma_usize n) {
    double mean = arithmetic_mean(data, n);
    double med  = 0.0; // Would need sorted copy
    double sd   = std_deviation(data, n);
    // Pearson's coefficient: 3(mean - median) / stddev
    // Using mode approximation: mode ≈ 3*median - 2*mean
    return 3.0 * (mean - med) / (sd + 1e-9);
}

double kurtosis(const double* data, sigma_usize n) {
    double mu4 = central_moment(data, n, 4);
    double sd  = std_deviation(data, n);
    double sd4 = sd * sd * sd * sd;
    if (sd4 < 1e-9) return 0.0;
    return mu4 / sd4;  // β2 = 3 for normal dist (mesokurtic)
}

// ─── Unit III: Correlation & Regression ──────────────────────────────────────

double pearson_r(const double* x, const double* y, sigma_usize n) {
    double xm = arithmetic_mean(x, n);
    double ym = arithmetic_mean(y, n);
    double num = 0.0, dx2 = 0.0, dy2 = 0.0;
    for (sigma_usize i = 0; i < n; i++) {
        double dx = x[i] - xm, dy = y[i] - ym;
        num += dx * dy;
        dx2 += dx * dx;
        dy2 += dy * dy;
    }
    double denom = sigma_sqrt(dx2 * dy2);
    if (denom < 1e-9) return 0.0;
    return num / denom;
}

double probable_error(double r, sigma_usize n) {
    // PE = 0.6745 * (1 - r²) / √n
    return 0.6745 * (1.0 - r * r) / sigma_sqrt((double)n);
}

double coeff_determination(double r) { return r * r; }

RegressionLine regression_y_on_x(const double* x, const double* y, sigma_usize n) {
    // y = a + bx where:
    // b = [nΣxy - ΣxΣy] / [nΣx² - (Σx)²]
    // a = ȳ - b*x̄
    double sx=0, sy=0, sxy=0, sx2=0;
    for (sigma_usize i = 0; i < n; i++) {
        sx += x[i]; sy += y[i];
        sxy += x[i]*y[i]; sx2 += x[i]*x[i];
    }
    double dn = (double)n;
    double b = (dn*sxy - sx*sy) / (dn*sx2 - sx*sx);
    double a = (sy - b*sx) / dn;
    return { .slope=b, .intercept=a };
}

// ─── Unit IV: Probability ─────────────────────────────────────────────────────

double classical_prob(int favorable, int total) {
    if (total == 0) return 0.0;
    return (double)favorable / (double)total;
}

double prob_union(double pA, double pB, double pAB) {
    return pA + pB - pAB;  // Addition theorem
}

double prob_intersection(double pA, double pB_given_A) {
    return pA * pB_given_A;  // Multiplication theorem: P(A∩B) = P(A)·P(B|A)
}

double prob_independent(double pA, double pB) {
    return pA * pB;  // Independent: P(A∩B) = P(A)·P(B)
}

double bayes(double pA, double pB_given_A, double pB) {
    // P(A|B) = P(B|A)·P(A) / P(B)
    if (pB < 1e-9) return 0.0;
    return (pB_given_A * pA) / pB;
}

// ─── Unit IV: Time Series ─────────────────────────────────────────────────────

sigma_usize moving_average(const double* data, sigma_usize n, int period,
                            double* out) {
    if ((sigma_usize)period > n) return 0;
    sigma_usize out_count = n - (sigma_usize)period + 1;
    for (sigma_usize i = 0; i < out_count; i++) {
        double sum = 0.0;
        for (int j = 0; j < period; j++) sum += data[i + j];
        out[i] = sum / (double)period;
    }
    return out_count;
}

RegressionLine trend_least_squares(const double* y, sigma_usize n) {
    // x values: 1, 2, ..., n (time index)
    double sx=0, sy=0, sxy=0, sx2=0;
    for (sigma_usize i = 0; i < n; i++) {
        double xi = (double)(i + 1);
        sx += xi; sy += y[i];
        sxy += xi * y[i]; sx2 += xi * xi;
    }
    double dn = (double)n;
    double b = (dn*sxy - sx*sy) / (dn*sx2 - sx*sx);
    double a = (sy - b*sx) / dn;
    return { .slope=b, .intercept=a };
}

} // namespace Sigma::Stats
