/**
 * Σ SIGMAOS PROFESSIONAL KERNELS (v160.0)
 * Low-Level, Zero-Dependency, User-Defined Functions.
 * ACHIEVES PURE PERFORMANCE WITHOUT STANDARD LIBRARIES.
 */

#include "SovereignOSBasicsZenith.h"

/* --- USER-DEFINED MATH FUNCTIONS (REPLACING MATH.H) --- */

float sigma_abs(float x) {
    return (x < 0) ? -x : x;
}

float sigma_pow(float base, int exp) {
    float res = 1.0;
    for (int i = 0; i < exp; i++) res *= base;
    return res;
}

/* --- USER-DEFINED AI KERNELS --- */

/**
 * SIGMA_GRADIENT_DESCENT
 * Executes raw silicon linear regression.
 */
void sigma_gradient_descent(float* x, float* y, int n, float* w, float* b, float alpha, int epochs) {
    for (int i = 0; i < epochs; i++) {
        float dw = 0;
        float db = 0;
        for (int j = 0; j < n; j++) {
            float pred = (*w) * x[j] + (*b);
            dw += (pred - y[j]) * x[j];
            db += (pred - y[j]);
        }
        *w -= (dw / n) * alpha;
        *b -= (db / n) * alpha;
    }
}

/* --- USER-DEFINED DSA KERNELS --- */

/**
 * SIGMA_QUICKSORT
 * In-place sorting without stdlib.h qsort.
 */
void sigma_swap(int* a, int* b) {
    int t = *a;
    *a = *b;
    *b = t;
}

int sigma_partition(int* arr, int low, int high) {
    int pivot = arr[high];
    int i = (low - 1);
    for (int j = low; j <= high - 1; j++) {
        if (arr[j] < pivot) {
            i++;
            sigma_swap(&arr[i], &arr[j]);
        }
    }
    sigma_swap(&arr[i + 1], &arr[high]);
    return (i + 1);
}

void sigma_quicksort(int* arr, int low, int high) {
    if (low < high) {
        int pi = sigma_partition(arr, low, high);
        sigma_quicksort(arr, low, pi - 1);
        sigma_quicksort(arr, pi + 1, high);
    }
}

/* --- USER-DEFINED DATA SCIENCE KERNELS --- */

/**
 * SIGMA_STAT_ANALYSIS
 * Performance variance and mean calculation.
 */
void sigma_stat_analysis(float* data, int n, float* mean, float* variance) {
    float sum = 0;
    for (int i = 0; i < n; i++) sum += data[i];
    *mean = sum / n;

    float v_sum = 0;
    for (int i = 0; i < n; i++) {
        v_sum += sigma_pow(data[i] - (*mean), 2);
    }
    *variance = v_sum / n;
}

/* --- USER-DEFINED CYBER SECURITY KERNELS --- */

/**
 * SIGMA_PATTERN_AUDIT
 * Manual string pattern matching without string.h.
 */
int sigma_str_contains(const char* str, const char* pattern) {
    if (!*pattern) return 1;
    for (const char* p = str; *p; p++) {
        const char* p1 = p;
        const char* p2 = pattern;
        while (*p1 && *p2 && *p1 == *p2) {
            p1++;
            p2++;
        }
        if (!*p2) return 1;
    }
    return 0;
}
