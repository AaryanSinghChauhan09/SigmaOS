#ifndef SIGMA_SENSOR_FUSION_H
#define SIGMA_SENSOR_FUSION_H

#include <stdint.h>

// SigmaOS Sensor Fusion Shard
// Modular integration for Gyroscopes, Accelerometers, and Magnetometers.

typedef struct {
    float x, y, z;
} SensorData;

// Initialize hardware sensor bus (I2C/SPI/HID)
void hal_sensor_init_fusion(void);

// Poll raw data from accelerometer
SensorData hal_sensor_read_accel(void);

// Poll raw data from gyroscope
SensorData hal_sensor_read_gyro(void);

// Calculate screen orientation/tilt for OS interaction
uint8_t hal_sensor_get_orientation(void);

#endif // SIGMA_SENSOR_FUSION_H
