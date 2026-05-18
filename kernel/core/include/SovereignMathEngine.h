/**
 * SovereignMathEngine.h — Discrete Mathematics Kernel Library
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-DiscreteMath — Set Theory, Matrices, Logic, Relations/Functions
 * Implements: SovereignSet, SovereignMatrix, SovereignRuleEngine, SovereignRelation
 *
 * Zero-dependency kernel math primitives. No STL.
 */
#pragma once
#include "../../../include/core/sigma_kernel_types.h"

namespace Sigma::Math {

// ════════════════════════════════════════════════════════════════════════════
// UNIT I: PROGRESSIONS
// ════════════════════════════════════════════════════════════════════════════

    /** Arithmetic Progression: nth term = a + (n-1)*d */
    double ap_nth_term(double a, double d, int n);

    /** AP sum of n terms: n/2 * (2a + (n-1)d) */
    double ap_sum(double a, double d, int n);

    /** Geometric Progression: nth term = a * r^(n-1) */
    double gp_nth_term(double a, double r, int n);

    /** GP sum of n terms: a*(r^n - 1)/(r - 1) for r != 1 */
    double gp_sum(double a, double r, int n);

    /** GP infinite sum: a/(1-r) for |r| < 1 */
    double gp_infinite_sum(double a, double r);

    /** Arithmetic Mean: (a + b) / 2 */
    double arithmetic_mean(double a, double b);

    /** Geometric Mean: sqrt(a * b) */
    double geometric_mean(double a, double b);

    /** Harmonic Mean: 2ab / (a + b) */
    double harmonic_mean(double a, double b);

    /** Verify AM >= GM >= HM (for positive a, b) */
    bool verify_am_gm_hm(double a, double b);

// ════════════════════════════════════════════════════════════════════════════
// UNIT II: MATRICES & DETERMINANTS (up to 4×4 compile-time)
// ════════════════════════════════════════════════════════════════════════════

    /** 2×2 determinant: ad - bc */
    double det2x2(const double m[2][2]);

    /** 3×3 determinant: cofactor expansion */
    double det3x3(const double m[3][3]);

    /** NxN determinant: LU decomposition (heap-allocated) */
    double det_nxn(const double* m, int n);

    /** Matrix multiply: C[m×p] = A[m×n] * B[n×p] */
    void mat_multiply(const double* A, int Ar, int Ac,
                      const double* B, int Br, int Bc,
                      double* C);

    /** Matrix transpose: B[c×r] = A[r×c]^T */
    void mat_transpose(const double* A, int r, int c, double* B);

    /** Matrix addition: C = A + B (same dimensions) */
    void mat_add(const double* A, const double* B, double* C, int r, int c);

    /** Scalar multiply: B = k * A */
    void mat_scalar_mul(const double* A, double k, double* B, int r, int c);

    /** Matrix inverse using Gauss-Jordan elimination (n×n)
     *  Returns false if matrix is singular */
    bool mat_inverse(const double* A, int n, double* Ainv);

    /** Cramer's Rule: solve Ax = b, n equations
     *  Returns false if det(A) == 0 */
    bool cramers_rule(const double* A, const double* b, int n, double* x);

    /** Check matrix type */
    bool is_identity(const double* A, int n);
    bool is_diagonal(const double* A, int n);
    bool is_symmetric(const double* A, int n);
    bool is_upper_triangular(const double* A, int n);
    bool is_lower_triangular(const double* A, int n);

// ════════════════════════════════════════════════════════════════════════════
// UNIT III: PROPOSITIONAL LOGIC
// ════════════════════════════════════════════════════════════════════════════

    enum class LogicVal : sigma_i8 { FALSE = 0, TRUE = 1, UNKNOWN = -1 };

    /** Logical AND, OR, NOT, XOR, NAND, NOR */
    LogicVal logic_and(LogicVal a, LogicVal b);
    LogicVal logic_or(LogicVal a, LogicVal b);
    LogicVal logic_not(LogicVal a);
    LogicVal logic_xor(LogicVal a, LogicVal b);
    LogicVal logic_implies(LogicVal p, LogicVal q);   // P → Q ≡ ¬P ∨ Q
    LogicVal logic_biconditional(LogicVal p, LogicVal q); // P ↔ Q ≡ (P→Q)∧(Q→P)

    /** Check if 2-variable proposition is tautology (true for all inputs) */
    bool is_tautology_2var(LogicVal (*fn)(LogicVal, LogicVal));

    /** Check if 2-variable proposition is contradiction */
    bool is_contradiction_2var(LogicVal (*fn)(LogicVal, LogicVal));

    /** De Morgan's Laws:
     *  ¬(A ∧ B) ≡ ¬A ∨ ¬B
     *  ¬(A ∨ B) ≡ ¬A ∧ ¬B
     */
    bool verify_de_morgan_and(LogicVal a, LogicVal b);
    bool verify_de_morgan_or(LogicVal a, LogicVal b);

// ════════════════════════════════════════════════════════════════════════════
// UNIT IV: RELATIONS & FUNCTIONS
// ════════════════════════════════════════════════════════════════════════════

    /**
     * Relation over integer domain [0, n)
     * Stored as adjacency matrix: rel[i*n + j] = true if (i,j) ∈ R
     */
    struct Relation {
        bool*      matrix;    // n×n adjacency matrix
        int        n;         // domain size

        bool has(int i, int j) const;
        void add(int i, int j);
        void remove(int i, int j);

        bool is_reflexive()    const;  // ∀x: (x,x) ∈ R
        bool is_symmetric()    const;  // (x,y) ∈ R → (y,x) ∈ R
        bool is_antisymmetric()const;  // (x,y)∈R ∧ (y,x)∈R → x=y
        bool is_transitive()   const;  // (x,y)∈R ∧ (y,z)∈R → (x,z)∈R
        bool is_equivalence()  const;  // reflexive + symmetric + transitive
        bool is_partial_order()const;  // reflexive + antisymmetric + transitive

        /** Warshall's algorithm: transitive closure */
        void transitive_closure();

        /** Equivalence classes (requires is_equivalence() == true)
         *  Writes class IDs into class_id[n] */
        int equivalence_classes(int* class_id) const;
    };

    /** Function injectivity check over finite domain */
    bool is_injective(int* f, int domain_size, int codomain_size);

    /** Function surjectivity check */
    bool is_surjective(int* f, int domain_size, int codomain_size);

    /** Pigeonhole principle: minimum guaranteed collisions */
    int pigeonhole_min_collisions(int items, int buckets);
    // = ceil(items / buckets) - 1 minimum guaranteed

} // namespace Sigma::Math
