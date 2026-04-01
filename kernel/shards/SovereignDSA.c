/*
 * Σ SHARD: SOVEREIGN-DSA — Algorithms v2.0
 * Doctrine: Pure C11. No stdlib. All UDF.
 * Provides: Quicksort, Binary Search, BFS (adjacency bitmap).
 */
#include "../sigma_kernel_types.h"

/* --- Quicksort (in-place, Hoare partition, u32 array) --- */
static inline void _qs_swap(u32* a, u32* b) { u32 t = *a; *a = *b; *b = t; }

static void sigma_quicksort(u32* arr, i32 lo, i32 hi) {
    if (lo >= hi) return;
    u32 pivot = arr[(u32)(lo + hi) / 2];
    i32 i = lo - 1, j = hi + 1;
    while (TRUE) {
        do { i++; } while (arr[i] < pivot);
        do { j--; } while (arr[j] > pivot);
        if (i >= j) break;
        _qs_swap(&arr[i], &arr[j]);
    }
    sigma_quicksort(arr, lo, j);
    sigma_quicksort(arr, j + 1, hi);
}

/* --- Binary Search (sorted u32 array) — returns index or -1 --- */
static i32 sigma_bsearch(const u32* arr, u32 n, u32 target) {
    i32 lo = 0, hi = (i32)n - 1;
    while (lo <= hi) {
        i32 mid = lo + (hi - lo) / 2;
        if (arr[mid] == target) return mid;
        if (arr[mid] < target)  lo = mid + 1;
        else                    hi = mid - 1;
    }
    return -1;
}

/* --- BFS on adjacency bitmap (up to 32 nodes) --- */
#define BFS_MAX_NODES 32
typedef struct { u32 adj[BFS_MAX_NODES]; u32 n; } SigmaGraph;

static void sigma_bfs(SigmaGraph* g, u32 start, u8* visited) {
    u8 q[BFS_MAX_NODES]; u32 head = 0, tail = 0;
    for (u32 i = 0; i < g->n; i++) visited[i] = 0;
    visited[start] = 1;
    q[tail++] = (u8)start;
    while (head != tail) {
        u32 node = q[head++];
        for (u32 nb = 0; nb < g->n; nb++) {
            if ((g->adj[node] & (1u << nb)) && !visited[nb]) {
                visited[nb] = 1;
                q[tail++] = (u8)nb;
            }
        }
    }
}

/* --- Merge Sort (stable, u32 array, scratch buffer needed) --- */
static void _merge(u32* a, u32* tmp, u32 lo, u32 mid, u32 hi) {
    u32 i = lo, j = mid, k = lo;
    while (i < mid && j < hi) { tmp[k++] = (a[i] <= a[j]) ? a[i++] : a[j++]; }
    while (i < mid) tmp[k++] = a[i++];
    while (j < hi)  tmp[k++] = a[j++];
    sigma_memcpy(a + lo, tmp + lo, (hi - lo) * sizeof(u32));
}
static void sigma_mergesort(u32* a, u32* tmp, u32 n) {
    for (u32 w = 1; w < n; w *= 2) {
        for (u32 lo = 0; lo < n; lo += 2 * w) {
            u32 mid = lo + w < n ? lo + w : n;
            u32 hi  = lo + 2*w < n ? lo + 2*w : n;
            _merge(a, tmp, lo, mid, hi);
        }
    }
}
