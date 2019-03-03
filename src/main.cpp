#ifdef ARDUINO
 #include <Arduino.h>
 #include <Wire.h>
 #include <DHT.h>
 #include <TimeLib.h>
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
#include "pm_sensor/time.h"
#include "pm_sensor/scheduler.h"
#include "pm_sensor/logging.h"

using namespace pm_sensor;

#ifdef ARDUINO
int sdaPin = D1;
int sclPin = D2;
int dhtPin = D3;
int rxPin = D5;
int txPin = D6;
#endif

#ifdef ARDUINO
DHT dht(dhtPin, DHT22);
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
SensorPMDeviceSDS011 sensor_pm_device(rxPin, txPin);
#else
SensorPMDeviceFake sensor_pm_device;
#endif

HourlyScheduler hourly_scheduler;
MinutelyScheduler minutely_scheduler;

int32_t pm_sample_time = 0;
int32_t temp_sample_time = 0;

void pm_measurement_callback(PMMeasurement measurement) {
  data.addPM(measurement, pm_sample_time);
}

SensorPM sensor_pm(pm_measurement_callback, sensor_pm_device);

// TODO: temporary method
void readTempHumidity(int32_t current_time) {
  temp_sample_time = current_time;

  #ifdef ARDUINO
  // Reading temperature or humidity takes about 250 milliseconds!
  // Sensor readings may also be up to 2 seconds 'old' (its a very slow sensor)
  float humidity = dht.readHumidity();
  // Read temperature as Celsius (the default)
  float temperature = dht.readTemperature();

  if (isnan(humidity) || isnan(temperature)) {
    Logging::println("Failed to read from DHT sensor!");
    return;
  }

  data.addTempHumidity(TemperatureHumidityMeasurement(temperature, humidity), current_time);
  #else
  data.addTempHumidity(TemperatureHumidityMeasurement(11.11, 22.22), current_time);
  #endif
}

void setup() {
  #ifdef ARDUINO
  Serial.begin(9600);
  dht.begin();
  #endif

  Time::start();

  sensor_pm.start();

  //                              3   7   11  15  19  23
  hourly_scheduler.hours_mask = 0b100000000000111111111111;
  hourly_scheduler.callback = [] (int32_t current_time) {
				pm_sample_time = current_time;
				sensor_pm.measure();
			      };

  minutely_scheduler.period = 10;
  minutely_scheduler.callback = &readTempHumidity;

  #ifdef ARDUINO
  Wire.begin(sdaPin, sclPin);
  #endif

  // FIXME: should not be under ifdef
  #ifdef ARDUINO
  display.start();
  #endif

  server.start();
}

int sent = 0;

void loop() {
  Time::tick();
  int32_t current_time = Time::now();

  hourly_scheduler.tick(current_time);
  minutely_scheduler.tick(current_time);

  sensor_pm.tick(current_time);

  // FIXME: should not be under ifdef
  #ifdef ARDUINO
  display.update();
  #endif

  server.tick();

  // FIXME: should be no delay at all, or very small one
  #ifdef ARDUINO
  delay(10);
  #endif
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
