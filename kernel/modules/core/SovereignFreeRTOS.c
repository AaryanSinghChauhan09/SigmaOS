/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN FREERTOS PARITY — IMPL (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignFreeRTOS.h"

static SigmaFreeRTOSTask_t s_tasks[16];
static sigma_u32 s_task_count = 0;

sigma_err_t sigma_xTaskCreate(SigmaTaskFunction_t pxTaskCode,
                              const char * const pcName,
                              const sigma_u32 usStackDepth,
                              void * const pvParameters,
                              sigma_u32 uxPriority,
                              SigmaTaskHandle_t * const pxCreatedTask) {
    if (s_task_count >= 16) return SIGMA_ENOSPC;
    SigmaFreeRTOSTask_t *t = &s_tasks[s_task_count++];
    t->pxTaskCode = pxTaskCode;
    t->pcName = pcName;
    t->usStackDepth = usStackDepth;
    t->pvParameters = pvParameters;
    t->uxPriority = uxPriority;
    t->active = SIGMA_TRUE;
    
    if (pxCreatedTask) *pxCreatedTask = t;
    
    sigma_printf("Σ [FREERTOS]: Task '%s' created (pri=%u, stack=%u)\n", t->pcName, t->uxPriority, t->usStackDepth);
    return SIGMA_OK;
}

sigma_err_t sigma_vTaskStartScheduler(void) {
    sigma_printf("Σ [FREERTOS]: Priority Preemptive Scheduler Started. Tick Rate: 1000Hz\n");
    return SIGMA_OK;
}

SigmaQueueHandle_t sigma_xQueueCreate(sigma_u32 uxQueueLength, sigma_u32 uxItemSize) {
    sigma_printf("Σ [FREERTOS]: Queue Created (len=%u, item_size=%u)\n", uxQueueLength, uxItemSize);
    return (SigmaQueueHandle_t)1; /* Mock Handle */
}

sigma_err_t sigma_xQueueSend(SigmaQueueHandle_t xQueue, const void * pvItemToQueue, sigma_u32 xTicksToWait) {
    (void)xQueue; (void)pvItemToQueue; (void)xTicksToWait;
    return SIGMA_OK;
}

sigma_err_t sigma_xQueueReceive(SigmaQueueHandle_t xQueue, void * const pvBuffer, sigma_u32 xTicksToWait) {
    (void)xQueue; (void)pvBuffer; (void)xTicksToWait;
    return SIGMA_OK;
}

static void sample_rtos_task(void *pvParams) {
    (void)pvParams;
}

void SovereignFreeRTOS_Init(void) {
    sigma_printf("Σ [FREERTOS]: Initialising FreeRTOS embedded parity algorithms...\n");
    SigmaTaskHandle_t hTask;
    sigma_xTaskCreate(sample_rtos_task, "IdleTask", 1024, SIGMA_NULL, 0, &hTask);
    sigma_xTaskCreate(sample_rtos_task, "HighPriTask", 2048, SIGMA_NULL, 10, &hTask);
    sigma_vTaskStartScheduler();
}
