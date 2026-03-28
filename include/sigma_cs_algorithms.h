/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Computer Science & DSA Algorithms Implementation
 * ==========================================================
 * Complete implementation of fundamental CS algorithms:
 * - Sorting Algorithms
 * - Searching Algorithms
 * - Graph Algorithms
 * - Tree Algorithms
 * - Dynamic Programming
 * - Greedy Algorithms
 * - String Algorithms
 * - Mathematical Algorithms
 */

#ifndef SIGMA_CS_ALGORITHMS_H
#define SIGMA_CS_ALGORITHMS_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// ==================== DATA STRUCTURES ====================

// Array/Vector
typedef struct {
    int* data;
    uint32_t size;
    uint32_t capacity;
} SigmaArray;

// Linked List Node
typedef struct SigmaListNode {
    int data;
    struct SigmaListNode* next;
} SigmaListNode;

typedef struct {
    SigmaListNode* head;
    SigmaListNode* tail;
    uint32_t size;
} SigmaLinkedList;

// Stack
typedef struct {
    int* data;
    uint32_t top;
    uint32_t capacity;
} SigmaStack;

// Queue
typedef struct {
    int* data;
    uint32_t front;
    uint32_t rear;
    uint32_t size;
    uint32_t capacity;
} SigmaQueue;

// Deque
typedef struct {
    int* data;
    uint32_t front;
    uint32_t rear;
    uint32_t size;
    uint32_t capacity;
} SigmaDeque;

// Priority Queue (Min Heap)
typedef struct {
    int* data;
    uint32_t size;
    uint32_t capacity;
} SigmaPriorityQueue;

// Binary Tree Node
typedef struct SigmaTreeNode {
    int data;
    struct SigmaTreeNode* left;
    struct SigmaTreeNode* right;
    int height; // For AVL
    int bf; // Balance factor
} SigmaTreeNode;

// Binary Search Tree
typedef struct {
    SigmaTreeNode* root;
    uint32_t size;
} SigmaBST;

// AVL Tree (Self-balancing BST)
typedef SigmaBST SigmaAVLTree;

// Red-Black Tree Node
typedef struct SigmaRBNode {
    int data;
    bool is_red;
    struct SigmaRBNode* parent;
    struct SigmaRBNode* left;
    struct SigmaRBNode* right;
} SigmaRBNode;

typedef struct {
    SigmaRBNode* root;
    uint32_t size;
} SigmaRBTree;

// Trie Node
typedef struct SigmaTrieNode {
    struct SigmaTrieNode* children[26];
    bool is_end_of_word;
    char character;
} SigmaTrieNode;

typedef struct {
    SigmaTrieNode* root;
    uint32_t word_count;
} SigmaTrie;

// Graph (Adjacency List)
typedef struct SigmaGraphEdge {
    int dest;
    int weight;
    struct SigmaGraphEdge* next;
} SigmaGraphEdge;

typedef struct {
    int src;
    SigmaGraphEdge* edges;
} SigmaGraphVertex;

typedef struct {
    SigmaGraphVertex* vertices;
    uint32_t n_vertices;
    uint32_t n_edges;
    bool is_directed;
} SigmaGraph;

// Disjoint Set (Union-Find)
typedef struct {
    int* parent;
    int* rank;
    uint32_t size;
} SigmaDisjointSet;

// Hash Table
typedef struct SigmaHashEntry {
    char* key;
    int value;
    struct SigmaHashEntry* next;
} SigmaHashEntry;

typedef struct {
    SigmaHashEntry** buckets;
    uint32_t capacity;
    uint32_t size;
} SigmaHashTable;

// ==================== SORTING ALGORITHMS ====================

// Comparison-based sorting
void sigma_bubble_sort(int* arr, uint32_t n);
void sigma_selection_sort(int* arr, uint32_t n);
void sigma_insertion_sort(int* arr, uint32_t n);
void sigma_merge_sort(int* arr, uint32_t left, uint32_t right);
void sigma_quick_sort(int* arr, int low, int high);
void sigma_heap_sort(int* arr, uint32_t n);
void sigma_shell_sort(int* arr, uint32_t n);
void sigma_tim_sort(int* arr, uint32_t n);

// Non-comparison sorting
void sigma_counting_sort(int* arr, uint32_t n, int max_val);
void sigma_radix_sort(int* arr, uint32_t n);
void sigma_bucket_sort(int* arr, uint32_t n);
void sigma_pigeonhole_sort(int* arr, uint32_t n, int min, int max);

// Hybrid sorting
void sigma_intro_sort(int* arr, uint32_t n);
void sigma_dual_pivot_quick_sort(int* arr, int low, int high);
void sigma_block_sort(int* arr, uint32_t n);
void sigma_tournament_sort(int* arr, uint32_t n);

// Sorting utilities
void sigma_swap(int* a, int* b);
void sigma_merge(int* arr, uint32_t left, uint32_t mid, uint32_t right);
void sigma_heapify(int* arr, uint32_t n, uint32_t i);
int sigma_partition(int* arr, int low, int high);
int sigma_randomized_partition(int* arr, int low, int high);
int sigma_median_of_three_partition(int* arr, int low, int high);
bool sigma_is_sorted(int* arr, uint32_t n);

// ==================== SEARCHING ALGORITHMS ====================

// Basic searching
int sigma_linear_search(int* arr, uint32_t n, int target);
int sigma_binary_search(int* arr, uint32_t n, int target);
int sigma_jump_search(int* arr, uint32_t n, int target);
int sigma_interpolation_search(int* arr, uint32_t n, int target);
int sigma_exponential_search(int* arr, uint32_t n, int target);
int sigma_fibonacci_search(int* arr, uint32_t n, int target);
int sigma_ternary_search(int* arr, uint32_t left, uint32_t right, int target);

// Advanced searching
int sigma_brents_search(int* arr, uint32_t n, int target);
int sigma_sentinel_linear_search(int* arr, uint32_t n, int target);
int sigma_meta_binary_search(int* arr, uint32_t n, int target);
int sigma_ubiquitous_binary_search(int* arr, uint32_t n, int target);

// Finding algorithms
int sigma_find_min(int* arr, uint32_t n);
int sigma_find_max(int* arr, uint32_t n);
int* sigma_find_kth_smallest(int* arr, uint32_t n, uint32_t k);
int sigma_find_median(int* arr, uint32_t n);
int* sigma_find_duplicates(int* arr, uint32_t n, uint32_t* count);
int* sigma_find_missing_number(int* arr, uint32_t n, uint32_t expected_n, uint32_t* count);
int* sigma_find_two_sum(int* arr, uint32_t n, int target, int* indices);
int* sigma_find_three_sum(int* arr, uint32_t n, int target, int** indices, uint32_t* count);
int* sigma_find_majority_element(int* arr, uint32_t n);

// String searching
int sigma_naive_string_search(const char* text, const char* pattern);
int sigma_kmp_search(const char* text, const char* pattern);
int sigma_rabin_karp_search(const char* text, const char* pattern, int prime);
int sigma_boyer_moore_search(const char* text, const char* pattern);
int sigma_z_algorithm_search(const char* text, const char* pattern);

// ==================== GRAPH ALGORITHMS ====================

// Graph creation and manipulation
SigmaGraph* sigma_graph_create(uint32_t n_vertices, bool is_directed);
void sigma_graph_add_edge(SigmaGraph* graph, int src, int dest, int weight);
void sigma_graph_remove_edge(SigmaGraph* graph, int src, int dest);
void sigma_graph_destroy(SigmaGraph* graph);
void sigma_graph_print(SigmaGraph* graph);

// Graph traversal
void sigma_dfs(SigmaGraph* graph, int start_vertex, bool* visited, int* result, uint32_t* count);
void sigma_bfs(SigmaGraph* graph, int start_vertex, int* result, uint32_t* count);
void sigma_dfs_recursive(SigmaGraph* graph, int vertex, bool* visited, void (*visit)(int));

// Shortest path algorithms
int* sigma_dijkstra(SigmaGraph* graph, int src, int* dist);
int* sigma_bellman_ford(SigmaGraph* graph, int src, int* dist, bool* negative_cycle);
int** sigma_floyd_warshall(SigmaGraph* graph);
int* sigma_a_star_search(SigmaGraph* graph, int src, int dest, int* h); // h = heuristic

// Minimum spanning tree
SigmaGraph* sigma_prim_mst(SigmaGraph* graph);
SigmaGraph* sigma_kruskal_mst(SigmaGraph* graph, SigmaDisjointSet* ds);

// Topological sorting
int* sigma_topological_sort_dfs(SigmaGraph* graph, uint32_t* count);
int* sigma_topological_sort_kahn(SigmaGraph* graph, uint32_t* count);

// Cycle detection
bool sigma_has_cycle_undirected(SigmaGraph* graph);
bool sigma_has_cycle_directed(SigmaGraph* graph);

// Strongly connected components
void sigma_tarjan_scc(SigmaGraph* graph, int** sccs, uint32_t* n_sccs);
void sigma_kosaraju_scc(SigmaGraph* graph, int** sccs, uint32_t* n_sccs);

// Maximum flow
int sigma_edmonds_karp(SigmaGraph* graph, int src, int sink);
int sigma_dinic_max_flow(SigmaGraph* graph, int src, int sink);
int sigma_push_relabel_max_flow(SigmaGraph* graph, int src, int sink);

// Minimum cut
int** sigma_stoer_wagner_min_cut(SigmaGraph* graph, uint32_t* n_components);

// Bipartite checking
bool sigma_is_bipartite(SigmaGraph* graph);
int* sigma_bipartite_coloring(SigmaGraph* graph);

// Hamiltonian and Eulerian paths
bool sigma_hamiltonian_path_exists(SigmaGraph* graph);
bool sigma_eulerian_path_exists(SigmaGraph* graph);
int* sigma_find_eulerian_path(SigmaGraph* graph, uint32_t* path_length);

// ==================== TREE ALGORITHMS ====================

// Binary Tree operations
SigmaTreeNode* sigma_tree_create_node(int data);
void sigma_tree_insert(SigmaTreeNode** root, int data);
void sigma_tree_delete(SigmaTreeNode** root, int data);
SigmaTreeNode* sigma_tree_search(SigmaTreeNode* root, int data);
int sigma_tree_height(SigmaTreeNode* root);
int sigma_tree_depth(SigmaTreeNode* root, int data);
int sigma_tree_size(SigmaTreeNode* root);
bool sigma_tree_is_balanced(SigmaTreeNode* root);
bool sigma_tree_is_full(SigmaTreeNode* root);
bool sigma_tree_is_complete(SigmaTreeNode* root);
bool sigma_tree_is_perfect(SigmaTreeNode* root);

// Tree traversals
void sigma_inorder_traversal(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_preorder_traversal(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_postorder_traversal(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_level_order_traversal(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_spiral_order_traversal(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_boundary_traversal(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_diagonal_traversal(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_vertical_order_traversal(SigmaTreeNode* root, int** result, uint32_t* n_lines);

// Tree views
void sigma_left_view(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_right_view(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_top_view(SigmaTreeNode* root, int* result, uint32_t* count);
void sigma_bottom_view(SigmaTreeNode* root, int* result, uint32_t* count);

// BST operations
SigmaBST* sigma_bst_create(void);
void sigma_bst_insert(SigmaBST* tree, int data);
void sigma_bst_delete(SigmaBST* tree, int data);
bool sigma_bst_search(SigmaBST* tree, int data);
int sigma_bst_min(SigmaBST* tree);
int sigma_bst_max(SigmaBST* tree);
int sigma_bst_kth_smallest(SigmaBST* tree, uint32_t k);
int sigma_bst_kth_largest(SigmaBST* tree, uint32_t k);
bool sigma_bst_is_valid(SigmaBST* tree);
void sigma_bst_destroy(SigmaBST* tree);

// AVL Tree operations
SigmaAVLTree* sigma_avl_create(void);
void sigma_avl_insert(SigmaAVLTree* tree, int data);
void sigma_avl_delete(SigmaAVLTree* tree, int data);
SigmaTreeNode* sigma_avl_left_rotate(SigmaTreeNode* y);
SigmaTreeNode* sigma_avl_right_rotate(SigmaTreeNode* x);
int sigma_avl_get_balance(SigmaTreeNode* node);
int sigma_avl_get_height(SigmaTreeNode* node);
void sigma_avl_destroy(SigmaAVLTree* tree);

// Red-Black Tree operations
SigmaRBTree* sigma_rbtree_create(void);
void sigma_rbtree_insert(SigmaRBTree* tree, int data);
void sigma_rbtree_delete(SigmaRBTree* tree, int data);
bool sigma_rbtree_search(SigmaRBTree* tree, int data);
void sigma_rbtree_destroy(SigmaRBTree* tree);

// Trie operations
SigmaTrie* sigma_trie_create(void);
void sigma_trie_insert(SigmaTrie* trie, const char* word);
bool sigma_trie_search(SigmaTrie* trie, const char* word);
bool sigma_trie_starts_with(SigmaTrie* trie, const char* prefix);
void sigma_trie_delete(SigmaTrie* trie, const char* word);
char** sigma_trie_autocomplete(SigmaTrie* trie, const char* prefix, uint32_t* count);
void sigma_trie_destroy(SigmaTrie* trie);

// Segment Tree
typedef struct {
    int* tree;
    int* lazy;
    uint32_t n;
} SigmaSegmentTree;

SigmaSegmentTree* sigma_segment_tree_create(int* arr, uint32_t n);
int sigma_segment_tree_query(SigmaSegmentTree* st, uint32_t l, uint32_t r);
void sigma_segment_tree_update(SigmaSegmentTree* st, uint32_t idx, int val);
void sigma_segment_tree_range_update(SigmaSegmentTree* st, uint32_t l, uint32_t r, int val);
void sigma_segment_tree_destroy(SigmaSegmentTree* st);

// Fenwick Tree (Binary Indexed Tree)
typedef struct {
    int* tree;
    uint32_t n;
} SigmaFenwickTree;

SigmaFenwickTree* sigma_fenwick_tree_create(uint32_t n);
void sigma_fenwick_tree_update(SigmaFenwickTree* ft, uint32_t idx, int val);
int sigma_fenwick_tree_query(SigmaFenwickTree* ft, uint32_t idx);
int sigma_fenwick_tree_range_query(SigmaFenwickTree* ft, uint32_t l, uint32_t r);
void sigma_fenwick_tree_destroy(SigmaFenwickTree* ft);

// ==================== DYNAMIC PROGRAMMING ====================

// Classic DP problems
int sigma_fibonacci_dp(int n);
int sigma_factorial_dp(int n);
int sigma_climbing_stairs(int n);
int sigma_house_robber(int* nums, uint32_t n);
int sigma_coin_change(int* coins, uint32_t n, int amount);
int sigma_min_coin_change(int* coins, uint32_t n, int amount);
bool sigma_can_partition(int* nums, uint32_t n);
int sigma_knapsack_01(int* weights, int* values, uint32_t n, int capacity);
int sigma_unbounded_knapsack(int* weights, int* values, uint32_t n, int capacity);
int* sigma_longest_increasing_subsequence(int* arr, uint32_t n, uint32_t* lis_length);
int* sigma_longest_common_subsequence(const char* s1, const char* s2, uint32_t* lcs_length);
int sigma_edit_distance(const char* s1, const char* s2);
int sigma_min_distance_word(const char* word1, const char* word2);
int sigma_max_subarray_sum(int* arr, uint32_t n); // Kadane's algorithm
int sigma_max_product_subarray(int* arr, uint32_t n);
bool sigma_regex_matching(const char* s, const char* p);
bool sigma_wildcard_matching(const char* s, const char* p);
int sigma_palindrome_partitioning(const char* s);
int* sigma_matrix_chain_mult(int* dims, uint32_t n);
int sigma_burst_balloons(int* nums, uint32_t n);
int sigma_longest_palindromic_subseq(const char* s);

// Grid-based DP
int sigma_unique_paths(uint32_t m, uint32_t n);
int sigma_unique_paths_with_obstacles(int** grid, uint32_t m, uint32_t n);
int sigma_min_path_sum(int** grid, uint32_t m, uint32_t n);
int sigma_max_path_sum(int** grid, uint32_t m, uint32_t n);
int sigma_triangle_minimum_path(int** triangle, uint32_t n);
int sigma_dungeon_game(int** dungeon, uint32_t m, uint32_t n);

// String DP
int sigma_longest_common_substring(const char* s1, const char* s2);
int sigma_shortest_common_supersequence(const char* s1, const char* s2);
int sigma_longest_repeated_subsequence(const char* s);
int sigma_count_palindromic_substrings(const char* s);
int sigma_distinct_subsequences(const char* s, const char* t);
int sigma_interleaving_strings(const char* s1, const char* s2, const char* s3);

// State-based DP
int sigma_stone_game(int* piles, uint32_t n);
int sigma_predict_winner(int* nums, uint32_t n);
bool sigma_can_i_win(int max_choosable, int desired_total);

// Bitmask DP
int sigma_traveling_salesman(int** graph, uint32_t n);
int sigma_assign_workers(int** cost, uint32_t n);

// ==================== GREEDY ALGORITHMS ====================

// Activity selection
void sigma_activity_selection(int* start, int* finish, uint32_t n, 
                              int** selected, uint32_t* count);

// Interval scheduling
void sigma_interval_scheduling(int* start, int* finish, uint32_t n,
                               int** selected, uint32_t* count);

// Fractional knapsack
typedef struct {
    int weight;
    int value;
    double ratio;
} SigmaItem;

int sigma_fractional_knapsack(SigmaItem* items, uint32_t n, int capacity);

// Huffman coding
typedef struct SigmaHuffmanNode {
    char data;
    int freq;
    struct SigmaHuffmanNode* left;
    struct SigmaHuffmanNode* right;
} SigmaHuffmanNode;

SigmaHuffmanNode* sigma_huffman_build_tree(char* data, int* freq, uint32_t n);
void sigma_huffman_generate_codes(SigmaHuffmanNode* root, char* code, 
                                  char** codes, int depth);

// Job sequencing
typedef struct {
    int id;
    int deadline;
    int profit;
} SigmaJob;

int* sigma_job_sequencing(SigmaJob* jobs, uint32_t n, uint32_t* count);

// Minimum coins
typedef struct {
    int coin;
    int count;
} SigmaCoin;

SigmaCoin* sigma_min_coins_greedy(int* coins, uint32_t n, int amount, uint32_t* coin_count);

// Gas station
int sigma_gas_station_circuit(int* gas, int* cost, uint32_t n);

// Jump game
bool sigma_jump_game(int* nums, uint32_t n);
int sigma_jump_game_min_jumps(int* nums, uint32_t n);

// Candy distribution
int sigma_candy_distribution(int* ratings, uint32_t n);

// Task scheduler
int sigma_task_scheduler_least_interval(char* tasks, uint32_t n, int cooling_time);

// Queue reconstruction
int** sigma_queue_reconstruction(int** people, uint32_t n);

// ==================== MATHEMATICAL ALGORITHMS ====================

// Number theory
bool sigma_is_prime(int n);
void sigma_sieve_of_eratosthenes(int n, bool* is_prime);
int sigma_gcd(int a, int b); // Euclidean algorithm
int sigma_lcm(int a, int b);
int sigma_extended_gcd(int a, int b, int* x, int* y);
int sigma_modular_exponentiation(int base, int exp, int mod);
int sigma_modular_inverse(int a, int m);
void sigma_prime_factors(int n, int** factors, uint32_t* count);
int sigma_euler_totient(int n);
int sigma_carmichael_function(int n);

// Modular arithmetic
int sigma_modular_add(int a, int b, int mod);
int sigma_modular_sub(int a, int b, int mod);
int sigma_modular_mul(int a, int b, int mod);
int sigma_modular_div(int a, int b, int mod);

// Matrix operations
void sigma_matrix_multiply_int(int** A, int** B, int** C, uint32_t n);
void sigma_matrix_power(int** A, uint32_t n, int power, int mod);
int sigma_matrix_determinant_int(int** A, uint32_t n);
void sigma_matrix_transpose_int(int** A, int** T, uint32_t n, uint32_t m);
int** sigma_matrix_inverse_int(int** A, uint32_t n);

// Fast algorithms
int sigma_fast_exponentiation(int base, int exp);
int sigma_fast_multiply(int a, int b);
int sigma_karatsuba_multiply(int a, int b);

// Combinatorics
int sigma_factorial(int n);
int sigma_nCr(int n, int r);
int sigma_nPr(int n, int r);
int sigma_catalan_number(int n);
int sigma_binomial_coefficient(int n, int k);
void sigma_pascal_triangle(int n, int** triangle);

// Bit manipulation
int sigma_count_set_bits(int n);
int sigma_parity(int n);
int sigma_reverse_bits(int n);
int sigma_highest_set_bit(int n);
int sigma_lowest_set_bit(int n);
int sigma_is_power_of_two(int n);
int sigma_next_power_of_two(int n);
int sigma_count_trailing_zeros(int n);
int sigma_count_leading_zeros(int n);
int sigma_swap_without_temp(int a, int b);
int sigma_find_single_number(int* arr, uint32_t n); // XOR all elements
void sigma_find_two_non_repeating(int* arr, uint32_t n, int* num1, int* num2);

// Geometry
typedef struct {
    int x;
    int y;
} SigmaPoint;

double sigma_distance(SigmaPoint p1, SigmaPoint p2);
double sigma_polygon_area(SigmaPoint* points, uint32_t n);
bool sigma_point_in_polygon(SigmaPoint p, SigmaPoint* polygon, uint32_t n);
SigmaPoint* sigma_convex_hull(SigmaPoint* points, uint32_t n, uint32_t* hull_size);
bool sigma_line_intersection(SigmaPoint p1, SigmaPoint p2, SigmaPoint p3, 
                              SigmaPoint p4, SigmaPoint* intersection);

// ==================== STRING ALGORITHMS ====================

// Pattern matching
int* sigma_kmp_compute_lps(const char* pattern, uint32_t m);
void sigma_kmp_search_all(const char* text, const char* pattern, 
                          int** positions, uint32_t* count);

int sigma_rabin_karp(const char* text, const char* pattern);
void sigma_rabin_karp_multiple(const char* text, char** patterns, 
                               uint32_t n_patterns, int** positions);

// Suffix arrays and trees
int* sigma_build_suffix_array(const char* text, uint32_t n);
int* sigma_build_lcp_array(const char* text, int* suffix_array, uint32_t n);
int sigma_count_distinct_substrings(const char* text);
char* sigma_longest_common_substring_str(const char* s1, const char* s2);

// String manipulation
void sigma_reverse_string(char* str);
bool sigma_is_palindrome(const char* str);
char* sigma_longest_palindromic_substring(const char* str);
char* sigma_manacher_algorithm(const char* str);
char* sigma_reverse_words(const char* str);
char* sigma_z_algorithm(const char* str, int* z, uint32_t n);

// Edit operations
int sigma_levenshtein_distance(const char* s1, const char* s2);
int sigma_hamming_distance(const char* s1, const char* s2);
int sigma_damerau_levenshtein(const char* s1, const char* s2);

// ==================== DISJOINT SET ====================

SigmaDisjointSet* sigma_disjoint_set_create(uint32_t n);
int sigma_disjoint_set_find(SigmaDisjointSet* ds, int i);
void sigma_disjoint_set_union(SigmaDisjointSet* ds, int x, int y);
bool sigma_disjoint_set_connected(SigmaDisjointSet* ds, int x, int y);
void sigma_disjoint_set_destroy(SigmaDisjointSet* ds);

// ==================== HASH TABLE ====================

SigmaHashTable* sigma_hash_table_create(uint32_t capacity);
void sigma_hash_table_insert(SigmaHashTable* ht, const char* key, int value);
int sigma_hash_table_get(SigmaHashTable* ht, const char* key, bool* found);
void sigma_hash_table_remove(SigmaHashTable* ht, const char* key);
bool sigma_hash_table_contains(SigmaHashTable* ht, const char* key);
void sigma_hash_table_destroy(SigmaHashTable* ht);

// ==================== STACK & QUEUE ====================

SigmaStack* sigma_stack_create(uint32_t capacity);
void sigma_stack_push(SigmaStack* stack, int val);
int sigma_stack_pop(SigmaStack* stack, bool* success);
int sigma_stack_peek(SigmaStack* stack, bool* success);
bool sigma_stack_is_empty(SigmaStack* stack);
bool sigma_stack_is_full(SigmaStack* stack);
void sigma_stack_destroy(SigmaStack* stack);

SigmaQueue* sigma_queue_create(uint32_t capacity);
void sigma_queue_enqueue(SigmaQueue* queue, int val);
int sigma_queue_dequeue(SigmaQueue* queue, bool* success);
int sigma_queue_front(SigmaQueue* queue, bool* success);
int sigma_queue_rear(SigmaQueue* queue, bool* success);
bool sigma_queue_is_empty(SigmaQueue* queue);
bool sigma_queue_is_full(SigmaQueue* queue);
void sigma_queue_destroy(SigmaQueue* queue);

// ==================== UTILITY FUNCTIONS ====================

void sigma_print_array(int* arr, uint32_t n);
void sigma_print_matrix(int** mat, uint32_t rows, uint32_t cols);
void sigma_swap_int(int* a, int* b);
int sigma_max(int a, int b);
int sigma_min(int a, int b);
int sigma_abs(int a);
int sigma_random_int(int min, int max);
void sigma_shuffle_array(int* arr, uint32_t n);
void sigma_copy_array(int* src, int* dest, uint32_t n);
bool sigma_arrays_equal(int* arr1, int* arr2, uint32_t n);
void sigma_reverse_array(int* arr, uint32_t n);
int* sigma_array_slice(int* arr, uint32_t start, uint32_t end);

#endif // SIGMA_CS_ALGORITHMS_H

