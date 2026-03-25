/*
 * SigmaOS Advanced Algorithms
 * ===========================
 * High-performance algorithms for critical OS operations
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

// Advanced data structures
typedef struct {
    int* data;
    size_t size;
    size_t capacity;
    size_t front;
    size_t rear;
    size_t count;
} CircularBuffer;

typedef struct {
    void** heap;
    size_t size;
    size_t capacity;
    int (*compare)(const void*, const void*);
} BinaryHeap;

typedef struct {
    int key;
    void* value;
    int height;
    struct AVLNode* left;
    struct AVLNode* right;
} AVLNode;

typedef struct {
    int key;
    void* value;
    bool color; // RED = true, BLACK = false
    struct RBNode* left;
    struct RBNode* right;
    struct RBNode* parent;
} RBNode;

// Lock-free stack
typedef struct LockFreeStackNode {
    void* data;
    struct LockFreeStackNode* next;
} LockFreeStackNode;

typedef struct {
    LockFreeStackNode* head;
    size_t size;
} LockFreeStack;

// Bloom filter for fast membership testing
typedef struct {
    uint64_t* bit_array;
    size_t bit_count;
    size_t hash_count;
    uint32_t (*hash_functions[8])(const void*, size_t);
} BloomFilter;

// Skip list for concurrent access
typedef struct SkipListNode {
    int key;
    void* value;
    struct SkipListNode** forward;
    int level;
} SkipListNode;

typedef struct {
    SkipListNode* header;
    int max_level;
    size_t size;
} SkipList;

// Circular buffer implementation
CircularBuffer* sigma_circular_buffer_create(size_t capacity) {
    CircularBuffer* buffer = (CircularBuffer*)malloc(sizeof(CircularBuffer));
    if (!buffer) return NULL;
    
    buffer->data = (int*)malloc(capacity * sizeof(int));
    if (!buffer->data) {
        free(buffer);
        return NULL;
    }
    
    buffer->size = 0;
    buffer->capacity = capacity;
    buffer->front = 0;
    buffer->rear = 0;
    buffer->count = 0;
    
    return buffer;
}

bool sigma_circular_buffer_enqueue(CircularBuffer* buffer, int item) {
    if (buffer->count >= buffer->capacity) return false;
    
    buffer->data[buffer->rear] = item;
    buffer->rear = (buffer->rear + 1) % buffer->capacity;
    buffer->count++;
    
    return true;
}

bool sigma_circular_buffer_dequeue(CircularBuffer* buffer, int* item) {
    if (buffer->count == 0) return false;
    
    *item = buffer->data[buffer->front];
    buffer->front = (buffer->front + 1) % buffer->capacity;
    buffer->count--;
    
    return true;
}

bool sigma_circular_buffer_is_empty(const CircularBuffer* buffer) {
    return buffer->count == 0;
}

bool sigma_circular_buffer_is_full(const CircularBuffer* buffer) {
    return buffer->count >= buffer->capacity;
}

// Binary heap implementation
BinaryHeap* sigma_binary_heap_create(size_t capacity, int (*compare)(const void*, const void*)) {
    BinaryHeap* heap = (BinaryHeap*)malloc(sizeof(BinaryHeap));
    if (!heap) return NULL;
    
    heap->heap = (void**)malloc(capacity * sizeof(void*));
    if (!heap->heap) {
        free(heap);
        return NULL;
    }
    
    heap->size = 0;
    heap->capacity = capacity;
    heap->compare = compare;
    
    return heap;
}

static void sigma_binary_heap_sift_up(BinaryHeap* heap, size_t index) {
    while (index > 0) {
        size_t parent = (index - 1) / 2;
        
        if (heap->compare(heap->heap[index], heap->heap[parent]) >= 0) break;
        
        // Swap with parent
        void* temp = heap->heap[index];
        heap->heap[index] = heap->heap[parent];
        heap->heap[parent] = temp;
        
        index = parent;
    }
}

static void sigma_binary_heap_sift_down(BinaryHeap* heap, size_t index) {
    size_t size = heap->size;
    
    while (true) {
        size_t left = 2 * index + 1;
        size_t right = 2 * index + 2;
        size_t smallest = index;
        
        if (left < size && heap->compare(heap->heap[left], heap->heap[smallest]) < 0) {
            smallest = left;
        }
        
        if (right < size && heap->compare(heap->heap[right], heap->heap[smallest]) < 0) {
            smallest = right;
        }
        
        if (smallest == index) break;
        
        // Swap with smallest child
        void* temp = heap->heap[index];
        heap->heap[index] = heap->heap[smallest];
        heap->heap[smallest] = temp;
        
        index = smallest;
    }
}

bool sigma_binary_heap_insert(BinaryHeap* heap, void* item) {
    if (heap->size >= heap->capacity) return false;
    
    heap->heap[heap->size] = item;
    sigma_binary_heap_sift_up(heap, heap->size);
    heap->size++;
    
    return true;
}

void* sigma_binary_heap_extract_min(BinaryHeap* heap) {
    if (heap->size == 0) return NULL;
    
    void* min_item = heap->heap[0];
    heap->heap[0] = heap->heap[heap->size - 1];
    heap->size--;
    sigma_binary_heap_sift_down(heap, 0);
    
    return min_item;
}

void* sigma_binary_heap_peek(const BinaryHeap* heap) {
    return (heap->size > 0) ? heap->heap[0] : NULL;
}

// AVL tree implementation
static int avl_node_height(AVLNode* node) {
    return node ? node->height : 0;
}

static int avl_balance_factor(AVLNode* node) {
    return node ? avl_node_height(node->left) - avl_node_height(node->right) : 0;
}

static AVLNode* avl_rotate_right(AVLNode* y) {
    AVLNode* x = y->left;
    AVLNode* T2 = x->right;
    
    x->right = y;
    y->left = T2;
    
    y->height = 1 + (avl_node_height(y->left) > avl_node_height(y->right) ? 
                       avl_node_height(y->left) : avl_node_height(y->right));
    x->height = 1 + (avl_node_height(x->left) > avl_node_height(x->right) ? 
                       avl_node_height(x->left) : avl_node_height(x->right));
    
    return x;
}

static AVLNode* avl_rotate_left(AVLNode* x) {
    AVLNode* y = x->right;
    AVLNode* T2 = y->left;
    
    y->left = x;
    x->right = T2;
    
    x->height = 1 + (avl_node_height(x->left) > avl_node_height(x->right) ? 
                       avl_node_height(x->left) : avl_node_height(x->right));
    y->height = 1 + (avl_node_height(y->left) > avl_node_height(y->right) ? 
                       avl_node_height(y->left) : avl_node_height(y->right));
    
    return y;
}

static AVLNode* avl_insert_node(AVLNode* node, int key, void* value) {
    if (!node) {
        AVLNode* new_node = (AVLNode*)malloc(sizeof(AVLNode));
        new_node->key = key;
        new_node->value = value;
        new_node->height = 1;
        new_node->left = NULL;
        new_node->right = NULL;
        return new_node;
    }
    
    if (key < node->key) {
        node->left = avl_insert_node(node->left, key, value);
    } else if (key > node->key) {
        node->right = avl_insert_node(node->right, key, value);
    } else {
        return node; // Duplicate key
    }
    
    // Update height
    node->height = 1 + (avl_node_height(node->left) > avl_node_height(node->right) ? 
                       avl_node_height(node->left) : avl_node_height(node->right));
    
    // Balance the tree
    int balance = avl_balance_factor(node);
    
    if (balance > 1 && key < node->left->key) {
        return avl_rotate_right(node);
    }
    
    if (balance < -1 && key > node->right->key) {
        return avl_rotate_left(node);
    }
    
    if (balance > 1 && key > node->left->key) {
        node->left = avl_rotate_left(node->left);
        return avl_rotate_right(node);
    }
    
    if (balance < -1 && key < node->right->key) {
        node->right = avl_rotate_right(node->right);
        return avl_rotate_left(node);
    }
    
    return node;
}

// Red-Black tree implementation
static void rb_rotate_left(RBNode** root, RBNode* x) {
    RBNode* y = x->right;
    x->right = y->left;
    
    if (y->left) y->left->parent = x;
    
    y->parent = x->parent;
    
    if (x->parent) {
        if (x == x->parent->left) {
            x->parent->left = y;
        } else {
            x->parent->right = y;
        }
    } else {
        *root = y;
    }
    
    y->left = x;
    x->parent = y;
}

static void rb_rotate_right(RBNode** root, RBNode* y) {
    RBNode* x = y->left;
    y->left = x->right;
    
    if (x->right) x->right->parent = y;
    
    x->parent = y->parent;
    
    if (y->parent) {
        if (y == y->parent->right) {
            y->parent->right = x;
        } else {
            y->parent->left = x;
        }
    } else {
        *root = x;
    }
    
    x->right = y;
    y->parent = x;
}

static void rb_insert_fixup(RBNode** root, RBNode* z) {
    while (z->parent && z->parent->color) { // While parent is RED
        if (z->parent == z->parent->parent->left) {
            RBNode* y = z->parent->parent->right;
            
            if (y && y->color) {
                z->parent->color = false;
                y->color = false;
                z->parent->parent->color = true;
                z = z->parent->parent;
            } else {
                if (z == z->parent->right) {
                    rb_rotate_left(root, z->parent);
                    z = z->left;
                }
                z->parent->color = false;
                z->parent->parent->color = true;
                rb_rotate_right(root, z->parent->parent);
            }
        } else {
            RBNode* y = z->parent->parent->left;
            
            if (y && y->color) {
                z->parent->color = false;
                y->color = false;
                z->parent->parent->color = true;
                z = z->parent->parent;
            } else {
                if (z == z->parent->left) {
                    rb_rotate_right(root, z->parent);
                    z = z->right;
                }
                z->parent->color = false;
                z->parent->parent->color = true;
                rb_rotate_left(root, z->parent->parent);
            }
        }
    }
    
    (*root)->color = false; // Root is always BLACK
}

// Lock-free stack implementation
LockFreeStack* sigma_lockfree_stack_create(void) {
    LockFreeStack* stack = (LockFreeStack*)malloc(sizeof(LockFreeStack));
    if (!stack) return NULL;
    
    stack->head = NULL;
    stack->size = 0;
    
    return stack;
}

bool sigma_lockfree_stack_push(LockFreeStack* stack, void* data) {
    LockFreeStackNode* new_node = (LockFreeStackNode*)malloc(sizeof(LockFreeStackNode));
    if (!new_node) return false;
    
    new_node->data = data;
    
    do {
        new_node->next = stack->head;
    } while (!__sync_bool_compare_and_swap(&stack->head, &new_node->next, new_node));
    
    stack->size++;
    return true;
}

void* sigma_lockfree_stack_pop(LockFreeStack* stack) {
    LockFreeStackNode* head;
    
    do {
        head = stack->head;
        if (!head) return NULL;
    } while (!__sync_bool_compare_and_swap(&stack->head, &head, head->next));
    
    void* data = head->data;
    free(head);
    stack->size--;
    
    return data;
}

// Bloom filter implementation
static uint32_t hash_fnv1(const void* data, size_t size) {
    const uint8_t* bytes = (const uint8_t*)data;
    uint32_t hash = 2166136261U;
    
    for (size_t i = 0; i < size; i++) {
        hash ^= bytes[i];
        hash *= 16777619U;
    }
    
    return hash;
}

static uint32_t hash_murmur3(const void* data, size_t size) {
    const uint8_t* bytes = (const uint8_t*)data;
    uint32_t h = 0;
    
    for (size_t i = 0; i < size; i++) {
        h ^= bytes[i];
        h *= 0x5bd1e995;
        h ^= h >> 15;
    }
    
    return h;
}

BloomFilter* sigma_bloom_filter_create(size_t bit_count, size_t hash_count) {
    BloomFilter* filter = (BloomFilter*)malloc(sizeof(BloomFilter));
    if (!filter) return NULL;
    
    size_t array_size = (bit_count + 63) / 64;
    filter->bit_array = (uint64_t*)calloc(array_size, sizeof(uint64_t));
    if (!filter->bit_array) {
        free(filter);
        return NULL;
    }
    
    filter->bit_count = bit_count;
    filter->hash_count = hash_count;
    
    // Initialize hash functions
    filter->hash_functions[0] = hash_fnv1;
    filter->hash_functions[1] = hash_murmur3;
    // Add more hash functions as needed
    
    return filter;
}

void sigma_bloom_filter_add(BloomFilter* filter, const void* data, size_t size) {
    for (size_t i = 0; i < filter->hash_count; i++) {
        uint32_t hash = filter->hash_functions[i](data, size);
        size_t bit_index = hash % filter->bit_count;
        size_t array_index = bit_index / 64;
        size_t bit_offset = bit_index % 64;
        
        filter->bit_array[array_index] |= (1ULL << bit_offset);
    }
}

bool sigma_bloom_filter_contains(const BloomFilter* filter, const void* data, size_t size) {
    for (size_t i = 0; i < filter->hash_count; i++) {
        uint32_t hash = filter->hash_functions[i](data, size);
        size_t bit_index = hash % filter->bit_count;
        size_t array_index = bit_index / 64;
        size_t bit_offset = bit_index % 64;
        
        if (!(filter->bit_array[array_index] & (1ULL << bit_offset))) {
            return false;
        }
    }
    
    return true;
}

// Skip list implementation
static int random_level(int max_level) {
    int level = 0;
    while (rand() < 0.5 && level < max_level) {
        level++;
    }
    return level;
}

SkipList* sigma_skip_list_create(int max_level) {
    SkipList* list = (SkipList*)malloc(sizeof(SkipList));
    if (!list) return NULL;
    
    // Create header node
    list->header = (SkipListNode*)malloc(sizeof(SkipListNode));
    if (!list->header) {
        free(list);
        return NULL;
    }
    
    list->header->forward = (SkipListNode**)malloc((max_level + 1) * sizeof(SkipListNode*));
    if (!list->header->forward) {
        free(list->header);
        free(list);
        return NULL;
    }
    
    for (int i = 0; i <= max_level; i++) {
        list->header->forward[i] = NULL;
    }
    
    list->header->key = INT_MIN;
    list->header->level = 0;
    list->max_level = max_level;
    list->size = 0;
    
    return list;
}

void sigma_skip_list_insert(SkipList* list, int key, void* value) {
    SkipListNode* update[list->max_level + 1];
    SkipListNode* current = list->header;
    
    // Find insertion point
    for (int i = list->max_level; i >= 0; i--) {
        while (current->forward[i] && current->forward[i]->key < key) {
            current = current->forward[i];
        }
        update[i] = current;
    }
    
    current = current->forward[0];
    if (current && current->key == key) {
        current->value = value; // Update existing key
        return;
    }
    
    // Create new node
    int new_level = random_level(list->max_level);
    SkipListNode* new_node = (SkipListNode*)malloc(sizeof(SkipListNode));
    new_node->forward = (SkipListNode**)malloc((new_level + 1) * sizeof(SkipListNode*));
    new_node->key = key;
    new_node->value = value;
    new_node->level = new_level;
    
    // Update forward pointers
    for (int i = 0; i <= new_level; i++) {
        new_node->forward[i] = update[i]->forward[i];
        update[i]->forward[i] = new_node;
    }
    
    list->size++;
}

void* sigma_skip_list_search(SkipList* list, int key) {
    SkipListNode* current = list->header;
    
    for (int i = list->max_level; i >= 0; i--) {
        while (current->forward[i] && current->forward[i]->key < key) {
            current = current->forward[i];
        }
    }
    
    current = current->forward[0];
    
    return (current && current->key == key) ? current->value : NULL;
}

// Advanced sorting algorithms
static void introsort(int* array, int left, int right, int max_depth) {
    if (max_depth == 0) {
        // Switch to heap sort
        heap_sort(array + left, right - left + 1);
        return;
    }
    
    if (left < right) {
        int pivot = array[(left + right) / 2];
        int i = left, j = right;
        
        while (i <= j) {
            while (array[i] < pivot) i++;
            while (array[j] > pivot) j--;
            
            if (i <= j) {
                int temp = array[i];
                array[i] = array[j];
                array[j] = temp;
                i++;
                j--;
            }
        }
        
        int depth = max_depth - 1;
        introsort(array, left, j, depth);
        introsort(array, i, right, depth);
    }
}

void sigma_introsort(int* array, int size) {
    int max_depth = 2 * (int)log2(size);
    introsort(array, 0, size - 1, max_depth);
}

// Radix sort for integers
void sigma_radix_sort(int* array, int size) {
    if (size <= 1) return;
    
    int max = array[0];
    for (int i = 1; i < size; i++) {
        if (abs(array[i]) > max) max = abs(array[i]);
    }
    
    for (int exp = 1; max / exp > 0; exp *= 10) {
        int* output = (int*)malloc(size * sizeof(int));
        int count[10] = {0};
        
        // Count occurrences
        for (int i = 0; i < size; i++) {
            int digit = (abs(array[i]) / exp) % 10;
            count[digit]++;
        }
        
        // Calculate positions
        for (int i = 1; i < 10; i++) {
            count[i] += count[i - 1];
        }
        
        // Build output array
        for (int i = size - 1; i >= 0; i--) {
            int digit = (abs(array[i]) / exp) % 10;
            output[count[digit] - 1] = array[i];
            count[digit]--;
        }
        
        // Copy back to original array
        for (int i = 0; i < size; i++) {
            array[i] = output[i];
        }
        
        free(output);
    }
}

// Advanced search algorithms
static int fibonacci_search(int* array, int size, int target) {
    int fibMMm2 = 0;
    int fibMMm1 = 1;
    int fibM = fibMMm2 + fibMMm1;
    
    while (fibM < size) {
        fibMMm2 = fibMMm1;
        fibMMm1 = fibM;
        fibM = fibMMm2 + fibMMm1;
    }
    
    int offset = -1;
    
    while (fibM > 1) {
        int i = (offset + fibMMm2 < size) ? offset + fibMMm2 : -1;
        
        if (i < size && array[i] < target) {
            fibM = fibMMm1;
            fibMMm1 = fibMMm2;
            fibMMm2 = fibM - fibMMm1;
            offset = i;
        } else if (i < size && array[i] > target) {
            fibM = fibMMm2;
            fibMMm1 = fibM - fibMMm1;
            fibMMm2 = fibM - fibMMm1;
        } else {
            return i; // Found target
        }
    }
    
    if (fibMMm1 && offset + 1 < size && array[offset + 1] == target) {
        return offset + 1;
    }
    
    return -1; // Not found
}

// Memory-efficient string matching (Knuth-Morris-Pratt)
static int* sigma_kmp_compute_prefix(const char* pattern, int pattern_len, int* prefix_len) {
    int* prefix = (int*)malloc(pattern_len * sizeof(int));
    if (!prefix) return NULL;
    
    prefix[0] = 0;
    int len = 0;
    int i = 1;
    
    while (i < pattern_len) {
        if (pattern[i] == pattern[len]) {
            len++;
            prefix[i] = len;
            i++;
        } else {
            if (len != 0) {
                len = prefix[len - 1];
            } else {
                prefix[i] = 0;
                i++;
            }
        }
    }
    
    *prefix_len = len;
    return prefix;
}

int sigma_kmp_search(const char* text, int text_len, const char* pattern, int pattern_len) {
    if (pattern_len == 0) return 0;
    if (text_len < pattern_len) return -1;
    
    int prefix_len;
    int* prefix = sigma_kmp_compute_prefix(pattern, pattern_len, &prefix_len);
    if (!prefix) return -1;
    
    int i = 0, j = 0;
    
    while (i < text_len) {
        if (text[i] == pattern[j]) {
            i++;
            j++;
            
            if (j == pattern_len) {
                free(prefix);
                return i - pattern_len; // Found at position i - pattern_len
            }
        } else {
            if (j != 0) {
                j = prefix[j - 1];
            } else {
                i++;
            }
        }
    }
    
    free(prefix);
    return -1; // Not found
}

// Cleanup functions
void sigma_circular_buffer_destroy(CircularBuffer* buffer) {
    if (buffer) {
        free(buffer->data);
        free(buffer);
    }
}

void sigma_binary_heap_destroy(BinaryHeap* heap) {
    if (heap) {
        free(heap->heap);
        free(heap);
    }
}

void sigma_bloom_filter_destroy(BloomFilter* filter) {
    if (filter) {
        free(filter->bit_array);
        free(filter);
    }
}

void sigma_skip_list_destroy(SkipList* list) {
    if (list) {
        SkipListNode* current = list->header;
        while (current) {
            SkipListNode* next = current->forward[0];
            free(current->forward);
            free(current);
            current = next;
        }
        free(list);
    }
}
