#pragma once
#include <cstdint>
#include "pm_sensor/data_recorder.h"
#include "pm_sensor/logging.h"

namespace pm_sensor {
  const int temp_humidity_period = 10 * 60;
  const int temp_humidity_capacity = 24 * 10;

  const int pm_period = 60 * 60;
  const int pm_capacity = 48;

  const int pressure_period = 10 * 60;
  const int pressure_capacity = 24 * 6;

  int16_t float_to_stored(float value);
  float stored_to_float(int16_t value);

  struct TemperatureHumidityMeasurement {
  public:
  TemperatureHumidityMeasurement(): temperature(0), humidity(0) {};
  TemperatureHumidityMeasurement(float temperature, float humidity):
    temperature(float_to_stored(temperature)),
      humidity(float_to_stored(humidity)) { }
    int16_t temperature;
    int16_t humidity;
  };

  #ifndef ARDUINO
  std::ostream &operator<<(std::ostream &os, TemperatureHumidityMeasurement const &m);
  #endif

  struct PMMeasurement {
  public:
  PMMeasurement(): pm2_5(0), pm10(0) {};
  PMMeasurement(float pm2_5, float pm10):
    pm2_5(float_to_stored(pm2_5)), pm10(float_to_stored(pm10)) { }
    int16_t pm2_5;
    int16_t pm10;
    operator bool() const { return pm2_5 && pm10; };
  };

  #ifndef ARDUINO
  std::ostream &operator<<(std::ostream &os, PMMeasurement const &m);
  #endif

  class DataStore {
  public:
    DataStore() :
      current_temperature_humidity(),
      current_pm(),
      temp_humidity_recorder(temp_humidity_data, temp_humidity_capacity, temp_humidity_period),
      pm_recorder(pm_data, pm_capacity, pm_period),
      pressure_recorder(pressure_data, pressure_capacity, pressure_period)
    { }

    TemperatureHumidityMeasurement current_temperature_humidity;
    PMMeasurement current_pm;
    int32_t current_pressure;

    TemperatureHumidityMeasurement temp_humidity_data[temp_humidity_capacity] = {};
    PMMeasurement pm_data[pm_capacity] = {};
    int32_t pressure_data[pressure_capacity] = {};

    DataRecorder<TemperatureHumidityMeasurement> temp_humidity_recorder;
    DataRecorder<PMMeasurement> pm_recorder;
    DataRecorder<int32_t> pressure_recorder;

    void addTempHumidity(TemperatureHumidityMeasurement measurement, int32_t time) {
      Logging::print("Humidity: ");
      Logging::print(measurement.humidity);
      Logging::print("%, ");
      Logging::print("temperature: ");
      Logging::print(measurement.temperature);

      current_temperature_humidity = measurement;
      if (time) {
	temp_humidity_recorder.add_sample(measurement, time);
      }

      #ifndef ARDUINO
      std::cout << "Temp/humidity measurements: "
		<< std::endl
		<< temp_humidity_recorder;
      #endif
    }

    void addPM(PMMeasurement measurement, int32_t time) {
      Logging::print("PM2.5 = ");
      Logging::print(measurement.pm2_5);
      Logging::print(", PM10 = ");
      Logging::println(measurement.pm10);

      current_pm = measurement;
      if (time) {
	pm_recorder.add_sample(measurement, time);
      }

      #ifndef ARDUINO
      std::cout << "PM Measurements: "
		<< std::endl
		<< pm_recorder;
      #endif
    }

    void addPressure(int32_t measurement, int32_t time) {
      Logging::print(FLS("Pressure: "));
      Logging::print(measurement);

      current_pressure = measurement;
      if (time) {
        pressure_recorder.add_sample(measurement, time);
      }
    }
  };
}
