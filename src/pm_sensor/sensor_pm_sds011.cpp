#include "pm_sensor/sensor_pm_sds011.h"
#include "pm_sensor/logging.h"

using pm_sensor::Logging;
using pm_sensor::SensorPMDeviceSDS011;
using pm_sensor::PMMeasurement;

void SensorPMDeviceSDS011::start() {
  Logging::println(FLS("Starting SDS011"));
  pinMode(switch_pin, OUTPUT);
  // Sleep mode actually turns on the sensor, and is done externally,
  // see #12. So must be turned on before doing anything.
  setSleepMode(false);
  sds.begin();

  // TODO: handle absence/not readiness of SDS011
  Logging::println(sds.queryFirmwareVersion().toString());

  Logging::println(FLS("Checking SDS011 reporting mode"));
  if (sds.queryReportingMode().isActive()) {
    Logging::println(FLS("SDS011 in active reporting mode, setting query reporting mode"));
    sds.setQueryReportingMode();
  }

  Logging::println(FLS("Checking SDS011 working period"));
  if (!sds.queryWorkingPeriod().isContinuous()) {
    Logging::println(FLS("SDS011 has working period enabled, disabling"));
    sds.setContinuousWorkingPeriod();
  }
}

void SensorPMDeviceSDS011::setSleepMode(bool sleep) {
  if (sleep) {
    Logging::println(FLS("SDS011 sleep on (turning off)"));
    digitalWrite(switch_pin, LOW);
  } else {
    Logging::println(FLS("SDS011 sleep off (turning on)"));
    digitalWrite(switch_pin, HIGH);
    // Wait for sensor to turn on. TODO: make it async.
    delay(500);
  }
}

PMMeasurement SensorPMDeviceSDS011::measure() {
  auto pm = sds.queryPm();
  if (pm.isOk()) {
    Logging::print(FLS("SDS011 PM data: "));
    Logging::print(pm.pm25);
    Logging::print(FLS(", "));
    Logging::println(pm.pm10);
    return pm_sensor::PMMeasurement(pm.pm25, pm.pm10);
  } else {
    Logging::print(FLS("Could not read values from sensor, reason: "));
    Logging::println(pm.statusToString());
    return pm_sensor::PMMeasurement();
  }
}

void SensorPMDeviceSDS011::idleCheck() {
  // Sometimes SDS011 wakes up by itself, either due to bug, or to
  // some power instability.
  //
  // "Query working state" command (and probably any communication on
  // serial port) causes SDS011 to wake up (might be a bug in
  // firmware). Putting it to sleep makes it wake up then sleep
  // again. So this does not work.
  //
  // TODO: it should be powered off completely externally.
}
