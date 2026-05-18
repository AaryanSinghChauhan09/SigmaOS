/**
 * SigmaDB.cpp — Sovereign Relational Database Engine
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-RDBMS (SQL/PL-SQL, Codd's Rules, Normalization)
 * Userland application: SigmaDB SQL Engine
 *
 * Implements: SQL parser, query executor, transaction manager,
 *             cursor engine, trigger dispatcher, PL/SQL runtime stub.
 * Storage backend: SovereignZFSPool via VFS API.
 */
#include "SigmaDB.h"

namespace Sigma::DB {

// ─── SQL Data Type System ─────────────────────────────────────────────────────
// Supports: INT, BIGINT, VARCHAR(n), TEXT, DATE, TIMESTAMP, BOOLEAN,
//           DECIMAL(p,s), FLOAT, BLOB, JSON, UUID, VECTOR(n)

// ─── DDL Executor ─────────────────────────────────────────────────────────────

int SigmaDB::create_table(const TableSchema& schema) {
    // 1. Validate schema (1NF check: atomic values, no repeating groups)
    if (!validate_1nf(schema)) return SIGMA_DB_ERR_NOT_1NF;
    // 2. Check uniqueness of table name in sigma_catalog
    if (catalog_has_table(schema.name)) return SIGMA_DB_ERR_TABLE_EXISTS;
    // 3. Write schema to sigma_catalog.tables
    write_catalog_table(schema);
    // 4. Create physical storage extent in SovereignZFSPool
    storage_.create_extent(schema.name, schema.initial_pages);
    sigma_klog(LOG_INFO, "[SigmaDB] CREATE TABLE %s (%d columns)\n",
               schema.name, schema.column_count);
    return SIGMA_DB_OK;
}

int SigmaDB::alter_table(const char* table_name, const AlterOp& op) {
    if (!catalog_has_table(table_name)) return SIGMA_DB_ERR_NO_TABLE;
    switch (op.type) {
        case AlterType::ADD_COLUMN:    return add_column_impl(table_name, op.column); break;
        case AlterType::DROP_COLUMN:   return drop_column_impl(table_name, op.col_name); break;
        case AlterType::MODIFY_COLUMN: return modify_column_impl(table_name, op.column); break;
        case AlterType::RENAME_TABLE:  return rename_table_impl(table_name, op.new_name); break;
        default: return SIGMA_DB_ERR_UNKNOWN_OP;
    }
}

int SigmaDB::drop_table(const char* table_name, bool if_exists) {
    if (!catalog_has_table(table_name)) {
        if (if_exists) return SIGMA_DB_OK;
        return SIGMA_DB_ERR_NO_TABLE;
    }
    // Check for foreign key references pointing to this table
    if (has_dependent_fkeys(table_name)) return SIGMA_DB_ERR_FK_CONSTRAINT;
    drop_catalog_table(table_name);
    storage_.drop_extent(table_name);
    sigma_klog(LOG_INFO, "[SigmaDB] DROP TABLE %s\n", table_name);
    return SIGMA_DB_OK;
}

int SigmaDB::truncate_table(const char* table_name) {
    if (!catalog_has_table(table_name)) return SIGMA_DB_ERR_NO_TABLE;
    storage_.truncate_extent(table_name);  // Fast: reset extent, keep schema
    sigma_klog(LOG_INFO, "[SigmaDB] TRUNCATE TABLE %s\n", table_name);
    return SIGMA_DB_OK;
}

// ─── DML Executor ─────────────────────────────────────────────────────────────

int SigmaDB::insert(const char* table_name, const Row& row) {
    // 1. Validate constraints (NOT NULL, UNIQUE, CHECK, FK)
    int rc = validate_constraints(table_name, row, DMLOp::INSERT);
    if (rc != SIGMA_DB_OK) return rc;
    // 2. Write row to table extent
    storage_.append_row(table_name, row);
    // 3. Update all affected indexes
    update_indexes(table_name, row, DMLOp::INSERT);
    // 4. Fire AFTER INSERT triggers
    fire_triggers(table_name, TriggerEvent::INSERT, nullptr, &row);
    current_txn_.rows_affected++;
    return SIGMA_DB_OK;
}

int SigmaDB::update(const char* table_name, const Row& new_vals, const WhereClause& where) {
    sigma_u32 affected = 0;
    Cursor c = open_cursor(table_name);
    while (c.has_next()) {
        Row old_row = c.current();
        if (where_matches(old_row, where)) {
            Row updated = merge_row(old_row, new_vals);
            int rc = validate_constraints(table_name, updated, DMLOp::UPDATE);
            if (rc != SIGMA_DB_OK) { close_cursor(c); return rc; }
            c.update_current(updated);
            fire_triggers(table_name, TriggerEvent::UPDATE, &old_row, &updated);
            affected++;
        }
        c.advance();
    }
    close_cursor(c);
    current_txn_.rows_affected += affected;
    return SIGMA_DB_OK;
}

int SigmaDB::delete_rows(const char* table_name, const WhereClause& where) {
    sigma_u32 affected = 0;
    Cursor c = open_cursor(table_name);
    while (c.has_next()) {
        Row row = c.current();
        if (where_matches(row, where)) {
            c.delete_current();
            fire_triggers(table_name, TriggerEvent::DELETE, &row, nullptr);
            affected++;
        }
        c.advance();
    }
    close_cursor(c);
    current_txn_.rows_affected += affected;
    return SIGMA_DB_OK;
}

// ─── SELECT & Joins ───────────────────────────────────────────────────────────

ResultSet SigmaDB::select(const SelectQuery& q) {
    ResultSet rs;
    // 1. FROM clause: get base table rows
    rs = storage_.full_scan(q.table);
    // 2. JOINs (INNER, LEFT, RIGHT, FULL, SELF)
    for (const auto& join : q.joins) {
        rs = apply_join(rs, join);
    }
    // 3. WHERE clause: filter rows
    if (q.has_where) rs = filter_rows(rs, q.where);
    // 4. GROUP BY + HAVING
    if (q.has_group_by) {
        rs = group_rows(rs, q.group_by_cols, q.group_by_count);
        if (q.has_having) rs = filter_rows(rs, q.having);
    }
    // 5. SELECT list: project columns + aggregate functions
    rs = project_columns(rs, q.select_list, q.select_count);
    // 6. ORDER BY
    if (q.has_order_by) rs = sort_rows(rs, q.order_by, q.order_asc);
    // 7. DISTINCT
    if (q.distinct) rs = remove_duplicates(rs);
    return rs;
}

// ─── Set Operators ────────────────────────────────────────────────────────────
ResultSet SigmaDB::set_union(const ResultSet& a, const ResultSet& b, bool all) {
    ResultSet r = a;
    for (sigma_u32 i = 0; i < b.row_count; i++) {
        bool found = false;
        if (!all) {
            for (sigma_u32 j = 0; j < r.row_count && !found; j++)
                found = rows_equal(r.rows[j], b.rows[i]);
        }
        if (!found) append_row(r, b.rows[i]);
    }
    return r;
}
ResultSet SigmaDB::set_intersect(const ResultSet& a, const ResultSet& b) {
    ResultSet r; r.row_count = 0;
    for (sigma_u32 i = 0; i < a.row_count; i++)
        for (sigma_u32 j = 0; j < b.row_count; j++)
            if (rows_equal(a.rows[i], b.rows[j])) { append_row(r, a.rows[i]); break; }
    return r;
}
ResultSet SigmaDB::set_minus(const ResultSet& a, const ResultSet& b) {
    ResultSet r; r.row_count = 0;
    for (sigma_u32 i = 0; i < a.row_count; i++) {
        bool found = false;
        for (sigma_u32 j = 0; j < b.row_count && !found; j++)
            found = rows_equal(a.rows[i], b.rows[j]);
        if (!found) append_row(r, a.rows[i]);
    }
    return r;
}

// ─── Transaction Control Language (TCL) ──────────────────────────────────────

int SigmaDB::begin_transaction() {
    current_txn_.active    = true;
    current_txn_.txn_id    = ++txn_counter_;
    current_txn_.rows_affected = 0;
    storage_.create_snapshot(current_txn_.txn_id); // CoW snapshot for rollback
    sigma_klog(LOG_DEBUG, "[SigmaDB] BEGIN TRANSACTION txn_id=%u\n", current_txn_.txn_id);
    return SIGMA_DB_OK;
}

int SigmaDB::commit() {
    if (!current_txn_.active) return SIGMA_DB_ERR_NO_TXN;
    storage_.commit_snapshot(current_txn_.txn_id);
    sigma_klog(LOG_INFO, "[SigmaDB] COMMIT txn_id=%u, %u rows affected\n",
               current_txn_.txn_id, current_txn_.rows_affected);
    current_txn_.active = false;
    return SIGMA_DB_OK;
}

int SigmaDB::rollback(const char* savepoint_name) {
    if (!current_txn_.active) return SIGMA_DB_ERR_NO_TXN;
    if (savepoint_name) {
        // Partial rollback to savepoint
        Savepoint* sp = find_savepoint(savepoint_name);
        if (!sp) return SIGMA_DB_ERR_NO_SAVEPOINT;
        storage_.rollback_to_snapshot(sp->snapshot_id);
        sigma_klog(LOG_INFO, "[SigmaDB] ROLLBACK TO SAVEPOINT %s\n", savepoint_name);
    } else {
        // Full rollback
        storage_.rollback_snapshot(current_txn_.txn_id);
        current_txn_.active = false;
        sigma_klog(LOG_INFO, "[SigmaDB] ROLLBACK txn_id=%u\n", current_txn_.txn_id);
    }
    return SIGMA_DB_OK;
}

int SigmaDB::savepoint(const char* name) {
    Savepoint sp;
    sigma_strncpy(sp.name, name, sizeof(sp.name));
    sp.snapshot_id = storage_.create_snapshot(current_txn_.txn_id);
    savepoints_[savepoint_count_++] = sp;
    sigma_klog(LOG_DEBUG, "[SigmaDB] SAVEPOINT %s\n", name);
    return SIGMA_DB_OK;
}

// ─── DCL: GRANT / REVOKE ──────────────────────────────────────────────────────

int SigmaDB::grant(const char* user, const char* table_name, Permission perms) {
    acl_.add(user, table_name, perms);
    sigma_klog(LOG_INFO, "[SigmaDB] GRANT 0x%X ON %s TO %s\n",
               (unsigned)perms, table_name, user);
    return SIGMA_DB_OK;
}

int SigmaDB::revoke(const char* user, const char* table_name, Permission perms) {
    acl_.remove(user, table_name, perms);
    sigma_klog(LOG_INFO, "[SigmaDB] REVOKE 0x%X ON %s FROM %s\n",
               (unsigned)perms, table_name, user);
    return SIGMA_DB_OK;
}

// ─── Triggers ─────────────────────────────────────────────────────────────────

int SigmaDB::create_trigger(const Trigger& trig) {
    triggers_[trigger_count_++] = trig;
    sigma_klog(LOG_INFO, "[SigmaDB] CREATE TRIGGER %s ON %s\n",
               trig.name, trig.table_name);
    return SIGMA_DB_OK;
}

void SigmaDB::fire_triggers(const char* table, TriggerEvent event,
                             const Row* old_row, const Row* new_row) {
    for (sigma_u32 i = 0; i < trigger_count_; i++) {
        Trigger& t = triggers_[i];
        if (sigma_strcmp(t.table_name, table) == 0 && t.event == event) {
            t.handler(old_row, new_row);
        }
    }
}

// ─── Built-in Functions ───────────────────────────────────────────────────────
// These implement the Oracle/SQL built-in function catalog per Syllabus-RDBMS Unit III

namespace BuiltIn {
    // NUMERIC
    double fn_abs(double x)       { return x < 0 ? -x : x; }
    double fn_ceil(double x)      { return (double)((sigma_i64)x + (x > (sigma_i64)x ? 1 : 0)); }
    double fn_floor(double x)     { return (double)((sigma_i64)x - (x < (sigma_i64)x ? 1 : 0)); }
    double fn_round(double x, int d) {
        double m = 1.0; for (int i = 0; i < d; i++) m *= 10.0;
        return fn_floor(x * m + 0.5) / m;
    }
    double fn_sqrt(double x) {
        if (x < 0) return -1; // Error
        double g = x / 2.0;
        for (int i = 0; i < 100; i++) g = (g + x / g) / 2.0;
        return g;
    }
    double fn_power(double b, double e) {
        double r = 1.0;
        for (int i = 0; i < (int)e; i++) r *= b; return r;
    }
    sigma_i64 fn_mod(sigma_i64 a, sigma_i64 b) { return a % b; }

    // CHARACTER
    sigma_usize fn_length(const char* s) { sigma_usize n=0; while(s[n]) n++; return n; }
    void fn_upper(const char* src, char* dst) {
        while (*src) { *dst++ = (*src>='a'&&*src<='z') ? (*src-32) : *src; src++; } *dst='\0';
    }
    void fn_lower(const char* src, char* dst) {
        while (*src) { *dst++ = (*src>='A'&&*src<='Z') ? (*src+32) : *src; src++; } *dst='\0';
    }
    int fn_instr(const char* hay, const char* needle) {
        const char* p = hay; int pos = 1;
        while (*p) {
            const char* h = p, *n = needle;
            while (*h && *n && *h==*n) { h++; n++; }
            if (!*n) return pos;
            p++; pos++;
        }
        return 0;
    }
    void fn_substr(const char* src, int start, int len, char* dst) {
        src += (start - 1); // 1-indexed
        while (len-- > 0 && *src) *dst++ = *src++;
        *dst = '\0';
    }
    void fn_trim(const char* src, char* dst) {
        while (*src == ' ') src++;
        const char* end = src;
        while (*end) end++;
        while (end > src && *(end-1) == ' ') end--;
        sigma_usize n = (sigma_usize)(end - src);
        for (sigma_usize i = 0; i < n; i++) dst[i] = src[i];
        dst[n] = '\0';
    }
    void fn_nvl(const char* val, const char* default_val, char* dst) {
        const char* use = (val && *val) ? val : default_val;
        while (*use) *dst++ = *use++; *dst = '\0';
    }
} // namespace BuiltIn

} // namespace Sigma::DB
