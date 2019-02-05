#include "pm_sensor/sensor_pm_sds011.h"

void pm_sensor::SensorPMDeviceSDS011::start() {
  sds.begin();

  if (sds.queryReportingMode().isActive()) {
    Serial.println("SDS011 in active reporting mode, setting query reporting mode");
    sds.setQueryReportingMode();
  }

  Serial.println("SDS011 fimrware version:");
  Serial.println(sds.queryFirmwareVersion().toString());

  // TODO: report measurement, but in a way that it will not be counted as periodical
}

void pm_sensor::SensorPMDeviceSDS011::setSleepMode(bool sleep) {
  if (sleep) {
    sds.sleep();
  } else {
    sds.wakeup();
  }
}

pm_sensor::PMMeasurement pm_sensor::SensorPMDeviceSDS011::measure() {
  auto pm = sds.queryPm();
  if (pm.isOk()) {
    return pm_sensor::PMMeasurement(pm.pm25, pm.pm10);
  } else {
    Serial.print("Could not read values from sensor, reason: ");
    Serial.println(pm.statusToString());
    return pm_sensor::PMMeasurement();
  }
}
