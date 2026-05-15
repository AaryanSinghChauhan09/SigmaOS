#include "../../../../../include/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN CONTROL ENGINE (v1.0)
 * =========================================================================
 * Mission: High-precision feedback control for Hardware and Robotics.
 * Principles: Proportional-Integral-Derivative (PID) Control.
 *
 * Implements a real PID controller for kernel thermal/power/IO regulation.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

typedef struct {
    sigma_f64 kp;
    sigma_f64 ki;
    sigma_f64 kd;
    sigma_f64 prev_error;
    sigma_f64 integral;
    sigma_f64 target;
} SigmaPID_t;

/**
 * sigma_pid_init: Initializes the controller with gains and target.
 */
void sigma_pid_init(SigmaPID_t* pid, sigma_f64 p, sigma_f64 i, sigma_f64 d, sigma_f64 target) {
    pid->kp = p;
    pid->ki = i;
    pid->kd = d;
    pid->target = target;
    pid->prev_error = 0;
    pid->integral = 0;
}

/**
 * sigma_pid_compute: Calculates the control output based on current error.
 */
sigma_f64 sigma_pid_compute(SigmaPID_t* pid, sigma_f64 current_value, sigma_f64 dt) {
    sigma_f64 error = pid->target - current_value;
    pid->integral += error * dt;
    sigma_f64 derivative = (error - pid->prev_error) / dt;
    
    sigma_f64 output = (pid->kp * error) + (pid->ki * pid->integral) + (pid->kd * derivative);
    
    pid->prev_error = error;
    return output;
}

/* --- Module Factory --- */

void SovereignControl_Register(void) {
    sigma_sigma_printf("[HAL]: Sovereign Control Engine (PID) active.\n");
}



