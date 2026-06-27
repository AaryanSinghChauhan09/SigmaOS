// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_transport.h — Road transport (Motor Vehicles Act 2019, e-Way Bill)
 * 5 million+ trucks, world's largest road network
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_PERMIT_NATIONAL_GOODS    = 1,
    SIGMA_PERMIT_CONTRACT_CARRIAGE = 2,
    SIGMA_PERMIT_STAGE_CARRIAGE    = 3,
    SIGMA_PERMIT_TOURIST           = 4,
} sigma_permit_type_t;

typedef struct {
    char   vehicle_reg[16];
    char   owner_name[128];
    char   owner_aadhaar[12];
    sigma_permit_type_t permit_type;
    char   permit_no[32];
    sigma_u64 permit_expiry_epoch;
    sigma_u64 fitness_expiry_epoch;
    sigma_u64 insurance_expiry_epoch;
    sigma_u64 puc_expiry_epoch;      /* Pollution Under Control            */
    bool   fasttag_active;
    char   fasttag_id[32];
    sigma_s64 fasttag_balance_paise;
} sigma_vehicle_permit_t;

typedef struct {
    char   driver_id[16];
    char   name[128];
    char   dl_no[20];
    sigma_u64 dl_expiry_epoch;
    double hours_driven_today;
    double hours_rest_today;
    double total_km_today;
    bool   compliant;             /* MORTH: max 10h driving, 8h rest      */
} sigma_driver_log_t;

typedef struct {
    char   eway_bill_no[13];
    char   gstin_supplier[16];
    char   gstin_recipient[16];
    char   vehicle_no[16];
    char   from_pin[7];
    char   to_pin[7];
    double distance_km;
    sigma_s64 invoice_value_paise;
    sigma_u64 generated_epoch;
    sigma_u64 valid_until_epoch;
    bool   extended;
} sigma_eway_bill_t;

int sigma_transport_vehicle_verify(const char *reg_no, sigma_vehicle_permit_t *out);
int sigma_transport_driver_log(const sigma_driver_log_t *log);
int sigma_transport_eway_generate(const sigma_eway_bill_t *ewb);
int sigma_transport_eway_extend(const char *eway_bill_no, const char *reason,
                                 sigma_u32 extend_hours);
int sigma_transport_fasttag_balance(const char *vehicle_reg,
                                     sigma_s64 *balance_paise_out);
