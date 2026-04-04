# Σ SIGMAOS: ZERO-DEPENDENCY ALGORITHMS & PROCEDURES
[![Domain](https://img.shields.io/badge/Domain-ALGORITHMS-00d2ff?style=for-the-badge)]()

**SigmaOS** replaces standard C libraries (`stdlib.h`, `string.h`) with internal, user-defined algorithmic equivalents to ensure absolute autonomy. The execution graph contains highly optimized routines structured for industrial domains.

## 🧬 Needleman-Wunsch Global Alignment (`bioshard`)
*   **Procedure**: Dynamic programming algorithm for scoring string alignment in bioinformatics.
*   **Application**: DNA sequence tracking matching the exact characters of strings without calling Regex.
*   **Space**: Configured for local bounded length arrays to eliminate heap allocation tracking (No `malloc()`).

## 📊 Volume-Weighted Average Price (VWAP) (`hftshard`)
*   **Procedure**: Aggregates market prices multiplied by transacted volume over sequential intervals.
*   **Formula**: $\text{VWAP} = \frac{\sum(\text{Price} \times \text{Volume})}{\sum(\text{Volume})}$
*   **Execution**: Zero-latency loop using native float multipliers (`kernel/shards/SovereignHFT.c`).

## 🧠 Transformer Self-Attention Scoring (`llmshard`)
*   **Procedure**: Calculates dot-products between Query ($Q$) and Key Transpose ($K^T$) matrices.
*   **Time Complexity**: $O(N^2 \cdot D)$; $N$ is sequence length, $D$ is embedding dimension.
*   **Execution**: Employs raw nested `for` loops within the kernel. Avoiding BLAS or cuBLAS ensures that the primitive transformer block is fundamentally owned by your silicon, completely disentangled from major corporate SDKs.

## 🔄 In-Place Quicksort (`dsashard`)
*   **Procedure**: A pure implementation of recursive array division utilizing the `sigma_partition()` sub-routine.
*   **Usage**: Fully replaces `qsort()`, placing boundary controls correctly outside system-level vulnerabilities like buffer overflow exploitation often found in outdated SDKs.

---
**Σ SIGMAOS: YOUR KERNEL. YOUR ALGORITHMS. FULL SOVEREIGNTY.**
