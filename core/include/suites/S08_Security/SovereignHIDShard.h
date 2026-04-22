/* S SIGMAOS: SOVEREIGN HID SHARD HEADER */
#ifndef SOVEREIGN_HID_SHARD_H
#define SOVEREIGN_HID_SHARD_H
#include "sigma_types.h"

typedef enum { EV_KEY, EV_REL, EV_ABS, EV_MSC } SigmaEvType_t;
typedef struct { SigmaEvType_t type; sigma_u16 code; sigma_i32 value; sigma_u64 timestamp; } SigmaInputEv_t;
typedef enum { HID_KEYBOARD, HID_MOUSE, HID_TOUCHPAD, HID_JOYSTICK } SigmaHIDType_t;

sigma_err_t sigma_hid_register (SigmaHIDType_t type, const char* vendor, sigma_u16 vid, sigma_u16 pid);
void        sigma_hid_push_event(SigmaEvType_t type, sigma_u16 code, sigma_i32 value);
sigma_bool  sigma_hid_pop_event (SigmaInputEv_t* out);
void        SovereignHIDShard_Init (void);
void        SovereignHID_Audit      (void);

#endif
