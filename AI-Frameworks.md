# SigmaOS: AI Frameworks & Runtime Integration Roadmap

This document outlines the architectural roadmap for bringing machine learning pipelines, pipelines tracking, and cognitive engines to SigmaOS.

## Target Repositories for Absorption

1. **`mlpack/mlpack` & `opencog/opencog`**
   - **Goal:** Fast C++ ML math and cognitive artificial general intelligence.
   - **SigmaOS Integration:** Adapt mlpack's header-only linear algebra optimizations and OpenCog's AtomSpace semantic network models to run on top of our `no_std` matrix math and graph allocators.

2. **`openai/whisper`**
   - **Goal:** Robust speech-to-text translation.
   - **SigmaOS Integration:** Support executing quantized Whisper GGUF models directly within the `local_llm.rs` stack to enable system voice input commands.

3. **`dvc/dvc` & `mlflow/mlflow`**
   - **Goal:** Data Version Control and ML experiment tracking.
   - **SigmaOS Integration:** Bind tracking hooks into the VFS and `sigpkg` system to snapshot model training states automatically using SovereignFS CoW.

### Last Updated: July 2026
