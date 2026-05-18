# Discrete Mathematics → SigmaOS Logic & Math Engine

> Maps the Discrete Mathematics syllabus to the `SovereignMath` kernel library — powering the rule engine, graph engine, and compliance dashboard.

---

## Unit I: Set Theory & Progression

### Set Theory

**Syllabus:**Basic concepts, union, intersection, complement, De Morgan's laws, Cartesian products, power sets.**SigmaOS Implementation:** `SovereignSetEngine` — a kernel-resident set operations library used by process scheduling, access control, and SigmaDB.

```cpp
// kernel/math/SovereignSetEngine.h
template<typename T>
class SovereignSet {
public:
    void insert(T element);
    void remove(T element);
    bool contains(T element) const;

    SovereignSet<T> union_with(const SovereignSet<T>& other) const;
    SovereignSet<T> intersect(const SovereignSet<T>& other) const;
    SovereignSet<T> complement(const SovereignSet<T>& universe) const;
    SovereignSet<T> difference(const SovereignSet<T>& other) const;

    // De Morgan's Laws: complement(A ∪ B) = complement(A) ∩ complement(B)
    static SovereignSet<T> de_morgan_union(const SovereignSet<T>& a, const SovereignSet<T>& b);
    static SovereignSet<T> de_morgan_intersect(const SovereignSet<T>& a, const SovereignSet<T>& b);

    // Cartesian product: A × B
    SovereignSet<std::pair<T,T>> cartesian_product(const SovereignSet<T>& other) const;

    // Power set: 2^A
    SovereignSet<SovereignSet<T>> power_set() const;
};

```text

### OS Applications
- Process capability sets (Linux-like capabilities as sets)
- Permission sets in MAC (Mandatory Access Control)
- SigmaDB JOIN operations modeled as set intersections

---

### Arithmetic & Geometric Progression

**Syllabus:**AP/GP sequences, series, AM-GM-HM relations, sum formulas.**SigmaOS Implementation:** `SovereignSequence` — used in memory allocation patterns, scheduler time-slice progression, and SigmaStats.

```cpp
// kernel/math/SovereignSequence.h
namespace Sigma::Math {
    // Arithmetic Progression: a, a+d, a+2d, ...
    double ap_nth_term(double a, double d, int n);
    double ap_sum(double a, double d, int n);

    // Geometric Progression: a, ar, ar², ...
    double gp_nth_term(double a, double r, int n);
    double gp_sum(double a, double r, int n);
    double gp_infinite_sum(double a, double r);  // | r | < 1

    // Mean relations
    double arithmetic_mean(double a, double b);
    double geometric_mean(double a, double b);
    double harmonic_mean(double a, double b);
    // AM ≥ GM ≥ HM (always holds for positive numbers)
}

```text

### OS Applications
- Memory allocator uses GP doubling (1 → 2 → 4 → 8 pages)
- Exponential backoff in network retries
- SigmaStats central tendency calculations

---

## Unit II: Matrices & Determinants

### Matrix Operations

**Syllabus:**Types (square, identity, diagonal, sparse, triangular), transpose, addition, multiplication, inverse.**SigmaOS Implementation:** `SovereignMatrix` — linear algebra engine for SigmaAI and SigmaStats.

```cpp
// kernel/math/SovereignMatrix.h
template<typename T, size_t Rows, size_t Cols>
class SovereignMatrix {
public:
    T data[Rows][Cols];

    // Types
    bool is_square() const;
    bool is_identity() const;
    bool is_diagonal() const;
    bool is_symmetric() const;
    bool is_sparse() const;

    // Operations
    SovereignMatrix<T, Cols, Rows> transpose() const;
    SovereignMatrix<T, Rows, Cols> operator+(const SovereignMatrix& other) const;
    SovereignMatrix<T, Rows, Cols> scalar_multiply(T scalar) const;

    template<size_t OtherCols>
    SovereignMatrix<T, Rows, OtherCols> multiply(
        const SovereignMatrix<T, Cols, OtherCols>& other) const;

    // Inverse (for square matrices)
    SovereignMatrix<T, Rows, Cols> inverse() const;  // Gauss-Jordan elimination
};

```text

---

### Determinants & Cramer's Method

**Syllabus:** Properties of determinants, cofactor expansion, Cramer's rule for linear systems.

```cpp
// kernel/math/SovereignDeterminant.h
namespace Sigma::Math {
    double determinant_2x2(double a[2][2]);
    double determinant_3x3(double a[3][3]);
    double determinant_nxn(double** a, int n);  // LU decomposition

    // Cramer's Rule: Ax = b → x_i = det(A_i) / det(A)
    std::vector<double> cramers_rule(double** A, double* b, int n);
}

```text

### OS Applications
- `SigmaAI` uses matrix multiplication for neural network forward passes
- `SigmaStats` regression uses matrix inverse for least-squares solution
- `SigmaModeler` ER diagram layout uses graph Laplacian matrices

---

## Unit III: Propositional & Predicate Logic

### Propositional Logic

**Syllabus:**Truth values, logical connectives (AND, OR, NOT, XOR, IMPLICATION, BICONDITIONAL), truth tables, tautologies, contradictions, logical equivalences.**SigmaOS Implementation:** `SovereignRuleEngine` — powers the compliance dashboard and access policy evaluator.

```cpp
// kernel/logic/SovereignRuleEngine.h
enum LogicValue { FALSE = 0, TRUE = 1, UNKNOWN = -1 };

class Proposition {
public:
    virtual LogicValue evaluate() const = 0;
};

class AndProp : public Proposition {
    Proposition* left; Proposition* right;
    LogicValue evaluate() const override {
        return (left->evaluate() == TRUE && right->evaluate() == TRUE) ? TRUE : FALSE;
    }
};

class ImpliesProp : public Proposition {
    Proposition* antecedent; Proposition* consequent;
    // P → Q ≡ ¬P ∨ Q
    LogicValue evaluate() const override;
};

class SovereignRuleEngine {
public:
    bool is_tautology(Proposition* p);      // True for ALL truth assignments
    bool is_contradiction(Proposition* p);  // False for ALL truth assignments
    bool are_equivalent(Proposition* p, Proposition* q);
    std::vector<std::map<std::string, bool>> truth_table(Proposition* p);
};

```text

### OS Applications
- Security policy: `IF user.role == "admin" AND file.sensitive == true THEN require_mfa`
- SigmaDB WHERE clause parsing using propositional logic AST
- Kernel capability checks: conjunction of permissions

---

### Predicate Logic

**Syllabus:** Predicates, quantifiers (∀, ∃), free and bound variables, logical inference.

```cpp
// kernel/logic/SovereignPredicateEngine.h
template<typename Domain>
class Predicate {
    std::function<bool(Domain)> predicate_fn;
public:
    // ∀x P(x) — For all x in domain, P(x) holds
    bool for_all(const std::vector<Domain>& domain) const;

    // ∃x P(x) — There exists x in domain such that P(x) holds
    bool exists(const std::vector<Domain>& domain) const;

    // Free variables: appear unbound in formula
    // Bound variables: quantified by ∀ or ∃
};

```text

### OS Applications
- SigmaDB: `SELECT * FROM files WHERE ∀ permission IN required: user HAS permission`
- Kernel verification: ∀ syscall: privilege_level ≥ required_ring
- SigmaLegalAI: predicate-based case law matching

---

## Unit IV: Relations & Functions

### Functions

**Syllabus:** Definition, domain/range/codomain, injective, surjective, bijective, composition, inverse, pigeonhole principle.

```cpp
// kernel/math/SovereignFunction.h
template<typename A, typename B>
class SovereignFunction {
public:
    std::function<B(A)> fn;

    bool is_injective(const std::vector<A>& domain) const;  // One-to-one
    bool is_surjective(const std::vector<A>& domain, const std::vector<B>& codomain) const;
    bool is_bijective(const std::vector<A>& domain, const std::vector<B>& codomain) const;

    // Composition: (g ∘ f)(x) = g(f(x))
    template<typename C>
    SovereignFunction<A, C> compose(const SovereignFunction<B, C>& g) const;

    // Inverse (requires bijection)
    SovereignFunction<B, A> inverse() const;
};

// Pigeonhole Principle: n+1 items into n holes → at least 1 hole has ≥ 2 items
// Used in: hash collision analysis, scheduling conflict detection
int pigeonhole_min_collisions(int items, int buckets);

```text

---

### Relations

**Syllabus:** Properties (reflexive, symmetric, antisymmetric, transitive), equivalence relations, partial orders, composite relations.

```cpp
// kernel/math/SovereignRelation.h
template<typename T>
class SovereignRelation {
    std::set<std::pair<T, T>> pairs;
public:
    bool is_reflexive(const std::set<T>& domain) const;
    bool is_symmetric() const;
    bool is_antisymmetric() const;
    bool is_transitive() const;

    bool is_equivalence() const;  // reflexive + symmetric + transitive
    bool is_partial_order() const;  // reflexive + antisymmetric + transitive

    // Equivalence classes
    std::vector<std::set<T>> equivalence_classes(const std::set<T>& domain) const;

    // Composite relation: R ∘ S = {(a,c) | ∃b: (a,b)∈R ∧ (b,c)∈S}
    SovereignRelation<T> compose(const SovereignRelation<T>& other) const;

    // Transitive closure (Warshall's algorithm)
    SovereignRelation<T> transitive_closure(const std::set<T>& domain) const;
};

```text

### OS Applications
- Process dependency graph: transitive closure determines deadlock detection
- File system permissions: partial order on privilege levels
- SigmaDB: equivalence relations for duplicate detection and normalization

---

## 🔗 Related Wiki Pages

- [Syllabus Implementation Map](Syllabus-Implementation-Map)
- [SigmaDB SQL Engine](Syllabus-RDBMS)
- [SigmaAI Intelligence Layer](Syllabus-AIML)
- [SigmaStats Toolkit](Syllabus-Statistics)

---

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*
