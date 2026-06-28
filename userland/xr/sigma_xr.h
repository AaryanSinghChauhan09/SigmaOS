// SPDX-License-Identifier: GPL-2.0-only
// sigma_xr.h — SigmaOS Extended Reality (AR/VR) Platform
// Purpose: OpenXR 1.1 runtime on SigmaOS. AR overlays for professionals
//          (architects, doctors, engineers). VR workspace. India's sovereign
//          XR platform — no Meta, no Google, no foreign cloud dependency.

#pragma once
#include <stdint.h>
#include <stdbool.h>

// ---------------------------------------------------------------------------
// XR Session Types
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_XR_MODE_AR_PASSTHROUGH  = 1, // Webcam/camera passthrough + overlay
    SIGMA_XR_MODE_VR_IMMERSIVE    = 2, // Full VR (HMD required)
    SIGMA_XR_MODE_MR_MIXED        = 3, // Mixed reality (HMD with passthrough)
    SIGMA_XR_MODE_DESKTOP_3D      = 4, // 3D desktop without HMD (depth camera)
} sigma_xr_mode_t;

typedef enum {
    SIGMA_XR_DEVICE_NONE          = 0, // No XR device (phone/webcam only)
    SIGMA_XR_DEVICE_META_QUEST    = 1,
    SIGMA_XR_DEVICE_PICO          = 2,
    SIGMA_XR_DEVICE_HTCVIVE       = 3,
    SIGMA_XR_DEVICE_PHONE_AR      = 4, // ARCore-compatible phone
    SIGMA_XR_DEVICE_CUSTOM        = 99,
} sigma_xr_device_type_t;

// ---------------------------------------------------------------------------
// AR Overlay Data Sources
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_XR_OVERLAY_RERA_DATA    = 1, // Property info (RERA) on building scan
    SIGMA_XR_OVERLAY_BHUVAN_MAP   = 2, // ISRO Bhuvan GIS data
    SIGMA_XR_OVERLAY_VITALS       = 3, // Patient vitals (sigma-health)
    SIGMA_XR_OVERLAY_MACHINE_INFO = 4, // Machine manual overlay (sigma-twin)
    SIGMA_XR_OVERLAY_WIRING_PLAN  = 5, // Electrical wiring (sigma-electrical)
    SIGMA_XR_OVERLAY_BUILDING_PLAN = 6,// Architectural plan on real site
    SIGMA_XR_OVERLAY_NAVIGATION   = 7, // Turn-by-turn NavIC navigation
    SIGMA_XR_OVERLAY_QR_SCAN      = 8, // QR → AR content
    SIGMA_XR_OVERLAY_CUSTOM       = 99,
} sigma_xr_overlay_source_t;

typedef struct {
    sigma_xr_overlay_source_t source;
    char     data_url[256];           // API endpoint for overlay data
    double   anchor_lat;              // GPS anchor for geo-AR
    double   anchor_lon;
    float    offset_x_m;             // Offset from anchor in meters
    float    offset_y_m;
    float    offset_z_m;
    float    scale;                   // Display scale
    bool     world_anchored;          // If true: stays on physical object
    bool     visible;
    float    opacity;                 // 0.0-1.0
    char     label[64];               // Display label
} sigma_xr_overlay_t;

// ---------------------------------------------------------------------------
// VR Workspace
// ---------------------------------------------------------------------------

typedef struct {
    uint32_t virtual_screens;         // Number of virtual monitor panels
    uint32_t screen_width_px;         // Per screen resolution
    uint32_t screen_height_px;
    float    screen_spacing_m;        // Distance between virtual screens
    float    curvature;               // Panel curvature (0=flat, 1=full wrap)
    bool     hand_tracking;
    bool     eye_tracking;
    bool     voice_control;           // sigma-bhashini voice in VR
    bool     sigma_apps_in_vr;        // Run sigma-* apps in VR panels
    // VR meeting room
    bool     meeting_room_enabled;    // sigma-meet in VR
    uint8_t  max_participants;
    // VR training simulations
    bool     surgery_sim;             // Medical students
    bool     fire_drill_sim;          // Safety officers
    bool     courtroom_sim;           // Law students (sigma-legal)
    bool     factory_sim;             // Operator training (sigma-twin)
} sigma_xr_vr_workspace_t;

// ---------------------------------------------------------------------------
// Professional AR Use Cases
// ---------------------------------------------------------------------------

typedef struct {
    // Architect AR
    bool     building_plan_overlay;   // CAD plan overlaid on real site via GPS
    bool     rera_property_info;      // Scan building → RERA data popup
    bool     structural_analysis;     // Stress analysis visualization

    // Medical AR
    bool     patient_vitals_overlay;  // See patient EMR while examining
    bool     anatomy_3d;              // 3D anatomy overlay on patient
    bool     drug_info_scan;          // Scan drug → dosage/interaction info

    // Electrician AR
    bool     wiring_behind_walls;     // Thermal cam + AR = see hidden wiring
    bool     circuit_diagram_overlay; // Live circuit diagram on panel

    // Factory/Maintenance AR
    bool     machine_manual_overlay;  // Scan machine → maintenance guide
    bool     component_identification;// Identify parts by scanning
    bool     repair_step_guide;       // Step-by-step repair overlay

    // Navigation AR
    bool     navic_ar_navigation;     // NavIC-based AR turn-by-turn
    bool     indoor_navigation;       // Hospital/mall indoor AR nav
} sigma_xr_professional_features_t;

// ---------------------------------------------------------------------------
// API
// ---------------------------------------------------------------------------

// Session management
int sigma_xr_init(sigma_xr_device_type_t device);
int sigma_xr_create_session(sigma_xr_mode_t mode);
int sigma_xr_end_session(void);
bool sigma_xr_device_connected(sigma_xr_device_type_t *detected);

// AR overlays
int sigma_xr_overlay_add(sigma_xr_overlay_t *overlay, char *overlay_id_out);
int sigma_xr_overlay_update(const char *overlay_id, sigma_xr_overlay_t *overlay);
int sigma_xr_overlay_remove(const char *overlay_id);
int sigma_xr_overlay_rera_scan(double lat, double lon,
                                 char *rera_data_json, size_t len);

// VR workspace
int sigma_xr_vr_workspace_configure(sigma_xr_vr_workspace_t *ws);
int sigma_xr_vr_meeting_start(const char *room_id);
int sigma_xr_vr_simulation_start(const char *sim_type);  // "surgery", "fire-drill"

// Device listing
int sigma_xr_devices_list(sigma_xr_device_type_t *types, int *count);

// CLI:
// sigma-xr devices list
// sigma-xr ar overlay --source rera-data --camera front
// sigma-xr ar overlay --source building-plan --lat 28.61 --lon 77.20
// sigma-xr vr workspace enable --screens 3 --resolution 4K
// sigma-xr vr simulation --type fire-drill
