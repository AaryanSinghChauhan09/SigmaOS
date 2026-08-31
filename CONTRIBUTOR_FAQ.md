# 🙋 SigmaOS Contributor Frequently Asked Questions (FAQ)

Welcome to the SigmaOS Developer FAQ! This guide answers common technical questions and outlines coding guidelines for writing secure, AI-native, capability-based microkernel code.

***

## 🏗️ 1. Architecture Questions

### What makes SigmaOS "AI-native"?

SigmaOS integrates AI security models and local resource orchestration directly into core submodules. Instead of relying on cloud LLM APIs, it ships with:

*   **`LocalModelRegistry`:** On-device registry (similar to Ollama) for model lookups.
*   **`SemanticQueryEngine`:** Zero-dependency, on-device vector database using true mathematically robust Cosine Similarity for RAG (Retrieval-Augmented Generation) context retrieval.
*   **`OpenShellAgentSandbox`:** Sandboxes AI agent executions to prevent prompt injections and system command escapes.

### Why is the codebase strictly `#![no_std]`?

SigmaOS aims to maintain an ultra-small memory and disk footprint so it can run on legacy x86/ARM devices as well as modern multi-core NVMe systems. Bypassing the standard library prevents unused runtime overhead and guarantees predictable real-time execution.

***

## 🔒 2. Memory & Custom Types

### Why do I need to implement `Drop` on custom `Vec<T>` structs?

In `#![no_std]` environments, when managing raw memory heaps directly, standard compiler-derived drop handlers do not know how to free resources.
You **must** implement `Drop` to:

1.  Call `core::ptr::drop_in_place` on all initialized elements.
2.  Deallocate the backing memory buffer using the allocator shim (`free`).

Failure to do so results in severe memory leaks under continuous bare-metal operations.

***

## 🛠️ 3. India Stack Integration

### How does the India Stack layer work?

Exposed in `src/compatibility/india_stack.rs`, this suite provides first-class sovereign Indian technologies:

*   **Mock UPI Service:** Cryptographically signs virtual transactions and formats standard VPA QR payments (`upi://pay?...`).
*   **GST Tax Engine:** Computes Central (CGST), State (SGST), and Integrated (IGST) slabs at official tax rates.
*   **Indic Multilingualism:** Direct, zero-dependency UTF-8 translation arrays for Hindi, Tamil, and Sanskrit.

***

## 🚀 4. Virtual Memory & Paging

### How does Copy-on-Write (COW) and Transparent Huge Pages (THP) work?

SigmaOS implements standard 4-level paging:

*   **COW:** Marks pages as read-only. On write-faults, the interrupt handler intercept clones the physical frame and remaps it as writable.
*   **THP:** Maps 2MB blocks directly at the Page Directory level (setting bit 7 `PS`), bypassing PT tables to drastically reduce TLB misses for massive machine learning weights.
*   **TLB Shootdowns:** Utilizes `TlbTracker` to register changes and invalidate translation caches across multi-core systems.
