# RDBMS & Oracle SQL → SigmaDB Engine

> Maps the RDBMS/Oracle SQL syllabus to `SigmaDB` — SigmaOS's native relational database engine, emphasizing ACID guarantees, MVCC concurrency, and B+ Tree indexing.

---

## Unit I: DBMS Overview & Codd's Rules

### What is an RDBMS?

A Relational Database Management System is foundational software that stores, manages, and queries structured relational data using mathematical tables, rows, and columns.

**Unique Selling Point (USP):** Absolute ACID transaction compliance, Multi-Version Concurrency Control (MVCC), and zero-loss durability backed by silicon-direct storage sharding.

### Codd's 12 Rules — SigmaDB Compliance

| Rule | SigmaDB Status | 
| :--- | :--- | 
| **Information Rule** — all data represented in tables | ✅ Fully compliant | 
| **Guaranteed Access** — Table Name + Primary Key + Column | ✅ Fully compliant | 
| **NULL Support** — Systematic treatment of missing data | ✅ Three-valued logic | 
| **Active Online Catalog** (`sigma_catalog`) | ✅ Accessible via standard SQL | 
| **Comprehensive Data Sublanguage** (SQL + PL/SQL) | ✅ Freestanding C++ parser | 
| **View Updatability** | ✅ Automatic view translation | 
| **Physical Data Independence** | ✅ S-ZFS storage abstraction | 
| **Distribution Independence** | ✅ SovereignCloudFS clustering | 

### Normalization & Schema Design

Normalization eliminates data redundancy and update anomalies by decomposing tables into strict normal forms:

* **1NF:** Eliminates repeating groups; enforces atomic scalar column values.
* **2NF:** Requires 1NF and full functional dependency of non-key attributes on the primary composite key.
* **3NF:** Requires 2NF and elimination of transitive functional dependencies.
* **BCNF:** Boyce-Codd Normal Form; requires every non-trivial determinant to be a candidate key.

Enforced automatically by the SigmaDB schema validator (`normalizer.cpp`).

---

## Unit II: DDL, DML, Joins, Transactions

```sql
-- DDL: Create table with domain constraints
CREATE TABLE employees (
    emp_id   INT PRIMARY KEY,
    emp_name VARCHAR(100) NOT NULL,
    dept_id  INT REFERENCES departments(dept_id),
    salary   DECIMAL(10,2) CHECK (salary > 0),
    email    VARCHAR(200) UNIQUE
);

-- DML & Joins: Inner join across foreign keys
SELECT e.emp_name, d.dept_name
FROM employees e INNER JOIN departments d ON e.dept_id = d.dept_id;

-- Set Operators
SELECT emp_id FROM employees UNION SELECT emp_id FROM contractors;
SELECT emp_id FROM employees INTERSECT SELECT emp_id FROM managers;
SELECT emp_id FROM employees MINUS SELECT emp_id FROM terminated;

-- TCL: ACID Transaction Commit / Rollback
BEGIN;
UPDATE accounts SET bal = bal - 1000 WHERE id = 1;
UPDATE accounts SET bal = bal + 1000 WHERE id = 2;
COMMIT;

-- DCL: Role-based Access Control
GRANT SELECT, INSERT ON employees TO sigma_admin;
REVOKE INSERT ON employees FROM sigma_admin;
```

---

## Unit III: Functions & Oracle Objects

```sql
-- Numeric: ABS, CEIL, FLOOR, ROUND, TRUNC, SQRT, MOD, POWER, GREATEST, LEAST
-- Character: UPPER, LOWER, INITCAP, SUBSTR, CONCAT, REPLACE, LPAD, RPAD, TRIM
-- Date: SYSDATE, ADD_MONTHS, LAST_DAY, MONTHS_BETWEEN, TO_DATE, TO_CHAR
-- Aggregate: COUNT, SUM, AVG, MAX, MIN

-- View Creation
CREATE VIEW high_earners AS
SELECT emp_name, salary FROM employees WHERE salary > 100000;

-- Sequence Generation
CREATE SEQUENCE emp_seq START WITH 1000 INCREMENT BY 1;
INSERT INTO employees (emp_id, emp_name) VALUES (emp_seq.NEXTVAL, 'Alice');

-- B+ Tree Index Creation
CREATE INDEX idx_salary ON employees(salary);
```

---

## Unit IV: PL/SQL Programming

```plsql
-- PL/SQL Anonymous Block Structure
DECLARE
    v_salary  employees.salary%TYPE;
    v_emp_row employees%ROWTYPE;
BEGIN
    SELECT salary INTO v_salary FROM employees WHERE emp_id = 1;
    IF v_salary > 100000 THEN
        DBMS_OUTPUT.PUT_LINE('High earner');
    END IF;
    FOR i IN 1..10 LOOP
        DBMS_OUTPUT.PUT_LINE(i);
    END LOOP;
EXCEPTION
    WHEN NO_DATA_FOUND THEN
        DBMS_OUTPUT.PUT_LINE('Not found');
END;

-- Explicit Cursor Traversal
DECLARE
    CURSOR c IS SELECT * FROM employees WHERE dept_id = 10;
BEGIN
    FOR r IN c LOOP
        DBMS_OUTPUT.PUT_LINE(r.emp_name);
    END LOOP;
END;

-- Stored Procedure Creation
CREATE OR REPLACE PROCEDURE give_raise(p_dept INT, p_pct NUMBER) IS
BEGIN
    UPDATE employees SET salary = salary * (1 + p_pct / 100) WHERE dept_id = p_dept;
    COMMIT;
END;

-- Row-level Trigger Execution
CREATE OR REPLACE TRIGGER emp_audit AFTER INSERT OR UPDATE ON employees
FOR EACH ROW
BEGIN
    INSERT INTO audit_log VALUES(SYSDATE, :NEW.emp_id);
END;
```

---

## Debugging & Problem-Solving in RDBMS

### Common Issues & Fix Strategies

* **Issue - Incorrect Indexing & Table Scans:** Missing B+ Tree indices cause full table scans ($O(N)$), stalling heavy analytical `JOIN` queries.
  * *Fix Strategy:* Run `EXPLAIN PLAN FOR` to inspect the query execution tree, identify unindexed nested loops, and create composite covering B+ Tree indices (`CREATE INDEX idx_emp_dept ON employees(dept_id, salary)`).
* **Issue - Database Deadlocks:** Concurrent transactions update identical table rows in reverse order, triggering circular lock wait states.
  * *Fix Strategy:* Enforce strict Two-Phase Locking (2PL), acquire row locks in a globally deterministic order, and utilize `SELECT ... FOR UPDATE NOWAIT` to prevent indefinite blocking.
* **Issue - Dirty Reads & Phantom Records:** Unsynchronized transaction isolation levels permit reading uncommitted data or phantom insertions.
  * *Fix Strategy:* Elevate transaction isolation levels from `READ COMMITTED` to `REPEATABLE READ` or `SERIALIZABLE`, leveraging SigmaDB's MVCC snapshot isolation engine.

---

## SigmaDB Architecture

```
SigmaDB Engine
├── SQL Parser + Query Optimizer
├── PL/SQL Runtime + Cursor Engine
├── Transaction Manager (MVCC, ACID)
├── Trigger Dispatcher + Package Registry
└── Storage: SovereignZFSPool (CoW + Snapshots)
```

**Files:** `userland/apps/SigmaDB/sql_engine.cpp`, `plsql_runtime.cpp`, `trigger_dispatcher.cpp`
*Last updated: 2026-05-19 | SigmaOS Zenith v15.2*
