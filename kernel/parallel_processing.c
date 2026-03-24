/*
 * SigmaOS Parallel Processing
 * ===========================
 * High-performance parallel processing and threading
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// Thread pool configuration
#define MAX_THREADS 64
#define TASK_QUEUE_SIZE 1024

// Task structure for parallel execution
typedef struct {
    void (*function)(void*);
    void* argument;
    void* result;
    bool completed;
    uint64_t task_id;
    uint32_t priority;
    uint64_t submit_time;
    uint64_t start_time;
    uint64_t completion_time;
} ParallelTask;

// Thread structure
typedef struct {
    uint32_t thread_id;
    void* thread_handle;
    ParallelTask* current_task;
    uint32_t task_count;
    uint64_t total_execution_time;
    bool is_active;
    uint32_t cpu_affinity;
} WorkerThread;

// Thread pool structure
typedef struct {
    WorkerThread threads[MAX_THREADS];
    ParallelTask task_queue[TASK_QUEUE_SIZE];
    volatile uint32_t queue_head;
    volatile uint32_t queue_tail;
    volatile uint32_t active_threads;
    uint32_t thread_count;
    uint32_t min_threads;
    uint32_t max_threads;
    bool shutdown_requested;
    uint64_t task_counter;
    uint64_t completed_tasks;
    uint64_t total_wait_time;
    uint64_t total_execution_time;
} ThreadPool;

// Work-stealing queue for load balancing
typedef struct {
    ParallelTask* tasks[TASK_QUEUE_SIZE];
    volatile uint32_t head;
    volatile uint32_t tail;
    volatile uint32_t steal_count;
    uint32_t owner_thread;
} WorkStealingQueue;

// Parallel reduction operations
typedef struct {
    void* data;
    size_t element_size;
    size_t count;
    void (*reduce_function)(void*, void*, void*);
    void* identity_element;
} ParallelReduction;

// Parallel map-reduce framework
typedef struct {
    ParallelTask* map_tasks[MAX_THREADS];
    void* map_results[MAX_THREADS];
    void* reduce_result;
    size_t chunk_size;
    void (*map_function)(void*, void*, void*);
    void (*reduce_function)(void*, void*, void*);
} MapReduceJob;

// Thread-local storage
typedef struct {
    void* data[MAX_THREADS];
    uint32_t keys[MAX_THREADS];
    size_t sizes[MAX_THREADS];
    uint32_t count;
} ThreadLocalStorage;

// CPU affinity and NUMA support
typedef struct {
    uint32_t cpu_id;
    uint32_t node_id;
    uint32_t core_id;
    uint32_t thread_id;
    bool is_hyperthreaded;
    uint64_t cache_line_size;
    uint64_t cache_size;
    uint64_t memory_bandwidth;
} CPUInfo;

// Lock-free task queue
typedef struct LockFreeTaskQueue {
    ParallelTask* buffer[TASK_QUEUE_SIZE];
    volatile uint64_t head;
    volatile uint64_t tail;
    uint64_t mask;
} LockFreeTaskQueue;

// Thread pool implementation
ThreadPool* sigma_thread_pool_create(uint32_t min_threads, uint32_t max_threads) {
    ThreadPool* pool = (ThreadPool*)calloc(1, sizeof(ThreadPool));
    if (!pool) return NULL;
    
    pool->min_threads = min_threads;
    pool->max_threads = max_threads;
    pool->thread_count = min_threads;
    pool->queue_head = 0;
    pool->queue_tail = 0;
    pool->active_threads = 0;
    pool->shutdown_requested = false;
    pool->task_counter = 0;
    pool->completed_tasks = 0;
    pool->total_wait_time = 0;
    pool->total_execution_time = 0;
    
    // Create worker threads
    for (uint32_t i = 0; i < pool->thread_count; i++) {
        WorkerThread* thread = &pool->threads[i];
        thread->thread_id = i;
        thread->current_task = NULL;
        thread->task_count = 0;
        thread->total_execution_time = 0;
        thread->is_active = false;
        thread->cpu_affinity = i;
        
        // Create OS thread (platform-specific)
        thread->thread_handle = sigma_create_thread(worker_thread_func, thread);
        if (!thread->thread_handle) {
            // Cleanup and return error
            for (uint32_t j = 0; j < i; j++) {
                sigma_destroy_thread(pool->threads[j].thread_handle);
            }
            free(pool);
            return NULL;
        }
    }
    
    return pool;
}

static void* worker_thread_func(void* arg) {
    WorkerThread* thread = (WorkerThread*)arg;
    uint64_t start_time = sigma_get_timestamp();
    
    // Set CPU affinity
    sigma_set_thread_affinity(thread->cpu_affinity);
    
    while (true) {
        ThreadPool* pool = get_thread_pool();
        
        // Check for shutdown
        if (pool->shutdown_requested) {
            break;
        }
        
        // Get next task
        ParallelTask* task = sigma_thread_pool_get_task(pool);
        if (!task) {
            // No tasks available, wait
            sigma_thread_yield();
            continue;
        }
        
        // Execute task
        thread->current_task = task;
        thread->is_active = true;
        task->start_time = sigma_get_timestamp();
        
        task->function(task->argument);
        
        task->completion_time = sigma_get_timestamp();
        task->completed = true;
        
        thread->current_task = NULL;
        thread->is_active = false;
        thread->task_count++;
        thread->total_execution_time += task->completion_time - task->start_time;
        
        // Update pool statistics
        pool->completed_tasks++;
        pool->total_execution_time += task->completion_time - task->start_time;
        pool->total_wait_time += task->start_time - task->submit_time;
    }
    
    return NULL;
}

ParallelTask* sigma_thread_pool_get_task(ThreadPool* pool) {
    uint32_t head = pool->queue_head;
    uint32_t tail = pool->queue_tail;
    
    if (head == tail) {
        return NULL; // Queue is empty
    }
    
    ParallelTask* task = &pool->task_queue[head];
    pool->queue_head = (head + 1) % TASK_QUEUE_SIZE;
    
    return task;
}

bool sigma_thread_pool_submit(ThreadPool* pool, void (*function)(void*), void* argument, 
                        uint32_t priority) {
    uint32_t tail = pool->queue_tail;
    uint32_t next_tail = (tail + 1) % TASK_QUEUE_SIZE;
    
    // Check if queue is full
    if (next_tail == pool->queue_head) {
        return false; // Queue is full
    }
    
    ParallelTask* task = &pool->task_queue[tail];
    task->function = function;
    task->argument = argument;
    task->result = NULL;
    task->completed = false;
    task->task_id = pool->task_counter++;
    task->priority = priority;
    task->submit_time = sigma_get_timestamp();
    task->start_time = 0;
    task->completion_time = 0;
    
    pool->queue_tail = next_tail;
    
    // Wake up worker thread if needed
    if (pool->active_threads < pool->thread_count) {
        sigma_thread_wakeup();
    }
    
    return true;
}

// Work-stealing queue implementation
WorkStealingQueue* sigma_work_stealing_queue_create(uint32_t owner_thread) {
    WorkStealingQueue* queue = (WorkStealingQueue*)malloc(sizeof(WorkStealingQueue));
    if (!queue) return NULL;
    
    queue->head = 0;
    queue->tail = 0;
    queue->steal_count = 0;
    queue->owner_thread = owner_thread;
    
    return queue;
}

bool sigma_work_stealing_queue_push(WorkStealingQueue* queue, ParallelTask* task) {
    uint32_t tail = queue->tail;
    uint32_t next_tail = (tail + 1) % TASK_QUEUE_SIZE;
    
    // Check if queue is full
    if (next_tail == queue->head) {
        return false;
    }
    
    queue->tasks[tail] = *task;
    queue->tail = next_tail;
    
    return true;
}

ParallelTask* sigma_work_stealing_queue_pop(WorkStealingQueue* queue) {
    uint32_t head = queue->head;
    
    if (head == queue->tail) {
        return NULL; // Queue is empty
    }
    
    ParallelTask* task = &queue->tasks[head];
    queue->head = (head + 1) % TASK_QUEUE_SIZE;
    
    return task;
}

ParallelTask* sigma_work_stealing_queue_steal(WorkStealingQueue* queue) {
    // Try to steal from the tail of another thread's queue
    uint32_t tail = queue->tail;
    
    if (tail == queue->head) {
        return NULL; // Nothing to steal
    }
    
    // Use atomic operation to steal
    uint32_t steal_index = (tail - 1 + TASK_QUEUE_SIZE) % TASK_QUEUE_SIZE;
    ParallelTask* task = &queue->tasks[steal_index];
    
    // Mark as stolen
    queue->steal_count++;
    
    return task;
}

// Parallel reduction implementation
void* sigma_parallel_reduce(ParallelReduction* reduction, ThreadPool* thread_pool) {
    if (!reduction || !thread_pool) return NULL;
    
    size_t chunk_size = reduction->count / thread_pool->thread_count;
    if (chunk_size == 0) chunk_size = 1;
    
    // Create reduction tasks
    ParallelTask* tasks = (ParallelTask*)malloc(thread_pool->thread_count * sizeof(ParallelTask));
    void** results = (void**)malloc(thread_pool->thread_count * sizeof(void*));
    
    for (uint32_t i = 0; i < thread_pool->thread_count; i++) {
        size_t start = i * chunk_size;
        size_t end = (i == thread_pool->thread_count - 1) ? 
                     reduction->count : start + chunk_size;
        
        // Create reduction task data
        struct {
            ParallelReduction* reduction;
            void* data;
            size_t start;
            size_t end;
            void* result;
        }* task_data = malloc(sizeof(struct));
        
        task_data->reduction = reduction;
        task_data->data = reduction->data;
        task_data->start = start;
        task_data->end = end;
        task_data->result = malloc(reduction->element_size);
        
        tasks[i].function = parallel_reduce_worker;
        tasks[i].argument = task_data;
        tasks[i].priority = 0;
        tasks[i].task_id = i;
        
        results[i] = task_data->result;
    }
    
    // Submit tasks
    for (uint32_t i = 0; i < thread_pool->thread_count; i++) {
        sigma_thread_pool_submit(thread_pool, tasks[i].function, tasks[i].argument, 0);
    }
    
    // Wait for completion
    bool all_completed = false;
    while (!all_completed) {
        all_completed = true;
        for (uint32_t i = 0; i < thread_pool->thread_count; i++) {
            if (!tasks[i].completed) {
                all_completed = false;
                break;
            }
        }
        sigma_thread_yield();
    }
    
    // Final reduction
    void* final_result = malloc(reduction->element_size);
    memcpy(final_result, results[0], reduction->element_size);
    
    for (uint32_t i = 1; i < thread_pool->thread_count; i++) {
        reduction->reduce_function(final_result, results[i], final_result);
    }
    
    // Cleanup
    for (uint32_t i = 0; i < thread_pool->thread_count; i++) {
        free(tasks[i].argument);
        free(results[i]);
    }
    
    free(tasks);
    free(results);
    
    return final_result;
}

static void parallel_reduce_worker(void* arg) {
    struct {
        ParallelReduction* reduction;
        void* data;
        size_t start;
        size_t end;
        void* result;
    }* task_data = (struct*)arg;
    
    // Initialize result with identity element
    if (task_data->reduction->identity_element) {
        memcpy(task_data->result, task_data->reduction->identity_element, 
               task_data->reduction->element_size);
    }
    
    // Perform reduction on assigned chunk
    for (size_t i = task_data->start; i < task_data->end; i++) {
        void* element = (uint8_t*)task_data->data + 
                        (i * task_data->reduction->element_size);
        task_data->reduction->reduce_function(task_data->result, element, 
                                       task_data->result);
    }
}

// Parallel map-reduce implementation
void* sigma_parallel_map_reduce(MapReduceJob* job, ThreadPool* thread_pool) {
    if (!job || !thread_pool) return NULL;
    
    // Calculate chunk size
    size_t total_size = job->chunk_size * thread_pool->thread_count;
    size_t chunk_size = job->chunk_size;
    
    // Create map tasks
    for (uint32_t i = 0; i < thread_pool->thread_count; i++) {
        size_t start = i * chunk_size;
        size_t end = (i == thread_pool->thread_count - 1) ? 
                     total_size : start + chunk_size;
        
        // Create map task data
        struct {
            MapReduceJob* job;
            void* input_data;
            size_t start;
            size_t end;
            void* output_data;
        }* task_data = malloc(sizeof(struct));
        
        task_data->job = job;
        task_data->input_data = job->map_tasks[i].argument;
        task_data->start = start;
        task_data->end = end;
        task_data->output_data = malloc(chunk_size * sizeof(void*));
        
        job->map_tasks[i].function = parallel_map_worker;
        job->map_tasks[i].argument = task_data;
        job->map_tasks[i].priority = 0;
        job->map_tasks[i].task_id = i;
        
        job->map_results[i] = task_data->output_data;
    }
    
    // Submit map tasks
    for (uint32_t i = 0; i < thread_pool->thread_count; i++) {
        sigma_thread_pool_submit(thread_pool, job->map_tasks[i].function, 
                              job->map_tasks[i].argument, 0);
    }
    
    // Wait for map completion
    bool all_completed = false;
    while (!all_completed) {
        all_completed = true;
        for (uint32_t i = 0; i < thread_pool->thread_count; i++) {
            if (!job->map_tasks[i].completed) {
                all_completed = false;
                break;
            }
        }
        sigma_thread_yield();
    }
    
    // Perform reduction
    void* final_result = job->reduce_function(job->map_results[0], job->map_results[1], NULL);
    for (uint32_t i = 2; i < thread_pool->thread_count; i++) {
        final_result = job->reduce_function(final_result, job->map_results[i], NULL);
    }
    
    // Cleanup
    for (uint32_t i = 0; i < thread_pool->thread_count; i++) {
        free(job->map_tasks[i].argument);
        free(job->map_results[i]);
    }
    
    return final_result;
}

static void parallel_map_worker(void* arg) {
    struct {
        MapReduceJob* job;
        void* input_data;
        size_t start;
        size_t end;
        void* output_data;
    }* task_data = (struct*)arg;
    
    // Apply map function to each element
    for (size_t i = task_data->start; i < task_data->end; i++) {
        void* element = (void*)((uint8_t*)task_data->input_data + 
                                 (i * sizeof(void*)));
        void* result = malloc(sizeof(void*));
        
        task_data->job->map_function(element, result, NULL);
        
        ((void**)task_data->output_data)[i - task_data->start] = result;
    }
}

// Thread-local storage implementation
ThreadLocalStorage* sigma_thread_local_storage_create(void) {
    ThreadLocalStorage* tls = (ThreadLocalStorage*)calloc(1, sizeof(ThreadLocalStorage));
    if (!tls) return NULL;
    
    return tls;
}

void sigma_thread_local_storage_set(ThreadLocalStorage* tls, uint32_t key, void* data, size_t size) {
    for (uint32_t i = 0; i < tls->count; i++) {
        if (tls->keys[i] == key) {
            // Update existing entry
            free(tls->data[i]);
            tls->data[i] = malloc(size);
            memcpy(tls->data[i], data, size);
            tls->sizes[i] = size;
            return;
        }
    }
    
    // Add new entry
    if (tls->count < MAX_THREADS) {
        tls->keys[tls->count] = key;
        tls->data[tls->count] = malloc(size);
        memcpy(tls->data[tls->count], data, size);
        tls->sizes[tls->count] = size;
        tls->count++;
    }
}

void* sigma_thread_local_storage_get(ThreadLocalStorage* tls, uint32_t key) {
    for (uint32_t i = 0; i < tls->count; i++) {
        if (tls->keys[i] == key) {
            return tls->data[i];
        }
    }
    return NULL;
}

// NUMA-aware memory allocation
typedef struct {
    void** nodes;
    size_t node_count;
    size_t node_size;
    uint32_t current_node;
} NUMAMemoryPool;

NUMAMemoryPool* sigma_numa_memory_pool_create(size_t node_size, size_t node_count) {
    NUMAMemoryPool* pool = (NUMAMemoryPool*)malloc(sizeof(NUMAMemoryPool));
    if (!pool) return NULL;
    
    pool->nodes = (void**)malloc(node_count * sizeof(void*));
    if (!pool->nodes) {
        free(pool);
        return NULL;
    }
    
    for (size_t i = 0; i < node_count; i++) {
        pool->nodes[i] = sigma_numa_alloc_on_node(i, node_size);
    }
    
    pool->node_count = node_count;
    pool->node_size = node_size;
    pool->current_node = 0;
    
    return pool;
}

void* sigma_numa_memory_pool_alloc(NUMAMemoryPool* pool, size_t size) {
    if (size > pool->node_size) return NULL;
    
    // Allocate from current node
    void* ptr = pool->nodes[pool->current_node];
    if (!ptr) {
        // Try other nodes
        for (size_t i = 0; i < pool->node_count; i++) {
            if (pool->nodes[i]) {
                ptr = pool->nodes[i];
                pool->current_node = i;
                break;
            }
        }
    }
    
    return ptr;
}

// Parallel quicksort implementation
static void parallel_quicksort_worker(void* arg) {
    struct {
        int* array;
        int left;
        int right;
        int depth;
        ThreadPool* thread_pool;
    }* task_data = (struct*)arg;
    
    if (task_data->depth <= 0 || task_data->right - task_data->left <= 1000) {
        // Use sequential sort for small arrays
        sigma_introsort(task_data->array + task_data->left, 
                     task_data->right - task_data->left);
        return;
    }
    
    int pivot = task_data->array[(task_data->left + task_data->right) / 2];
    int i = task_data->left, j = task_data->right;
    
    while (i <= j) {
        while (task_data->array[i] < pivot) i++;
        while (task_data->array[j] > pivot) j--;
        
        if (i <= j) {
            int temp = task_data->array[i];
            task_data->array[i] = task_data->array[j];
            task_data->array[j] = temp;
            i++;
            j--;
        }
    }
    
    // Create subtasks for parallel execution
    if (task_data->depth > 1) {
        // Left subarray
        struct {
            int* array;
            int left;
            int right;
            int depth;
            ThreadPool* thread_pool;
        }* left_task = malloc(sizeof(struct));
        
        left_task->array = task_data->array;
        left_task->left = task_data->left;
        left_task->right = j;
        left_task->depth = task_data->depth - 1;
        left_task->thread_pool = task_data->thread_pool;
        
        // Right subarray
        struct {
            int* array;
            int left;
            int right;
            int depth;
            ThreadPool* thread_pool;
        }* right_task = malloc(sizeof(struct));
        
        right_task->array = task_data->array;
        right_task->left = j + 1;
        right_task->right = task_data->right;
        right_task->depth = task_data->depth - 1;
        right_task->thread_pool = task_data->thread_pool;
        
        // Submit subtasks
        sigma_thread_pool_submit(task_data->thread_pool, parallel_quicksort_worker, 
                              left_task, 0);
        sigma_thread_pool_submit(task_data->thread_pool, parallel_quicksort_worker, 
                              right_task, 0);
        
        // Wait for completion
        while (!left_task->completed || !right_task->completed) {
            sigma_thread_yield();
        }
        
        free(left_task);
        free(right_task);
    } else {
        // Sequential execution
        sigma_introsort(task_data->array + task_data->left, 
                     task_data->right - task_data->left);
        sigma_introsort(task_data->array + j + 1, 
                     task_data->right - j);
    }
}

void sigma_parallel_quicksort(int* array, int size, ThreadPool* thread_pool) {
    if (!array || size <= 1 || !thread_pool) return;
    
    struct {
        int* array;
        int left;
        int right;
        int depth;
        ThreadPool* thread_pool;
    }* task_data = malloc(sizeof(struct));
    
    task_data->array = array;
    task_data->left = 0;
    task_data->right = size - 1;
    task_data->depth = (int)log2(size);
    task_data->thread_pool = thread_pool;
    
    sigma_thread_pool_submit(thread_pool, parallel_quicksort_worker, task_data, 0);
    
    // Wait for completion
    while (!task_data->completed) {
        sigma_thread_yield();
    }
    
    free(task_data);
}

// Performance monitoring
typedef struct {
    uint64_t total_tasks;
    uint64_t completed_tasks;
    uint64_t failed_tasks;
    uint64_t total_wait_time;
    uint64_t total_execution_time;
    uint64_t average_wait_time;
    uint64_t average_execution_time;
    uint32_t active_threads;
    uint32_t peak_threads;
    double throughput;
} ThreadPoolStats;

ThreadPoolStats* sigma_thread_pool_get_stats(ThreadPool* pool) {
    ThreadPoolStats* stats = (ThreadPoolStats*)malloc(sizeof(ThreadPoolStats));
    if (!stats) return NULL;
    
    stats->total_tasks = pool->task_counter;
    stats->completed_tasks = pool->completed_tasks;
    stats->failed_tasks = pool->task_counter - pool->completed_tasks;
    stats->total_wait_time = pool->total_wait_time;
    stats->total_execution_time = pool->total_execution_time;
    stats->active_threads = pool->active_threads;
    stats->peak_threads = pool->thread_count;
    
    if (stats->completed_tasks > 0) {
        stats->average_wait_time = stats->total_wait_time / stats->completed_tasks;
        stats->average_execution_time = stats->total_execution_time / stats->completed_tasks;
    } else {
        stats->average_wait_time = 0;
        stats->average_execution_time = 0;
    }
    
    // Calculate throughput (tasks per second)
    uint64_t current_time = sigma_get_timestamp();
    uint64_t runtime = current_time - pool->start_time;
    if (runtime > 0) {
        stats->throughput = (double)stats->completed_tasks / (runtime / 1000000.0);
    } else {
        stats->throughput = 0.0;
    }
    
    return stats;
}

// Cleanup functions
void sigma_thread_pool_destroy(ThreadPool* pool) {
    if (!pool) return;
    
    // Signal shutdown
    pool->shutdown_requested = true;
    
    // Wait for all threads to finish
    for (uint32_t i = 0; i < pool->thread_count; i++) {
        if (pool->threads[i].thread_handle) {
            sigma_join_thread(pool->threads[i].thread_handle);
        }
    }
    
    free(pool);
}

void sigma_work_stealing_queue_destroy(WorkStealingQueue* queue) {
    if (queue) free(queue);
}

void sigma_thread_local_storage_destroy(ThreadLocalStorage* tls) {
    if (!tls) return;
    
    for (uint32_t i = 0; i < tls->count; i++) {
        if (tls->data[i]) free(tls->data[i]);
    }
    
    free(tls);
}

void sigma_numa_memory_pool_destroy(NUMAMemoryPool* pool) {
    if (!pool) return;
    
    for (size_t i = 0; i < pool->node_count; i++) {
        if (pool->nodes[i]) {
            sigma_numa_free(pool->nodes[i]);
        }
    }
    
    free(pool->nodes);
    free(pool);
}
