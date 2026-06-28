// SPDX-License-Identifier: GPL-2.0-only
// sigma_digital_twin.h — SigmaOS Digital Twin Platform
// Purpose: Real-time virtual replica of physical systems.
//          Factory floor, hospital, farm, building — mirrored in SigmaOS.
//          Simulate failures before they happen. Predictive maintenance.
//          IoT sensors → live twin. sigma-ai → predictions.

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

// ---------------------------------------------------------------------------
// Twin Types
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_TWIN_FACTORY      = 1,   // Manufacturing plant / workshop
    SIGMA_TWIN_HOSPITAL     = 2,   // Hospital / clinic
    SIGMA_TWIN_FARM         = 3,   // Agricultural field
    SIGMA_TWIN_BUILDING     = 4,   // Commercial / residential building
    SIGMA_TWIN_POWER_GRID   = 5,   // Electrical distribution network
    SIGMA_TWIN_WATER_INFRA  = 6,   // Water supply / sewage system
    SIGMA_TWIN_TRANSPORT    = 7,   // Road / fleet / logistics network
    SIGMA_TWIN_CUSTOM       = 99,
} sigma_twin_type_t;

typedef enum {
    SIGMA_TWIN_STATE_NORMAL     = 0,
    SIGMA_TWIN_STATE_WARNING    = 1,  // Anomaly detected, prediction triggered
    SIGMA_TWIN_STATE_CRITICAL   = 2,  // Imminent failure predicted
    SIGMA_TWIN_STATE_FAILED     = 3,  // Asset has failed (physical confirmation)
    SIGMA_TWIN_STATE_SIMULATING = 4,  // Running simulation, not live
} sigma_twin_state_t;

// ---------------------------------------------------------------------------
// IoT Sensor
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_TWIN_SENSOR_TEMPERATURE   = 1,
    SIGMA_TWIN_SENSOR_VIBRATION     = 2,
    SIGMA_TWIN_SENSOR_PRESSURE      = 3,
    SIGMA_TWIN_SENSOR_CURRENT       = 4,
    SIGMA_TWIN_SENSOR_FLOW          = 5,
    SIGMA_TWIN_SENSOR_HUMIDITY      = 6,
    SIGMA_TWIN_SENSOR_OCCUPANCY     = 7,  // PIR / radar
    SIGMA_TWIN_SENSOR_SOIL_MOISTURE = 8,
    SIGMA_TWIN_SENSOR_CO2           = 9,
    SIGMA_TWIN_SENSOR_NDVI          = 10, // Normalized Difference Vegetation Index
    SIGMA_TWIN_SENSOR_POWER_METER   = 11,
    SIGMA_TWIN_SENSOR_GPS           = 12,
    SIGMA_TWIN_SENSOR_CAMERA        = 13, // sigma-ai vision analysis
    SIGMA_TWIN_SENSOR_CUSTOM        = 99,
} sigma_twin_sensor_type_t;

typedef struct {
    char     sensor_id[32];
    sigma_twin_sensor_type_t type;
    char     location[128];          // Physical location description
    char     asset_id[32];           // Which asset this sensor is on
    double   value;                  // Current reading
    char     unit[16];               // "°C", "mm/s", "kPa", etc.
    double   min_normal;             // Normal operating range min
    double   max_normal;             // Normal operating range max
    double   alarm_low;              // Alert below this
    double   alarm_high;             // Alert above this
    time_t   last_reading;
    uint32_t reading_interval_s;     // How often to sample
    bool     online;
    char     protocol[16];           // "MQTT", "Modbus", "OPC-UA", "HTTP"
    char     connection_string[128]; // MQTT topic, Modbus register, etc.
    bool     battery_powered;
    uint8_t  battery_pct;
} sigma_twin_sensor_t;

// ---------------------------------------------------------------------------
// Asset (Physical Entity Being Twinned)
// ---------------------------------------------------------------------------

typedef struct {
    char     asset_id[32];
    char     asset_name[128];
    char     asset_type[64];         // "CNC Machine", "ICU Bed", "Wheat Field Block A"
    char     twin_id[32];            // Parent twin
    sigma_twin_state_t state;
    double   health_score;           // 0-100, predicted by sigma-ai
    time_t   last_maintenance;
    time_t   predicted_failure;      // sigma-ai predicted failure time
    double   failure_probability;    // 0.0-1.0 for next 30 days
    char     maintenance_recommendation[256];
    double   latitude;
    double   longitude;
    // Sensor array
    sigma_twin_sensor_t sensors[32];
    int      sensor_count;
    // Simulation state
    bool     in_simulation;
    char     sim_failure_scenario[64];
} sigma_twin_asset_t;

// ---------------------------------------------------------------------------
// Digital Twin Instance
// ---------------------------------------------------------------------------

typedef struct {
    char     twin_id[32];
    sigma_twin_type_t type;
    char     name[128];
    char     owner_did[128];
    char     location[256];
    bool     live;                   // true = connected to real IoT sensors
    time_t   created_at;
    time_t   last_sync;
    uint32_t asset_count;
    uint32_t sensor_count;
    uint32_t active_alerts;
    double   overall_health_score;
    bool     simulation_running;
    char     simulation_scenario[64];
} sigma_twin_t;

// ---------------------------------------------------------------------------
// Simulation
// ---------------------------------------------------------------------------

typedef struct {
    char     sim_id[32];
    char     twin_id[32];
    char     scenario[64];           // "machine-failure", "fire", "flood", "power-cut"
    char     affected_asset_id[32];
    double   failure_severity;       // 0.0-1.0
    // Predicted impact
    char     predicted_impact[512];
    double   revenue_loss_estimate;  // ₹ estimated loss
    uint32_t downtime_estimate_min;  // Minutes of downtime
    // Recommended mitigations
    char     mitigations[4][256];
    int      mitigation_count;
    time_t   sim_started;
    uint32_t sim_duration_s;
    bool     completed;
} sigma_twin_simulation_t;

// ---------------------------------------------------------------------------
// Factory Twin Specifics
// ---------------------------------------------------------------------------

typedef struct {
    char     machine_id[32];
    char     machine_name[64];
    double   throughput_units_per_hr;
    double   target_throughput;
    double   oee;                    // Overall Equipment Effectiveness (%)
    double   availability;           // % time machine is available
    double   performance;            // % of theoretical max speed
    double   quality_rate;           // % defect-free units
    bool     running;
    bool     alarm_active;
    time_t   last_pm;                // Last Preventive Maintenance
    time_t   next_pm_due;
    uint32_t total_runtime_h;
    double   mtbf_h;                 // Mean Time Between Failures
    double   mttr_h;                 // Mean Time To Repair
} sigma_twin_machine_t;

// ---------------------------------------------------------------------------
// Hospital Twin Specifics
// ---------------------------------------------------------------------------

typedef struct {
    uint32_t total_beds;
    uint32_t occupied_beds;
    uint32_t icu_beds;
    uint32_t icu_occupied;
    uint32_t ot_rooms;               // Operation Theatres
    uint32_t ot_scheduled;
    uint32_t er_waiting;             // ER queue
    double   avg_er_wait_min;
    // Equipment location (RFID/BLE tracking)
    uint32_t ventilators_total;
    uint32_t ventilators_in_use;
    uint32_t defibrillators_total;
    uint32_t defibrillators_available;
    // Mass casualty simulation
    uint32_t mci_capacity;          // Max patients in mass casualty event
    bool     mci_plan_loaded;
} sigma_twin_hospital_status_t;

// ---------------------------------------------------------------------------
// Farm Twin Specifics
// ---------------------------------------------------------------------------

typedef struct {
    double   area_hectares;
    char     crop[64];
    double   ndvi;                   // Current vegetation index (0.0-1.0)
    double   soil_moisture_pct;
    double   temperature_c;
    double   humidity_pct;
    double   rainfall_mm_7day;
    double   evapotranspiration_mm;  // ET0 for irrigation scheduling
    double   yield_prediction_kg_ha; // sigma-ai predicted yield
    double   yield_baseline_kg_ha;   // Historical average
    double   stress_index;           // 0=no stress, 1=severe stress
    char     stress_type[32];        // "drought", "waterlogging", "pest", "nutrient"
    time_t   harvest_predicted;
    double   satellite_ndvi;         // From ISRO Bhuvan RESOURCESAT
    time_t   satellite_last_update;
} sigma_twin_farm_status_t;

// ---------------------------------------------------------------------------
// API
// ---------------------------------------------------------------------------

// Twin lifecycle
int sigma_twin_create(sigma_twin_type_t type, const char *name,
                       const char *owner_did, sigma_twin_t *out);
int sigma_twin_get(const char *twin_id, sigma_twin_t *out);
int sigma_twin_sync(const char *twin_id);              // Pull latest sensor data
int sigma_twin_delete(const char *twin_id);

// Asset management
int sigma_twin_asset_add(const char *twin_id, sigma_twin_asset_t *asset);
int sigma_twin_asset_get(const char *asset_id, sigma_twin_asset_t *out);
int sigma_twin_asset_get_health(const char *asset_id, double *health_score,
                                  time_t *predicted_failure);

// Sensor management
int sigma_twin_sensor_add(const char *asset_id, sigma_twin_sensor_t *sensor);
int sigma_twin_sensor_read(const char *sensor_id, double *value, time_t *ts);
int sigma_twin_sensor_history(const char *sensor_id, time_t from, time_t to,
                                double *values, time_t *timestamps, int *count);

// Simulation
int sigma_twin_simulate(const char *twin_id, const char *scenario,
                          const char *asset_id, sigma_twin_simulation_t *out);
int sigma_twin_simulate_optimize(const char *twin_id,
                                   const char *parameter,   // "energy", "throughput"
                                   const char *constraint,  // "comfort", "cost"
                                   char *recommendation_out,
                                   double *improvement_pct);

// Alerts
int sigma_twin_get_alerts(const char *twin_id,
                            sigma_twin_asset_t *critical_assets, int *count);

// CLI:
// sigma-twin create --type factory --sensors 50 --name "Workshop A"
// sigma-twin simulate --event "machine-failure" --machine M003
// sigma-twin optimize --parameter energy --constraint comfort
// sigma-twin status --twin <id>
// sigma-twin asset health --asset M003
