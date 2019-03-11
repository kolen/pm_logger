#include "pm_sensor/sensor_dht.h"
#include <cmath>

using pm_sensor::SensorDHT;
using std::isnan;

const int32_t RETRY_PERIOD = 5;

void SensorDHT::start() {
  #ifdef ARDUINO
  dht.begin();
  #endif
}

void SensorDHT::measure() {
  #ifdef ARDUINO
  float humidity = dht.readHumidity();
  float temperature = dht.readTemperature();
  #else
  float humidity = 12.34;
  float temperature = 56.78;
  #endif

  Logging::print(PSTR("Temperature: "));
  Logging::print(temperature);
  Logging::print(PSTR(", humidity: "));
  Logging::println(humidity);
  if (isnan(humidity) || isnan(temperature)) {
    Logging::println("Failed to read from DHT sensor!");
    retry_at = -1;
    return;
  }

  callback(TemperatureHumidityMeasurement(temperature, humidity));
}

void SensorDHT::tick(int32_t time) {
  if (retry_at == 0) {
    return;
  } else if (retry_at == -1) {
    retry_at = time + RETRY_PERIOD;
  } else if (time > retry_at) {
    retry_at = 0;
    measure();
  }
}
