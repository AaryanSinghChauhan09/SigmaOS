/**
 * SigmaDB.h — Sovereign Relational Database Engine Header
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-RDBMS (SQL/PL-SQL, Codd's Rules, Normalization)
 */
#pragma once
#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "sigma_string.h"

namespace Sigma::DB {

// ─── Constants & Error Codes ──────────────────────────────────────────────────
constexpr int SIGMA_DB_OK = 0;
constexpr int SIGMA_DB_ERR_NOT_1NF = -1;
constexpr int SIGMA_DB_ERR_TABLE_EXISTS = -2;
constexpr int SIGMA_DB_ERR_NO_TABLE = -3;
constexpr int SIGMA_DB_ERR_FK_CONSTRAINT = -4;
constexpr int SIGMA_DB_ERR_UNKNOWN_OP = -5;
constexpr int SIGMA_DB_ERR_NO_TXN = -6;
constexpr int SIGMA_DB_ERR_NO_SAVEPOINT = -7;
constexpr int SIGMA_DB_ERR_CONSTRAINT_VIOLATION = -8;

// ─── SQL Data Types ───────────────────────────────────────────────────────────
enum class ColType : sigma_u8 {
    INT, BIGINT, VARCHAR, TEXT, DATE, TIMESTAMP, BOOLEAN, DECIMAL, FLOAT, BLOB, JSON, UUID, VECTOR
};

struct ColumnDef {
    char name[64];
    ColType type;
    sigma_u32 max_len;
    bool not_null;
    bool is_primary_key;
    bool is_unique;
};

struct TableSchema {
    char name[64];
    ColumnDef columns[32];
    sigma_u32 column_count;
    sigma_u32 initial_pages;
};

// ─── Alter Table ──────────────────────────────────────────────────────────────
enum class AlterType { ADD_COLUMN, DROP_COLUMN, MODIFY_COLUMN, RENAME_TABLE };

struct AlterOp {
    AlterType type;
    ColumnDef column;
    char col_name[64];
    char new_name[64];
};

// ─── DML Structures ───────────────────────────────────────────────────────────
struct RowValue {
    ColType type;
    union {
        sigma_i64 int_val;
        double float_val;
        bool bool_val;
    } num;
    char str_val[256];
};

struct Row {
    RowValue values[32];
    sigma_u32 value_count;
};

enum class DMLOp { INSERT, UPDATE, DELETE };

enum class WhereOp { EQ, NEQ, GT, LT, GTE, LTE, LIKE };

struct WhereClause {
    char col_name[64];
    WhereOp op;
    RowValue val;
};

// ─── Joins & Select ───────────────────────────────────────────────────────────
enum class JoinType { INNER, LEFT, RIGHT, FULL, SELF };

struct JoinClause {
    JoinType type;
    char target_table[64];
    char left_col[64];
    char right_col[64];
};

struct SelectQuery {
    char table[64];
    char select_list[32][64];
    sigma_u32 select_count;
    bool distinct;
    bool has_where;
    WhereClause where;
    JoinClause joins[8];
    sigma_u32 join_count;
    bool has_group_by;
    char group_by_cols[4][64];
    sigma_u32 group_by_count;
    bool has_having;
    WhereClause having;
    bool has_order_by;
    char order_by[64];
    bool order_asc;
};

struct ResultSet {
    Row rows[1024];
    sigma_u32 row_count;
    ColumnDef columns[32];
    sigma_u32 column_count;
};

// ─── Cursor ───────────────────────────────────────────────────────────────────
struct Cursor {
    char table_name[64];
    sigma_u32 current_row;
    sigma_u32 max_rows;
    bool active;

    bool has_next() const { return active && current_row < max_rows; }
    void advance() { if (active) current_row++; }
    Row current() const;
    void update_current(const Row& r);
    void delete_current();
};

// ─── TCL & Savepoints ─────────────────────────────────────────────────────────
struct TransactionState {
    bool active;
    sigma_u32 txn_id;
    sigma_u32 rows_affected;
};

struct Savepoint {
    char name[64];
    sigma_u32 snapshot_id;
};

// ─── DCL Permissions ──────────────────────────────────────────────────────────
enum class Permission : sigma_u32 {
    SELECT = 0x01, INSERT = 0x02, UPDATE = 0x04, DELETE = 0x08, ALL = 0x0F
};

struct AccessControlList {
    void add(const char* user, const char* table, Permission p);
    void remove(const char* user, const char* table, Permission p);
    bool check(const char* user, const char* table, Permission p);
};

// ─── Triggers ─────────────────────────────────────────────────────────────────
enum class TriggerEvent { INSERT, UPDATE, DELETE };

struct Trigger {
    char name[64];
    char table_name[64];
    TriggerEvent event;
    void (*handler)(const Row* old_row, const Row* new_row);
};

// ─── Storage Backend Stub ─────────────────────────────────────────────────────
class StorageBackend {
public:
    void create_extent(const char* name, sigma_u32 pages) {}
    void drop_extent(const char* name) {}
    void truncate_extent(const char* name) {}
    void append_row(const char* table, const Row& r) {}
    ResultSet full_scan(const char* table) { ResultSet r{}; return r; }
    sigma_u32 create_snapshot(sigma_u32 txn_id) { return 1; }
    void commit_snapshot(sigma_u32 txn_id) {}
    void rollback_snapshot(sigma_u32 txn_id) {}
    void rollback_to_snapshot(sigma_u32 snap_id) {}
};

// ─── SigmaDB Engine ───────────────────────────────────────────────────────────
class SigmaDB {
public:
    // DDL
    int create_table(const TableSchema& schema);
    int alter_table(const char* table_name, const AlterOp& op);
    int drop_table(const char* table_name, bool if_exists = false);
    int truncate_table(const char* table_name);

    // DML
    int insert(const char* table_name, const Row& row);
    int update(const char* table_name, const Row& new_vals, const WhereClause& where);
    int delete_rows(const char* table_name, const WhereClause& where);

    // SELECT & Joins
    ResultSet select(const SelectQuery& q);

    // Set Operators
    ResultSet set_union(const ResultSet& a, const ResultSet& b, bool all = false);
    ResultSet set_intersect(const ResultSet& a, const ResultSet& b);
    ResultSet set_minus(const ResultSet& a, const ResultSet& b);

    // TCL
    int begin_transaction();
    int commit();
    int rollback(const char* savepoint_name = nullptr);
    int savepoint(const char* name);

    // DCL
    int grant(const char* user, const char* table_name, Permission perms);
    int revoke(const char* user, const char* table_name, Permission perms);

    // Triggers
    int create_trigger(const Trigger& trig);
    void fire_triggers(const char* table, TriggerEvent event, const Row* old_row, const Row* new_row);

    // Cursor management
    Cursor open_cursor(const char* table_name) {
        Cursor c{}; sigma_strncpy(c.table_name, table_name, sizeof(c.table_name));
        c.active = true; c.max_rows = 10; return c;
    }
    void close_cursor(Cursor& c) { c.active = false; }

private:
    bool validate_1nf(const TableSchema& schema) { return true; }
    bool catalog_has_table(const char* name) { return false; }
    void write_catalog_table(const TableSchema& schema) {}
    void drop_catalog_table(const char* name) {}
    bool has_dependent_fkeys(const char* name) { return false; }
    int validate_constraints(const char* table, const Row& r, DMLOp op) { return SIGMA_DB_OK; }
    void update_indexes(const char* table, const Row& r, DMLOp op) {}
    bool where_matches(const Row& r, const WhereClause& w) { return true; }
    Row merge_row(const Row& old_r, const Row& new_r) { return new_r; }

    ResultSet apply_join(const ResultSet& rs, const JoinClause& j) { return rs; }
    ResultSet filter_rows(const ResultSet& rs, const WhereClause& w) { return rs; }
    ResultSet group_rows(const ResultSet& rs, char cols[4][64], sigma_u32 cnt) { return rs; }
    ResultSet project_columns(const ResultSet& rs, char list[32][64], sigma_u32 cnt) { return rs; }
    ResultSet sort_rows(const ResultSet& rs, const char* col, bool asc) { return rs; }
    ResultSet remove_duplicates(const ResultSet& rs) { return rs; }

    bool rows_equal(const Row& a, const Row& b) { return false; }
    void append_row(ResultSet& rs, const Row& r) { if(rs.row_count < 1024) rs.rows[rs.row_count++] = r; }

    int add_column_impl(const char* t, const ColumnDef& c) { return SIGMA_DB_OK; }
    int drop_column_impl(const char* t, const char* c) { return SIGMA_DB_OK; }
    int modify_column_impl(const char* t, const ColumnDef& c) { return SIGMA_DB_OK; }
    int rename_table_impl(const char* t, const char* n) { return SIGMA_DB_OK; }

    Savepoint* find_savepoint(const char* name) {
        for(sigma_u32 i=0; i<savepoint_count_; i++)
            if(sigma_strcmp(savepoints_[i].name, name) == 0) return &savepoints_[i];
        return nullptr;
    }

    StorageBackend storage_;
    TransactionState current_txn_{};
    sigma_u32 txn_counter_{0};
    Savepoint savepoints_[32];
    sigma_u32 savepoint_count_{0};
    AccessControlList acl_;
    Trigger triggers_[64];
    sigma_u32 trigger_count_{0};
};

inline Row Cursor::current() const { Row r{}; return r; }
inline void Cursor::update_current(const Row& r) {}
inline void Cursor::delete_current() {}

inline void AccessControlList::add(const char* user, const char* table, Permission p) {}
inline void AccessControlList::remove(const char* user, const char* table, Permission p) {}
inline bool AccessControlList::check(const char* user, const char* table, Permission p) { return true; }

} // namespace Sigma::DB
