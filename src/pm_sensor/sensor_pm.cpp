#include "pm_sensor/sensor_pm.h"

void pm_sensor::SensorPM::start() {
  device.start();
}

void pm_sensor::SensorPM::tick(int32_t time) {
  switch(state) {
  case 0:
    // Idle: do nothing
    break;
  case 1:
    // Waiting for warmup
    if (!measure_time) measure_time = time;
    if (time - measure_time >= 10) {
      state = 2;
    }
    break;
  case 2:
    device.measure();
    device.setSleepMode(true);
    state = 0;
    measure_time = 0;
    break;
  }
}

void pm_sensor::SensorPM::measure() {
  device.setSleepMode(false);
  state = 1; // TODO: make enum or something like that
}
