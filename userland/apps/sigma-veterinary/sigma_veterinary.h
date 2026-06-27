// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_veterinary.h — Veterinary professionals
 * PCA Act 1960, VCI regulations, INAPH integration, PMFBY livestock
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_SPECIES_BOVINE    = 1,  /* Cow, Buffalo                        */
    SIGMA_SPECIES_EQUINE    = 2,  /* Horse, Donkey                       */
    SIGMA_SPECIES_CANINE    = 3,  /* Dog                                 */
    SIGMA_SPECIES_FELINE    = 4,  /* Cat                                 */
    SIGMA_SPECIES_PORCINE   = 5,  /* Pig                                 */
    SIGMA_SPECIES_POULTRY   = 6,  /* Chicken, Duck                       */
    SIGMA_SPECIES_OVINE     = 7,  /* Sheep, Goat                         */
    SIGMA_SPECIES_WILDLIFE  = 99,
} sigma_species_t;

/* ── Animal patient record ───────────────────────────────────────────────── */
typedef struct {
    sigma_u32       id;
    char            uid_tag[20];      /* Govt cattle tag: "IN-MH-123456"    */
    sigma_species_t species;
    char            breed[64];
    char            name[64];
    char            owner_name[128];
    char            owner_phone[16];
    sigma_u64       dob_epoch;
    double          weight_kg;
    char            sex[8];           /* "male", "female", "castrated"      */
    bool            vaccinated_rabies;
    sigma_u64       rabies_due_epoch;
} sigma_animal_patient_t;

/* ── Drug dosage calculator ─────────────────────────────────────────────── */
typedef struct {
    char   drug_name[64];     /* "Ivermectin", "Amoxicillin", "Enrofloxacin" */
    sigma_species_t species;
    double body_weight_kg;
    double dose_mg_per_kg;   /* species-specific from VCI schedule           */
    char   route[16];        /* "oral", "IM", "IV", "SC"                    */
    /* Output */
    double total_dose_mg;
    double volume_ml;        /* if concentration known                       */
    double concentration_mg_per_ml;
    char   frequency[32];    /* "once daily", "BID", "TID"                  */
    int    duration_days;
    bool   scheduled;        /* Schedule H/H1 — requires prescription       */
} sigma_vet_dose_t;

/* ── Vaccination record ──────────────────────────────────────────────────── */
typedef struct {
    sigma_u32  animal_id;
    char       vaccine_name[64];  /* "Rabies", "FMD", "HPAI H5N1", "LSD"   */
    char       batch_no[32];
    sigma_u64  given_epoch;
    sigma_u64  due_epoch;
    char       given_by[128];     /* vet name + registration number          */
    char       certificate_no[32];/* for travel/export certificates          */
} sigma_vaccination_t;

/* ── Milk production record (dairy) ─────────────────────────────────────── */
typedef struct {
    sigma_u32  animal_id;
    sigma_u64  date_epoch;
    double     morning_litres;
    double     evening_litres;
    double     fat_pct;
    double     snf_pct;          /* Solid Not Fat                           */
    sigma_s64  price_per_litre_paise;
} sigma_milk_record_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_vet_patient_create(const sigma_animal_patient_t *patient);
int sigma_vet_drug_dose(sigma_vet_dose_t *dose);
int sigma_vet_vaccine_record(const sigma_vaccination_t *vacc);
int sigma_vet_milk_record(const sigma_milk_record_t *rec);
int sigma_vet_disease_alert(sigma_species_t species, const char *disease,
                              const char *district, char *advisory_out, size_t max);
int sigma_vet_inaph_sync(sigma_u32 animal_id);
int sigma_vet_rabies_cert(sigma_u32 animal_id, char *cert_json_out, size_t max);
