#include "pm_sensor/sensor_pm.h"

void pm_sensor::SensorPM::start() {
  device.start();
  device.setSleepMode(true);

  // TODO: report measurement, but in a way that it will not be
  // counted as periodical
}

namespace {
  const int32_t warmup_time = 30;
  const int32_t measurement_timeout = 120;
  const int32_t measurement_retry = 2;
}

void pm_sensor::SensorPM::tick(int32_t time) {
  pm_sensor::PMMeasurement result;
  switch(state) {
  case SensorPMState::idle:
    if (!idle_check_time) { idle_check_time = time; }
    if (time - idle_check_time > 30) {
      device.idleCheck();
      idle_check_time = time;
    }
    break;
  case SensorPMState::warmup:
    if (!measure_time) measure_time = time;
    if (time - measure_time >= warmup_time) {
      state = SensorPMState::measure;
    }
    break;
  case SensorPMState::measure:
    result = device.measure();
    if (result) {
      callback(result);
      device.setSleepMode(true);
      state = SensorPMState::idle;
      measure_time = 0;
    } else {
      failed_measure_time = time;
      state = SensorPMState::failed_measure;
    }
    break;
  case SensorPMState::failed_measure:
    if (failed_measure_time - measure_time >= measurement_timeout) {
      device.setSleepMode(true);
      state = SensorPMState::idle;
      measure_time = 0;
    } else if (time - failed_measure_time >= measurement_retry) {
      state = SensorPMState::measure;
    }
    break;
  }
}

void pm_sensor::SensorPM::measure() {
  device.setSleepMode(false);
  state = SensorPMState::warmup;
}
