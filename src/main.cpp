#ifdef ARDUINO
 #include <Arduino.h>
 #include <Wire.h>
#endif

#include "pm_sensor/data_store.h"
#include "pm_sensor/server.h"
#ifdef ARDUINO
 #include "pm_sensor/display.h"
 #include "pm_sensor/arduino_network_responder.h"
 #include "pm_sensor/sensor_pm_sds011.h"
#else
 #include "pm_sensor/posix_network_responder.h"
 #include "pm_sensor/sensor_pm_fake.h"
#endif
#include "pm_sensor/sensor_pm.h"
#include "pm_sensor/sensor_dht.h"
#include "pm_sensor/time.h"
#include "pm_sensor/scheduler.h"
#include "pm_sensor/logging.h"

using namespace pm_sensor;

#ifdef ARDUINO
// int sdaPin = D1;
// int sclPin = D2;
int dhtPin = D7;
int rxPin = D5;
int txPin = D6;
int sds_switch_pin = D0;
#else
int dhtPin = -1;
#endif

DataStore data;
#ifdef ARDUINO
Display display(data);
ArduinoNetworkResponder network_responder;
#else
// TODO: display
PosixNetworkResponder network_responder;
#endif

pm_sensor::Server server(data, network_responder);

#ifdef ARDUINO
SensorPMDeviceSDS011 sensor_pm_device(rxPin, txPin, sds_switch_pin);
#else
SensorPMDeviceFake sensor_pm_device;
#endif

HourlyScheduler hourly_scheduler;
MinutelyScheduler minutely_scheduler;

Time time_;

int32_t pm_sample_time = 0;
int32_t temp_sample_time = 0;

void pm_measurement_callback(PMMeasurement measurement) {
  data.addPM(measurement, pm_sample_time);
}

void dht_measurement_callback(TemperatureHumidityMeasurement measurement) {
  data.addTempHumidity(measurement, temp_sample_time);
}

SensorPM sensor_pm(pm_measurement_callback, sensor_pm_device);
SensorDHT sensor_dht(dht_measurement_callback, dhtPin);

void hourlySchedulerCallback(int32_t current_time) {
  pm_sample_time = current_time;
  sensor_pm.measure();
}

void minutelySchedulerCallback(int32_t current_time) {
  temp_sample_time = current_time;
  sensor_dht.measure();
}

void setup() {
  #ifdef ARDUINO
  Serial.begin(9600);
  #endif

  Logging::println(FLS("pm_sensor starting"));

  Logging::println(FLS("Starting server and networking"));
  server.start();

  // TODO: make it work without wifi (and therefore time) too. Wifi is
  // required for time.
  #ifdef ARDUINO
  while (WiFi.status() != WL_CONNECTED)
  {
    delay(500);
    Serial.print(".");
  }
  Serial.println("");
  #endif

  Logging::println(FLS("Starting time"));
  time_.start();

  sensor_pm.start();

  Logging::println(FLS("Setting up scheduler"));

  //                              3   7   11  15  19  23
  hourly_scheduler.hours_mask = 0b100000000000111111111111;
  hourly_scheduler.callback = hourlySchedulerCallback;

  minutely_scheduler.period = 10;
  minutely_scheduler.callback = minutelySchedulerCallback;

  #ifdef ARDUINO
  // i2c is disabled for now
  //Wire.begin(sdaPin, sclPin);
  #endif

  Logging::println(FLS("Setting up DHT22"));
  sensor_dht.start();

  // FIXME: should not be under ifdef
  #ifdef ARDUINO
  // Display is disabled for now
  //display.start();
  #endif

  Logging::println(FLS("Initialization complete, entering main loop"));
}

int sent = 0;

void loop() {
  //Logging::println(FLS("Main loop - time tick"));
  time_.tick();
  //Logging::println(FLS("Main loop - getting current time"));
  int32_t current_time = time_.now();

  //Logging::println(FLS("Main loop - schedulers tick"));
  hourly_scheduler.tick(current_time);
  minutely_scheduler.tick(current_time);

  //Logging::println(FLS("Main loop - sensors tick"));
  sensor_pm.tick(current_time);
  sensor_dht.tick(current_time);

  // FIXME: should not be under ifdef
  #ifdef ARDUINO
  //display.update();
  #endif

  //Logging::println(FLS("Main loop - server tick"));
  server.tick();

  //Logging::println(FLS("Main loop - delay"));

  // FIXME: should be no delay at all, or very small one
  #ifdef ARDUINO
  delay(10);
  #endif
  //Logging::println(FLS("Main loop - finished"));
}

#ifndef ARDUINO
int main(int argc, char *argv[]) {
  std::cout << "Running setup" << std::endl;
  setup();
  std::cout << "Running main loop" << std::endl;
  for(;;) {
    loop();
    usleep(100000);
  }
}
#endif
