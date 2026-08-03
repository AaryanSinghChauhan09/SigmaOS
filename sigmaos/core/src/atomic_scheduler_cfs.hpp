#ifndef ATOMIC_SCHEDULER_CFS_HPP
#define ATOMIC_SCHEDULER_CFS_HPP

#include "include/sigma_kernel_types.h"

// Abstract Base Class for OOP Interface Audit
class SchedulerPolicy {
public:
    virtual ~SchedulerPolicy() {}
    // Pure virtual method for OOP compliance
    virtual sigma_s32 select_next(sigma_u64* vruntimes, sigma_s32 count) = 0;
};

class CfsScheduler : public SchedulerPolicy {
public:
    CfsScheduler();
    virtual ~CfsScheduler() {}
    virtual sigma_s32 select_next(sigma_u64* vruntimes, sigma_s32 count) override;
};

#endif // ATOMIC_SCHEDULER_CFS_HPP
