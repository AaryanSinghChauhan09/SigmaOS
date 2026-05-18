/**
 * SigmaStats.h — Statistical Analysis Toolkit Header
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-Statistics (Central Tendency, Dispersion,
 *          Correlation, Regression, Probability, Time Series)
 */
#pragma once
#include "../../include/core/sigma_kernel_types.h"

namespace Sigma::Stats {

// ─── Regression Structure ─────────────────────────────────────────────────────
struct RegressionLine {
    double slope;      // b
    double intercept;  // a
};

// ─── Unit II: Central Tendency ────────────────────────────────────────────────
double arithmetic_mean(const double* data, sigma_usize n);
double median(double* data, sigma_usize n); // May modify array order
double mode(const double* data, sigma_usize n);
double geometric_mean(const double* data, sigma_usize n);
double harmonic_mean(const double* data, sigma_usize n);

// ─── Unit II: Dispersion ─────────────────────────────────────────────────────
double range_stat(const double* data, sigma_usize n);
double mean_deviation(const double* data, sigma_usize n);
double variance(const double* data, sigma_usize n);
double std_deviation(const double* data, sigma_usize n);
double quartile(const double* sorted, sigma_usize n, int q); // q=1(Q1), q=2(Med), q=3(Q3)
double quartile_deviation(const double* data, sigma_usize n);

// ─── Unit II: Skewness & Kurtosis ────────────────────────────────────────────
double central_moment(const double* data, sigma_usize n, int r);
double skewness_pearson(const double* data, sigma_usize n);
double kurtosis(const double* data, sigma_usize n);

// ─── Unit III: Correlation & Regression ──────────────────────────────────────
double pearson_r(const double* x, const double* y, sigma_usize n);
double probable_error(double r, sigma_usize n);
double coeff_determination(double r);
RegressionLine regression_y_on_x(const double* x, const double* y, sigma_usize n);

// ─── Unit IV: Probability ─────────────────────────────────────────────────────
double classical_prob(int favorable, int total);
double prob_union(double pA, double pB, double pAB);
double prob_intersection(double pA, double pB_given_A);
double prob_independent(double pA, double pB);
double bayes(double pA, double pB_given_A, double pB);

// ─── Unit IV: Time Series ─────────────────────────────────────────────────────
sigma_usize moving_average(const double* data, sigma_usize n, int period, double* out);
RegressionLine trend_least_squares(const double* y, sigma_usize n);

} // namespace Sigma::Stats
