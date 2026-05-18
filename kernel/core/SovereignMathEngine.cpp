/**
 * SovereignMathEngine.cpp — Discrete Mathematics Implementation
 * SigmaOS Zenith v15.1
 * Maps to: Syllabus-DiscreteMath (Progressions, Matrices, Logic, Relations)
 */
#include "SovereignMathEngine.h"
#include "sigma_kernel_types.h"

namespace Sigma::Math {

// ─── Unit I: Progressions ─────────────────────────────────────────────────────
double ap_nth_term(double a, double d, int n) { return a + (n - 1) * d; }
double ap_sum(double a, double d, int n)      { return (n / 2.0) * (2 * a + (n - 1) * d); }
double gp_nth_term(double a, double r, int n) {
    double result = a;
    for (int i = 1; i < n; i++) result *= r;
    return result;
}
double gp_sum(double a, double r, int n) {
    if (r == 1.0) return a * n;
    double rn = 1.0;
    for (int i = 0; i < n; i++) rn *= r;
    return a * (rn - 1.0) / (r - 1.0);
}
double gp_infinite_sum(double a, double r) {
    // Valid only for |r| < 1
    if (r >= 1.0 || r <= -1.0) return 0.0; // undefined/diverges
    return a / (1.0 - r);
}
double arithmetic_mean(double a, double b) { return (a + b) / 2.0; }
double geometric_mean(double a, double b)  {
    // Kernel sqrt — use Newton's method
    double x = (a + b) / 2.0;
    for (int i = 0; i < 50; i++) x = (x + (a * b) / x) / 2.0;
    return x;
}
double harmonic_mean(double a, double b) { return 2.0 * a * b / (a + b); }
bool verify_am_gm_hm(double a, double b) {
    double am = arithmetic_mean(a, b);
    double gm = geometric_mean(a, b);
    double hm = harmonic_mean(a, b);
    return (am >= gm - 1e-9) && (gm >= hm - 1e-9);
}

// ─── Unit II: Matrices & Determinants ────────────────────────────────────────
double det2x2(const double m[2][2]) { return m[0][0]*m[1][1] - m[0][1]*m[1][0]; }

double det3x3(const double m[3][3]) {
    return m[0][0]*(m[1][1]*m[2][2] - m[1][2]*m[2][1])
         - m[0][1]*(m[1][0]*m[2][2] - m[1][2]*m[2][0])
         + m[0][2]*(m[1][0]*m[2][1] - m[1][1]*m[2][0]);
}

void mat_multiply(const double* A, int Ar, int Ac,
                  const double* B, int Br, int Bc, double* C) {
    for (int i = 0; i < Ar; i++)
        for (int j = 0; j < Bc; j++) {
            double sum = 0;
            for (int k = 0; k < Ac; k++) sum += A[i*Ac+k] * B[k*Bc+j];
            C[i*Bc+j] = sum;
        }
}

void mat_transpose(const double* A, int r, int c, double* B) {
    for (int i = 0; i < r; i++)
        for (int j = 0; j < c; j++)
            B[j*r+i] = A[i*c+j];
}

void mat_add(const double* A, const double* B, double* C, int r, int c) {
    for (int i = 0; i < r*c; i++) C[i] = A[i] + B[i];
}

void mat_scalar_mul(const double* A, double k, double* B, int r, int c) {
    for (int i = 0; i < r*c; i++) B[i] = A[i] * k;
}

bool is_identity(const double* A, int n) {
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++) {
            double expected = (i == j) ? 1.0 : 0.0;
            if (A[i*n+j] < expected - 1e-9 || A[i*n+j] > expected + 1e-9) return false;
        }
    return true;
}
bool is_diagonal(const double* A, int n) {
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (i != j && (A[i*n+j] > 1e-9 || A[i*n+j] < -1e-9)) return false;
    return true;
}
bool is_symmetric(const double* A, int n) {
    for (int i = 0; i < n; i++)
        for (int j = i+1; j < n; j++) {
            double diff = A[i*n+j] - A[j*n+i];
            if (diff > 1e-9 || diff < -1e-9) return false;
        }
    return true;
}

// ─── Unit III: Propositional Logic ───────────────────────────────────────────
LogicVal logic_and(LogicVal a, LogicVal b) {
    if (a == LogicVal::FALSE || b == LogicVal::FALSE) return LogicVal::FALSE;
    if (a == LogicVal::TRUE  && b == LogicVal::TRUE)  return LogicVal::TRUE;
    return LogicVal::UNKNOWN;
}
LogicVal logic_or(LogicVal a, LogicVal b) {
    if (a == LogicVal::TRUE  || b == LogicVal::TRUE)  return LogicVal::TRUE;
    if (a == LogicVal::FALSE && b == LogicVal::FALSE) return LogicVal::FALSE;
    return LogicVal::UNKNOWN;
}
LogicVal logic_not(LogicVal a) {
    if (a == LogicVal::TRUE)  return LogicVal::FALSE;
    if (a == LogicVal::FALSE) return LogicVal::TRUE;
    return LogicVal::UNKNOWN;
}
LogicVal logic_xor(LogicVal a, LogicVal b) {
    if (a == LogicVal::UNKNOWN || b == LogicVal::UNKNOWN) return LogicVal::UNKNOWN;
    return (a != b) ? LogicVal::TRUE : LogicVal::FALSE;
}
LogicVal logic_implies(LogicVal p, LogicVal q) {
    // P → Q ≡ ¬P ∨ Q
    return logic_or(logic_not(p), q);
}
LogicVal logic_biconditional(LogicVal p, LogicVal q) {
    // P ↔ Q ≡ (P→Q) ∧ (Q→P)
    return logic_and(logic_implies(p, q), logic_implies(q, p));
}

bool is_tautology_2var(LogicVal (*fn)(LogicVal, LogicVal)) {
    LogicVal vals[2] = { LogicVal::FALSE, LogicVal::TRUE };
    for (auto a : vals)
        for (auto b : vals)
            if (fn(a, b) != LogicVal::TRUE) return false;
    return true;
}
bool is_contradiction_2var(LogicVal (*fn)(LogicVal, LogicVal)) {
    LogicVal vals[2] = { LogicVal::FALSE, LogicVal::TRUE };
    for (auto a : vals)
        for (auto b : vals)
            if (fn(a, b) != LogicVal::FALSE) return false;
    return true;
}

// De Morgan's Laws verification
bool verify_de_morgan_and(LogicVal a, LogicVal b) {
    // ¬(A ∧ B) == ¬A ∨ ¬B
    return logic_not(logic_and(a, b)) == logic_or(logic_not(a), logic_not(b));
}
bool verify_de_morgan_or(LogicVal a, LogicVal b) {
    // ¬(A ∨ B) == ¬A ∧ ¬B
    return logic_not(logic_or(a, b)) == logic_and(logic_not(a), logic_not(b));
}

// ─── Unit IV: Relations ───────────────────────────────────────────────────────
bool Relation::has(int i, int j) const { return matrix[i*n+j]; }
void Relation::add(int i, int j)       { matrix[i*n+j] = true; }
void Relation::remove(int i, int j)    { matrix[i*n+j] = false; }

bool Relation::is_reflexive() const {
    for (int i = 0; i < n; i++) if (!matrix[i*n+i]) return false;
    return true;
}
bool Relation::is_symmetric() const {
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (matrix[i*n+j] && !matrix[j*n+i]) return false;
    return true;
}
bool Relation::is_antisymmetric() const {
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (i != j && matrix[i*n+j] && matrix[j*n+i]) return false;
    return true;
}
bool Relation::is_transitive() const {
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            if (matrix[i*n+j])
                for (int k = 0; k < n; k++)
                    if (matrix[j*n+k] && !matrix[i*n+k]) return false;
    return true;
}
bool Relation::is_equivalence()   const { return is_reflexive() && is_symmetric() && is_transitive(); }
bool Relation::is_partial_order() const { return is_reflexive() && is_antisymmetric() && is_transitive(); }

void Relation::transitive_closure() {
    // Warshall's Algorithm: O(n³)
    for (int k = 0; k < n; k++)
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (matrix[i*n+k] && matrix[k*n+j]) matrix[i*n+j] = true;
}

// ─── Functions ────────────────────────────────────────────────────────────────
bool is_injective(int* f, int domain_size, int codomain_size) {
    // f: A → B is injective if no two domain elements map to the same codomain element
    bool* seen = new bool[codomain_size]();
    for (int i = 0; i < domain_size; i++) {
        if (f[i] < 0 || f[i] >= codomain_size || seen[f[i]]) {
            delete[] seen; return false;
        }
        seen[f[i]] = true;
    }
    delete[] seen; return true;
}
bool is_surjective(int* f, int domain_size, int codomain_size) {
    bool* covered = new bool[codomain_size]();
    for (int i = 0; i < domain_size; i++)
        if (f[i] >= 0 && f[i] < codomain_size) covered[f[i]] = true;
    for (int j = 0; j < codomain_size; j++)
        if (!covered[j]) { delete[] covered; return false; }
    delete[] covered; return true;
}
int pigeonhole_min_collisions(int items, int buckets) {
    // At least ceil(items/buckets) - 1 extra items must share a bucket
    if (buckets <= 0) return items;
    return (items - 1) / buckets; // minimum guaranteed pigeons in one hole - 1
}

} // namespace Sigma::Math
