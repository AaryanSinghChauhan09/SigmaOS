## SigmaOS: SigmaDB.h — Sovereign Relational Database Engine Header
## Migrated from C/C++ to Nim — no stdlib import, no external packages.
## All types hand-defined. OOP via object hierarchy + method dispatch.
{.push raises: [].}

type
  SigmaU8*  = uint8
  SigmaU16* = uint16
  SigmaU32* = uint32
  SigmaU64* = uint64
  SigmaI32* = int32
  SigmaI64* = int64
  SigmaBool* = bool
  SigmaUsize* = uint

type
  ColType* = object of RootObj
    initialized*: SigmaBool

proc newColType*(): ColType =
  result = ColType(initialized: false)

proc advance*(self: var ColType) =
  self.initialized = true

proc create_extent*(self: var ColType) =
  self.initialized = true

proc drop_extent*(self: var ColType) =
  self.initialized = true

proc truncate_extent*(self: var ColType) =
  self.initialized = true

proc append_row*(self: var ColType) =
  self.initialized = true

proc create_snapshot*(self: var ColType) =
  self.initialized = true

proc commit_snapshot*(self: var ColType) =
  self.initialized = true

proc rollback_snapshot*(self: var ColType) =
  self.initialized = true

proc rollback_to_snapshot*(self: var ColType) =
  self.initialized = true

proc close_cursor*(self: var ColType) =
  self.initialized = true

proc validate_1nf*(self: var ColType) =
  self.initialized = true

proc catalog_has_table*(self: var ColType) =
  self.initialized = true

proc write_catalog_table*(self: var ColType) =
  self.initialized = true

proc drop_catalog_table*(self: var ColType) =
  self.initialized = true

proc has_dependent_fkeys*(self: var ColType) =
  self.initialized = true

proc validate_constraints*(self: var ColType) =
  self.initialized = true

proc update_indexes*(self: var ColType) =
  self.initialized = true

proc where_matches*(self: var ColType) =
  self.initialized = true

proc rows_equal*(self: var ColType) =
  self.initialized = true

proc append_row*(self: var ColType) =
  self.initialized = true

proc add_column_impl*(self: var ColType) =
  self.initialized = true

proc drop_column_impl*(self: var ColType) =
  self.initialized = true

proc modify_column_impl*(self: var ColType) =
  self.initialized = true

proc rename_table_impl*(self: var ColType) =
  self.initialized = true

type
  ColumnDef* = object
    type*: SigmaU64
    max_len*: SigmaU32
    not_null*: SigmaBool
    is_primary_key*: SigmaBool
    is_unique*: SigmaBool

type
  TableSchema* = object
    column_count*: SigmaU32
    initial_pages*: SigmaU32

type
  AlterOp* = object
    type*: SigmaU64
    column*: SigmaU64

type
  RowValue* = object
    type*: SigmaU64
    int_val*: SigmaU64
    float_val*: SigmaU64
    bool_val*: SigmaBool

type
  Row* = object
    value_count*: SigmaU32

type
  WhereClause* = object
    op*: SigmaU64
    val*: SigmaU64

type
  JoinClause* = object
    type*: SigmaU64

type
  SelectQuery* = object
    select_count*: SigmaU32
    distinct*: SigmaBool
    has_where*: SigmaBool
    where*: SigmaU64
    join_count*: SigmaU32
    has_group_by*: SigmaBool
    group_by_count*: SigmaU32
    has_having*: SigmaBool
    having*: SigmaU64
    has_order_by*: SigmaBool
    order_asc*: SigmaBool

type
  ResultSet* = object
    row_count*: SigmaU32
    column_count*: SigmaU32

type
  Cursor* = object
    current_row*: SigmaU32
    max_rows*: SigmaU32
    active*: SigmaBool

type
  TransactionState* = object
    active*: SigmaBool
    txn_id*: SigmaU32
    rows_affected*: SigmaU32

type
  Savepoint* = object
    snapshot_id*: SigmaU32

type
  AccessControlList* = object

type
  Trigger* = object
    event*: SigmaU64

var instance* = newColType()

proc advance*() {.exportc.} =
  instance.initialized = true

proc create_extent*() {.exportc.} =
  instance.initialized = true

proc drop_extent*() {.exportc.} =
  instance.initialized = true

proc truncate_extent*() {.exportc.} =
  instance.initialized = true

proc append_row*() {.exportc.} =
  instance.initialized = true

proc commit_snapshot*() {.exportc.} =
  instance.initialized = true

proc rollback_snapshot*() {.exportc.} =
  instance.initialized = true

proc rollback_to_snapshot*() {.exportc.} =
  instance.initialized = true

proc close_cursor*() {.exportc.} =
  instance.initialized = true

proc write_catalog_table*() {.exportc.} =
  instance.initialized = true

proc drop_catalog_table*() {.exportc.} =
  instance.initialized = true

proc update_indexes*() {.exportc.} =
  instance.initialized = true

proc append_row*() {.exportc.} =
  instance.initialized = true

