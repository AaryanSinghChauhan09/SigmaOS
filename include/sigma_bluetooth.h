/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN BLUETOOTH STACK (S-BT)
 * =========================================================================
 * Mission: Silicon-native BT 5.x + BLE stack without external daemon bloat.
 * Competitor parity: Linux BlueZ / macOS CoreBluetooth / Windows BT Stack.
 * ZERO-DEPENDENCY: Direct HCI controller orchestration.
 * =========================================================================
 */

#ifndef SIGMA_BLUETOOTH_H
#define SIGMA_BLUETOOTH_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Bluetooth State --- */
#define SIGMA_BT_OFF         0x00u
#define SIGMA_BT_SCANNING    0x01u
#define SIGMA_BT_PAIRED      0x02u
#define SIGMA_BT_CONNECTED   0x03u
#define SIGMA_BT_BLE_ACTIVE  0x04u

#define SIGMA_BT_ADDR_LEN    6u
#define SIGMA_BT_NAME_LEN    32u

typedef struct {
    sigma_u8  addr[SIGMA_BT_ADDR_LEN]; /* 6-byte BD_ADDR              */
    char      name[SIGMA_BT_NAME_LEN]; /* Friendly device name         */
    sigma_i32 rssi;                    /* Signal strength dBm          */
    sigma_u8  state;                   /* SIGMA_BT_* state flag        */
} sigma_bt_device_t;

typedef struct {
    sigma_u8  controller_state;  /* SIGMA_BT_* global state      */
    sigma_u32 paired_count;
    sigma_u32 scan_interval_ms;
} sigma_bt_config_t;

/* --- Bluetooth Primitives --- */
void bt_init(void);
void bt_enable(void);
void bt_disable(void);
void bt_start_scan(sigma_u32 duration_ms);
void bt_stop_scan(void);
void bt_pair(const sigma_u8* addr);
void bt_disconnect(const sigma_u8* addr);
sigma_u32 bt_get_paired_count(void);
const sigma_bt_config_t* bt_get_config(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_BLUETOOTH_H */
