// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
#include <sigma_kernel_types.h>

void     sigma_sched_rt_init(void);
int      sigma_sched_rt_admit(sigma_u32 pid, int priority,
                               sigma_u64 period_ns, sigma_u64 deadline_ns);
sigma_u32 sigma_sched_rt_pick_next(sigma_u64 now_ns);
void     sigma_sched_rt_boost_priority(sigma_u32 holder_pid, sigma_u32 waiter_pid);
