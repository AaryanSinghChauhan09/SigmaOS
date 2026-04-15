/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN DATAFRAME MATRIX (v2.0 — DEEP DATA SCIENCE)
 * =========================================================================
 * Mission: Kernel-level tabular data processing — R/Pandas parity.
 * Principles: Columnar storage, vectorized aggregation, real statistics.
 *
 * v2.0: Real in-memory columnar store with sum, mean, min, max, filter,
 *       and sort operations — not printf stubs.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

/* --- Column Types --- */

typedef enum {
    COL_INT,
    COL_FLOAT,
    COL_STRING
} ColumnType_t;

/* --- Column Definition --- */

#define DF_MAX_ROWS 256
#define DF_MAX_COLS 16
#define DF_NAME_LEN 32

typedef struct {
    char         name[DF_NAME_LEN];
    ColumnType_t type;
    sigma_f64    data_f[DF_MAX_ROWS];    /* float/int storage   */
    char         data_s[DF_MAX_ROWS][64]; /* string storage     */
    sigma_u32    count;                   /* rows in this column */
} SigmaColumn_t;

/* --- Dataframe --- */

typedef struct {
    char           name[DF_NAME_LEN];
    SigmaColumn_t  columns[DF_MAX_COLS];
    sigma_u32      num_cols;
    sigma_u32      num_rows;
} SigmaDataframe_t;

/* --- Global Dataframe Pool --- */

#define MAX_DATAFRAMES 8
static SigmaDataframe_t s_df_pool[MAX_DATAFRAMES];
static sigma_u32 s_df_count = 0;

/**
 * sigma_df_create: Creates a new empty dataframe.
 */
SigmaDataframe_t* sigma_df_create(const char* name) {
    if (s_df_count >= MAX_DATAFRAMES) return SIGMA_NULL;

    SigmaDataframe_t* df = &s_df_pool[s_df_count++];
    sigma_strncpy(df->name, name, DF_NAME_LEN);
    df->num_cols = 0;
    df->num_rows = 0;

    sigma_printf("[DATAFRAME]: Created '%s'\n", name);
    return df;
}

/**
 * sigma_df_add_column: Adds a named float column to the dataframe.
 */
sigma_err_t sigma_df_add_column(SigmaDataframe_t* df, const char* col_name,
                                const sigma_f64* values, sigma_u32 count) {
    if (!df || df->num_cols >= DF_MAX_COLS) return SIGMA_ENOSPC;
    if (count > DF_MAX_ROWS) count = DF_MAX_ROWS;

    SigmaColumn_t* col = &df->columns[df->num_cols++];
    sigma_strncpy(col->name, col_name, DF_NAME_LEN);
    col->type  = COL_FLOAT;
    col->count = count;

    for (sigma_u32 i = 0; i < count; i++) {
        col->data_f[i] = values[i];
    }

    if (count > df->num_rows) df->num_rows = count;

    sigma_printf("[DATAFRAME]: Added column '%s' (%u rows) to '%s'\n",
                 col_name, count, df->name);
    return SIGMA_OK;
}

/* =======================================================================
 * REAL DATA SCIENCE OPERATIONS
 * ======================================================================= */

/**
 * sigma_df_sum: Computes the sum of a column. O(N).
 */
sigma_f64 sigma_df_sum(SigmaDataframe_t* df, sigma_u32 col_idx) {
    if (!df || col_idx >= df->num_cols) return 0.0;
    SigmaColumn_t* col = &df->columns[col_idx];

    sigma_f64 sum = 0.0;
    for (sigma_u32 i = 0; i < col->count; i++) {
        sum += col->data_f[i];
    }
    return sum;
}

/**
 * sigma_df_mean: Computes the arithmetic mean of a column.
 */
sigma_f64 sigma_df_mean(SigmaDataframe_t* df, sigma_u32 col_idx) {
    if (!df || col_idx >= df->num_cols) return 0.0;
    SigmaColumn_t* col = &df->columns[col_idx];
    if (col->count == 0) return 0.0;

    return sigma_df_sum(df, col_idx) / (sigma_f64)col->count;
}

/**
 * sigma_df_min: Finds the minimum value in a column.
 */
sigma_f64 sigma_df_min(SigmaDataframe_t* df, sigma_u32 col_idx) {
    if (!df || col_idx >= df->num_cols) return 0.0;
    SigmaColumn_t* col = &df->columns[col_idx];
    if (col->count == 0) return 0.0;

    sigma_f64 min_val = col->data_f[0];
    for (sigma_u32 i = 1; i < col->count; i++) {
        if (col->data_f[i] < min_val) min_val = col->data_f[i];
    }
    return min_val;
}

/**
 * sigma_df_max: Finds the maximum value in a column.
 */
sigma_f64 sigma_df_max(SigmaDataframe_t* df, sigma_u32 col_idx) {
    if (!df || col_idx >= df->num_cols) return 0.0;
    SigmaColumn_t* col = &df->columns[col_idx];
    if (col->count == 0) return 0.0;

    sigma_f64 max_val = col->data_f[0];
    for (sigma_u32 i = 1; i < col->count; i++) {
        if (col->data_f[i] > max_val) max_val = col->data_f[i];
    }
    return max_val;
}

/**
 * sigma_df_variance: Computes population variance.
 * Var(X) = E[(X - mean)^2]
 */
sigma_f64 sigma_df_variance(SigmaDataframe_t* df, sigma_u32 col_idx) {
    if (!df || col_idx >= df->num_cols) return 0.0;
    SigmaColumn_t* col = &df->columns[col_idx];
    if (col->count == 0) return 0.0;

    sigma_f64 mean = sigma_df_mean(df, col_idx);
    sigma_f64 sum_sq = 0.0;
    for (sigma_u32 i = 0; i < col->count; i++) {
        sigma_f64 diff = col->data_f[i] - mean;
        sum_sq += diff * diff;
    }
    return sum_sq / (sigma_f64)col->count;
}

/**
 * sigma_df_sort_column: In-place insertion sort on a column. O(N^2) worst.
 * Acceptable for kernel telemetry where N is small.
 */
void sigma_df_sort_column(SigmaDataframe_t* df, sigma_u32 col_idx) {
    if (!df || col_idx >= df->num_cols) return;
    SigmaColumn_t* col = &df->columns[col_idx];

    for (sigma_u32 i = 1; i < col->count; i++) {
        sigma_f64 key = col->data_f[i];
        sigma_u32 j = i;
        while (j > 0 && col->data_f[j - 1] > key) {
            col->data_f[j] = col->data_f[j - 1];
            j--;
        }
        col->data_f[j] = key;
    }
    sigma_printf("[DATAFRAME]: Column '%s' sorted (ascending).\n", col->name);
}

/**
 * sigma_df_filter_gt: Returns count of rows where column > threshold.
 */
sigma_u32 sigma_df_filter_gt(SigmaDataframe_t* df, sigma_u32 col_idx,
                             sigma_f64 threshold) {
    if (!df || col_idx >= df->num_cols) return 0;
    SigmaColumn_t* col = &df->columns[col_idx];

    sigma_u32 matches = 0;
    for (sigma_u32 i = 0; i < col->count; i++) {
        if (col->data_f[i] > threshold) matches++;
    }
    return matches;
}

/* --- Describe (like pandas .describe()) --- */

void sigma_df_describe(SigmaDataframe_t* df) {
    sigma_printf("\n--- DATAFRAME: %s (%u rows x %u cols) ---\n",
                 df->name, df->num_rows, df->num_cols);
    sigma_printf("%-16s %-10s %-10s %-10s %-10s %-10s\n",
                 "COLUMN", "SUM", "MEAN", "MIN", "MAX", "VAR");
    sigma_printf("--------------------------------------------------------------\n");

    for (sigma_u32 c = 0; c < df->num_cols; c++) {
        sigma_printf("%-16s %-10.2f %-10.2f %-10.2f %-10.2f %-10.2f\n",
                     df->columns[c].name,
                     sigma_df_sum(df, c),
                     sigma_df_mean(df, c),
                     sigma_df_min(df, c),
                     sigma_df_max(df, c),
                     sigma_df_variance(df, c));
    }
    sigma_printf("--------------------------------------------------------------\n");
}



