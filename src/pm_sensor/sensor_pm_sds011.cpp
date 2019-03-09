#include "pm_sensor/sensor_pm_sds011.h"
#include "pm_sensor/logging.h"

using pm_sensor::Logging;
using pm_sensor::SensorPMDeviceSDS011;
using pm_sensor::PMMeasurement;

void SensorPMDeviceSDS011::start() {
  Logging::println(PSTR("Starting SDS011"));
  sds.begin();

  if (sds.queryReportingMode().isActive()) {
    Logging::println(PSTR("SDS011 in active reporting mode, setting query reporting mode"));
    sds.setQueryReportingMode();
  }

  Logging::println("SDS011 fimrware version:");
  Logging::println(sds.queryFirmwareVersion().toString());
}

void SensorPMDeviceSDS011::setSleepMode(bool sleep) {
  if (sleep) {
    sds.sleep();
  } else {
    sds.wakeup();
  }
}

PMMeasurement SensorPMDeviceSDS011::measure() {
  auto pm = sds.queryPm();
  if (pm.isOk()) {
    return pm_sensor::PMMeasurement(pm.pm25, pm.pm10);
  } else {
    Logging::print(PSTR("Could not read values from sensor, reason: "));
    Logging::println(pm.statusToString());
    return pm_sensor::PMMeasurement();
  }
}
