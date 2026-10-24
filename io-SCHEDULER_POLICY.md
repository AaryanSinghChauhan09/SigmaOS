# SigmaOS I/O Scheduler Policy

## 1. Subsystem Architecture
The I/O Scheduler (`src/io/scheduler.rs` and `src/io/scheduler_queue.rs`) organizes, prioritizes, and dispatches pending I/O requests (`IoRequest`) to underlying storage and device drivers.

## 2. Key Rules & Guarantees

1. **Race-Free Enqueue/Dequeue**:
   - Queue insertions and extractions are protected by internal synchronization (`Spinlock` / mutex).
   - Priority-aware scheduling classes (Low, Normal, High, Realtime) prevent starvation of lower-priority requests via aging algorithms.

2. **Defined I/O Reuse Policy**:
   - Completed requests are recycled into the pre-allocated request pool.
   - Request IDs are monotonically incremented and verified against duplicate submissions.

3. **Bounded Metadata**:
   - Queue capacity is strictly bounded (`MAX_SCHEDULER_QUEUE_DEPTH = 512`).

4. **Lost-I/O & Double-I/O Prevention**:
   - Each request contains a stage status (`IoStage::Submitted`, `IoStage::Validated`, `IoStage::Enqueued`, `IoStage::Executing`, `IoStage::Completed`).
   - Re-dispatching an executing or completed request returns `SchedulerError::InvalidStageTransition`.

5. **OOM & Resource Enforcement**:
   - Exceeding maximum queue depth yields `SchedulerError::QueueFull`.
