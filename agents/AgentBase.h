#ifndef AGENT_BASE_H
#define AGENT_BASE_H

#include "include/sigma_types.h"

/**
 * Base class for all Autonomous Agents
 * Provides an abstract interface for execution.
 */
class AgentBase {
public:
    virtual ~AgentBase() {}
    virtual void start() = 0;
    virtual void stop() = 0;
    virtual void executeTask(const char* task_name) = 0;
};

#endif
