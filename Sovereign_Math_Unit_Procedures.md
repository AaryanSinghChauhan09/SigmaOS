# 🧮 Sovereign Math Unit Procedures

SigmaOS implements a **zero-dependency math engine** covering DSA, ML mathematics, calculator operations, and data science kernels — all in pure C11 without importing `<math.h>`.

---

## DSA (Data Structures & Algorithms)

### Sorting Algorithms

```c
// Quicksort (user-defined, in-place)
void sigma_quicksort(sigma_i64* arr, int low, int high);

// Merge Sort
void sigma_mergesort(sigma_i64* arr, int left, int right);

// Heapsort
void sigma_heapsort(sigma_i64* arr, sigma_u32 n);
```

### Search Algorithms

```c
// Binary Search — O(log N)
int sigma_binary_search(sigma_i64* arr, sigma_u32 n, sigma_i64 target);

// KMP String Search — O(N + M)
int sigma_kmp_search(const char* text, const char* pattern);
```

### Data Structures

| Structure | Implementation File | Description |
|-----------|---------------------|-------------|
| Dynamic Array | `sigma_std.c` | Growable buffer with amortized O(1) push |
| Linked List | `sigma_std.c` | Singly linked node chain |
| Hash Map | `sigma_std.c` | FNV-1a hash with open addressing |
| Binary Heap | `SovereignSuperCalculator.c` | Min/Max heap for priority queues |
| Graph (Adj. List) | `SovereignSuperCalculator.c` | BFS, DFS, Dijkstra's algorithm |
| Trie | `SovereignSearch.c` | Prefix tree for shell autocomplete |

---

## ML Mathematics

### Linear Algebra (Native C11)

```c
// Matrix multiply: C = A × B
void sigma_matmul(sigma_f64* A, sigma_f64* B, sigma_f64* C,
                  sigma_u32 m, sigma_u32 k, sigma_u32 n);

// Dot product
sigma_f64 sigma_dot(sigma_f64* a, sigma_f64* b, sigma_u32 n);

// Softmax activation
void sigma_softmax(sigma_f64* logits, sigma_u32 n);

// ReLU activation
void sigma_relu(sigma_f64* arr, sigma_u32 n);
```

### Gradient Descent Kernel

```c
// Single SGD step
void sigma_sgd_step(sigma_f64* weights, sigma_f64* grads,
                    sigma_u32 n, sigma_f64 lr);

// MSE Loss
sigma_f64 sigma_mse_loss(sigma_f64* pred, sigma_f64* target, sigma_u32 n);
```

---

## Post-Quantum Cryptography (`SovereignLatticePQC.c`)

### LWE (Learning With Errors) Lattice

```c
// Key generation
void sigma_lwe_keygen(sigma_i32* A, sigma_i32* s, sigma_i32* e,
                      sigma_i32* b, sigma_u32 n, sigma_i32 q);

// Encryption
void sigma_lwe_encrypt(sigma_i32* A, sigma_i32* b, sigma_i32 msg,
                       sigma_i32* u, sigma_i32* v, sigma_u32 n, sigma_i32 q);

// Decryption
sigma_i32 sigma_lwe_decrypt(sigma_i32* s, sigma_i32* u, sigma_i32 v,
                             sigma_u32 n, sigma_i32 q);
```

---

## Calculator Engine (`SovereignSuperCalculator.c`)

| Function | Description |
|----------|-------------|
| `sigma_pow(base, exp)` | Integer exponentiation |
| `sigma_sqrt_approx(x)` | Newton-Raphson square root |
| `sigma_log2(n)` | Bit-shift log base 2 |
| `sigma_gcd(a, b)` | Euclidean GCD |
| `sigma_lcm(a, b)` | LCM via GCD |
| `sigma_is_prime(n)` | Miller-Rabin primality test |
| `sigma_factorial(n)` | Iterative factorial |
| `sigma_fibonacci(n)` | Matrix exponentiation O(log N) |

---

## NCERT Education Labs

### Biology (`ncert_shard.c`)
- Needleman-Wunsch DNA sequence alignment
- Cell mitosis simulation step-by-step
- Ecosystem predator-prey ODE solver (Lotka-Volterra)

### Mathematics (`ncert_shard.c`)
- Coordinate geometry: distance, slope, midpoint
- Matrix determinant and inverse (2×2, 3×3)
- Trigonometry: sin/cos/tan via Taylor series (no `<math.h>`)
- Calculus: numerical differentiation and integration (Simpson's Rule)
