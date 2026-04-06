# 🎓 NCERT Education & EdTech Shards

SigmaOS is the **first sovereign operating system with a fully integrated NCERT-aligned education layer** — no external apps, no internet, just pure silicon-local interactive learning.

---

## Education Architecture

```text
User selects Subject
    └─► ncert_shard.c loaded by SOD system
            ├─► Biology Lab  (DNA alignment, cell simulation)
            ├─► Math Lab     (algebra, calculus, trigonometry)
            ├─► Physics Lab  (mechanics, waves, optics ODE)
            ├─► Chemistry    (periodic table, reaction balancer)
            └─► AI Tutor     (context-aware hints + quiz generation)
```

---

## Biology Lab (Class 11/12 NCERT)

### DNA Sequence Alignment

```c
// Needleman-Wunsch global alignment
void sigma_nw_align(const char* seq1, const char* seq2,
                    int match, int mismatch, int gap,
                    char* aligned1, char* aligned2);

// Example
sigma_nw_align("AGCTAG", "AGCTTG", 1, -1, -2, out1, out2);
// out1: AGCT-AG
// out2: AGCTTG-
```

### Ecosystem Simulator (Lotka-Volterra ODE)

```c
// Predator-prey population dynamics
void sigma_lotka_volterra(sigma_f64 prey0, sigma_f64 pred0,
                          sigma_f64 alpha, sigma_f64 beta,
                          sigma_f64 gamma, sigma_f64 delta,
                          sigma_u32 steps, sigma_f64 dt,
                          sigma_f64* prey_out, sigma_f64* pred_out);
```

---

## Mathematics Lab (Class 11/12 NCERT)

### Implemented Functions (No `<math.h>`)

```c
// Trigonometry via Taylor Series
sigma_f64 sigma_sin(sigma_f64 x);     // 12-term Taylor expansion
sigma_f64 sigma_cos(sigma_f64 x);     // 12-term Taylor expansion
sigma_f64 sigma_tan(sigma_f64 x);     // sin/cos ratio

// Calculus
sigma_f64 sigma_derivative(sigma_f64 (*f)(sigma_f64), sigma_f64 x, sigma_f64 h);
sigma_f64 sigma_integrate_simpson(sigma_f64 (*f)(sigma_f64), sigma_f64 a, sigma_f64 b, sigma_u32 n);

// Matrix Operations (2x2, 3x3)
sigma_f64 sigma_det2x2(sigma_f64 a[2][2]);
sigma_f64 sigma_det3x3(sigma_f64 a[3][3]);
void sigma_inverse2x2(sigma_f64 a[2][2], sigma_f64 out[2][2]);

// Number Theory
int sigma_is_prime(sigma_u64 n);      // Miller-Rabin
sigma_u64 sigma_gcd(sigma_u64 a, sigma_u64 b);
sigma_u64 sigma_lcm(sigma_u64 a, sigma_u64 b);
```

---

## Physics Lab (Class 11/12 NCERT)

| Simulation | Method | Description |
| --- | --- | --- |
| Projectile Motion | Euler integration | 2D trajectory with drag |
| Simple Harmonic Motion | Analytical ODE | Spring-mass oscillator |
| Wave Interference | Superposition principle | Double-slit visualization |
| Optics: Refraction | Snell's law | Ray tracing simulation |
| Thermodynamics | Ideal Gas Law | P-V-T state calculator |

---

## AI Tutor Integration

Every NCERT shard connects to the local `Sigma_Tutor` AI model:

- **Contextual Hints**: If a student pauses on an alignment problem, the tutor explains the dynamic programming table step-by-step
- **Adaptive Quizzes**: Generates 5 new questions based on topics the student got wrong
- **Spaced Repetition**: Automation shard schedules review sessions at optimal memory consolidation intervals
- **Difficulty Scaling**: Tracks performance and adjusts question complexity automatically

---

## NCERT Shard vs External EdTech Apps

| Feature | GeoGebra / GCompris | SigmaOS NCERT Shard |
| --- | --- | --- |
| Installation required | Yes | No — built into OS |
| Internet needed | Sometimes | Never |
| AI tutor | No | Yes — local, no API |
| NCERT curriculum aligned | Partial | Full (Class 11/12) |
| Forensic/legal compliance | No | BNSS-ready evidence logs |
| Custom simulation | No | Native C11 ODE solver |

---

## Roadmap

- [ ] Class 9/10 NCERT coverage expansion
- [ ] 3D chemistry molecule viewer (canvas renderer)
- [ ] Interactive periodic table with electron shell animations
- [ ] Competitive exam (JEE/NEET) practice mode
- [ ] Offline AI essay grader for CBSE answers
