
# RDBMS & Oracle SQL → SigmaDB Engine


> Maps the RDBMS/Oracle SQL syllabus to `SigmaDB` — SigmaOS's native relational database engine.

---


## Unit I: DBMS Overview



### Codd's 12 Rules — SigmaDB Compliance


| Rule | SigmaDB Status |
| --- | --- |
| Information Rule — all data in tables | ✅ |
| Guaranteed Access — PK + table name | ✅ |
| NULL Support | ✅ Three-valued logic |
| Active Online Catalog (`sigma_catalog`) | ✅ |
| SQL + PL/SQL support | ✅ |
| View Updatability | ✅ |
| Physical Data Independence | ✅ S-ZFS backend |
| Distribution Independence | ✅ SovereignCloudFS |


### Normalization

1NF → 2NF → 3NF → BCNF — enforced by SigmaDB schema validator (`normalizer.cpp`).

---


## Unit II: DDL, DML, Joins, Transactions


```sql
-- Create with constraints
CREATE TABLE employees (
    emp_id   INT PRIMARY KEY,
    emp_name VARCHAR(100) NOT NULL,
    dept_id  INT REFERENCES departments(dept_id),
    salary   DECIMAL(10,2) CHECK (salary > 0),
    email    VARCHAR(200) UNIQUE
);

-- Joins
SELECT e.emp_name, d.dept_name
FROM employees e INNER JOIN departments d ON e.dept_id = d.dept_id;

-- Set Operators
SELECT emp_id FROM employees UNION SELECT emp_id FROM contractors;
SELECT emp_id FROM employees INTERSECT SELECT emp_id FROM managers;
SELECT emp_id FROM employees MINUS SELECT emp_id FROM terminated;

-- Transaction
BEGIN; UPDATE accounts SET bal=bal-1000 WHERE id=1;
      UPDATE accounts SET bal=bal+1000 WHERE id=2; COMMIT;

-- DCL
GRANT SELECT, INSERT ON employees TO sigma_admin;
REVOKE INSERT ON employees FROM sigma_admin;

```text

---


## Unit III: Functions & Oracle Objects


```sql
-- Numeric: ABS, CEIL, FLOOR, ROUND, TRUNC, SQRT, MOD, POWER, GREATEST, LEAST
-- Character: UPPER, LOWER, INITCAP, SUBSTR, CONCAT, REPLACE, LPAD, RPAD, TRIM
-- Date: SYSDATE, ADD_MONTHS, LAST_DAY, MONTHS_BETWEEN, TO_DATE, TO_CHAR
-- Aggregate: COUNT, SUM, AVG, MAX, MIN

-- View
CREATE VIEW high_earners AS SELECT emp_name, salary FROM employees WHERE salary > 100000;

-- Sequence
CREATE SEQUENCE emp_seq START WITH 1000 INCREMENT BY 1;
INSERT INTO employees (emp_id, emp_name) VALUES (emp_seq.NEXTVAL, 'Alice');

-- Index
CREATE INDEX idx_salary ON employees(salary);

```text

---


## Unit IV: PL/SQL


```plsql
-- Block structure
DECLARE
    v_salary  employees.salary%TYPE;
    v_emp_row employees%ROWTYPE;
BEGIN
    SELECT salary INTO v_salary FROM employees WHERE emp_id = 1;
    IF v_salary > 100000 THEN DBMS_OUTPUT.PUT_LINE('High earner'); END IF;
    FOR i IN 1..10 LOOP DBMS_OUTPUT.PUT_LINE(i); END LOOP;
EXCEPTION
    WHEN NO_DATA_FOUND THEN DBMS_OUTPUT.PUT_LINE('Not found');
END;

-- Explicit Cursor
DECLARE CURSOR c IS SELECT * FROM employees WHERE dept_id=10;
BEGIN FOR r IN c LOOP DBMS_OUTPUT.PUT_LINE(r.emp_name); END LOOP; END;

-- Procedure
CREATE OR REPLACE PROCEDURE give_raise(p_dept INT, p_pct NUMBER) IS
BEGIN UPDATE employees SET salary=salary*(1+p_pct/100) WHERE dept_id=p_dept; COMMIT; END;

-- Trigger
CREATE OR REPLACE TRIGGER emp_audit AFTER INSERT OR UPDATE ON employees
FOR EACH ROW BEGIN INSERT INTO audit_log VALUES(SYSDATE, :NEW.emp_id); END;

```text

---


## SigmaDB Architecture


```text
SigmaDB Engine
├── SQL Parser + Query Optimizer
├── PL/SQL Runtime + Cursor Engine
├── Transaction Manager (MVCC, ACID)
├── Trigger Dispatcher + Package Registry
└── Storage: SovereignZFSPool (CoW + Snapshots)

```text

**Files:**`userland/apps/SigmaDB/sql_engine.cpp`, `plsql_runtime.cpp`, `trigger_dispatcher.cpp`*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
