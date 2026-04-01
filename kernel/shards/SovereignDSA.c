/**
 * Σ SIGMAOS PROFESSIONAL KERNELS (v160.0)
 * Low-Level, Zero-Dependency, User-Defined Functions for DSA.
 * ACHIEVES PURE PERFORMANCE WITHOUT STANDARD LIBRARIES.
 */

#include "../SovereignOSBasicsZenith.h"

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
