/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN FREERTOS PARITY (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: FreeRTOS
 *   https://github.com/FreeRTOS/FreeRTOS-Kernel
 *
 * Features implemented:
 *   ✓ Priority-preemptive task scheduler simulation
 *   ✓ xTaskCreate / vTaskStartScheduler
 *   ✓ Inter-task Queues (xQueueCreate, xQueueSend, xQueueReceive)
 *   ✓ Tick mechanism
 * =========================================================================
 */

#ifndef SOVEREIGN_FREERTOS_H
#define SOVEREIGN_FREERTOS_H

#include "sigma_types.h"

typedef void (*SigmaTaskFunction_t)(void *pvParameters);
typedef void *SigmaTaskHandle_t;
typedef void *SigmaQueueHandle_t;

typedef struct {
    SigmaTaskFunction_t pxTaskCode;
    const char *pcName;
    sigma_u32 usStackDepth;
    void *pvParameters;
    sigma_u32 uxPriority;
    SigmaTaskHandle_t *pxCreatedTask;
    sigma_bool active;
} SigmaFreeRTOSTask_t;

/* API */
sigma_err_t sigma_xTaskCreate(SigmaTaskFunction_t pxTaskCode,
                              const char * const pcName,
                              const sigma_u32 usStackDepth,
                              void * const pvParameters,
                              sigma_u32 uxPriority,
                              SigmaTaskHandle_t * const pxCreatedTask);

sigma_err_t sigma_vTaskStartScheduler(void);

SigmaQueueHandle_t sigma_xQueueCreate(sigma_u32 uxQueueLength, sigma_u32 uxItemSize);
sigma_err_t sigma_xQueueSend(SigmaQueueHandle_t xQueue, const void * pvItemToQueue, sigma_u32 xTicksToWait);
sigma_err_t sigma_xQueueReceive(SigmaQueueHandle_t xQueue, void * const pvBuffer, sigma_u32 xTicksToWait);

void SovereignFreeRTOS_Init(void);

#endif /* SOVEREIGN_FREERTOS_H */
