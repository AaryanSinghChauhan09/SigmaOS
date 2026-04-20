/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN GRAPH ENGINE (v1.0)
 * =========================================================================
 * Mission: High-performance Routing and Graph Theory in the Kernel.
 * Principles: Dijkstra's Algorithm, Adjacency List, Path Optimization.
 *
 * Implements a real shortest-path engine for network and resource routing.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

#define MAX_NODES 16
#define INF 9999

typedef struct {
    int weights[MAX_NODES][MAX_NODES];
    int size;
} SigmaGraph_t;

/**
 * sigma_graph_dijkstra: Finds the shortest path from start_node.
 */
void sigma_graph_dijkstra(SigmaGraph_t* g, int start_node, int* dist) {
    int visited[MAX_NODES] = {0};
    
    for (int i = 0; i < g->size; i++) dist[i] = g->weights[start_node][i];
    visited[start_node] = 1;
    dist[start_node] = 0;
    
    for (int i = 0; i < g->size - 1; i++) {
        int min_dist = INF;
        int u = -1;
        
        for (int v = 0; v < g->size; v++) {
            if (!visited[v] && dist[v] <= min_dist) {
                min_dist = dist[v];
                u = v;
            }
        }
        
        if (u == -1) break;
        visited[u] = 1;
        
        for (int v = 0; v < g->size; v++) {
            if (!visited[v] && g->weights[u][v] != INF && dist[u] != INF &&
                dist[u] + g->weights[u][v] < dist[v]) {
                dist[v] = dist[u] + g->weights[u][v];
            }
        }
    }
}

/* --- Module Factory --- */

void SovereignGraph_Register(void) {
    sigma_sigma_sigma_printf("[ALGO]: Sovereign Graph Engine (Dijkstra) seeded.\n");
}



