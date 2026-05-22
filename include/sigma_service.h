#ifndef SIGMA_SERVICE_H
#define SIGMA_SERVICE_H

#define MAX_SERVICE_DEPS 4

typedef enum {
    SERVICE_STATE_INACTIVE,
    SERVICE_STATE_STARTING,
    SERVICE_STATE_ACTIVE,
    SERVICE_STATE_FAILED
} sigma_service_state_t;

typedef struct sigma_service {
    char name[32];
    sigma_service_state_t state;
    
    // Dependencies (who must be active before we start)
    char requires[MAX_SERVICE_DEPS][32];
    int num_requires;
    
    // Function to start the service
    int (*start_func)(void);
} sigma_service_t;

// Register a service with its dependencies
void sigma_service_register(const char* name, int (*start_func)(void), const char** requires, int num_deps);

// Start all services respecting the dependency graph
void sigma_service_start_all(void);

#endif // SIGMA_SERVICE_H
