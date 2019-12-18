#include "pm_sensor/cayenne_notifier.h"

#ifdef ARDUINO
#include <CayenneMQTTESP8266.h>
#define INFO_DEVICE "Wemos D1 Mini"
#endif

using pm_sensor::CayenneNotifier;
using pm_sensor::stored_to_float;

enum class CayenneChannel: unsigned int
  {
   pm2_5 = 1,
   pm10 = 2,
   temperature = 3,
   humidity = 4,
   pressure = 5
  };

void CayenneNotifier::begin() {
  #ifdef ARDUINO
  Cayenne.begin(username, password, clientID, NULL, NULL);
  #else
  Logging::print("Init cayenne ");
  Logging::print(username);
  Logging::print(" ");
  Logging::print(password);
  Logging::print(" ");
  Logging::println(clientID);
  #endif
}

void CayenneNotifier::tick(int32_t time) {
  #ifdef ARDUINO
  Cayenne.loop();
  #endif
}

void CayenneNotifier::notifyPM(PMMeasurement pm) {
  #ifdef ARDUINO
  Cayenne.virtualWrite(static_cast<unsigned int>(CayenneChannel::pm2_5),
                       stored_to_float(pm.pm2_5),
                       "pm2.5", "ppm");
  Cayenne.virtualWrite(static_cast<unsigned int>(CayenneChannel::pm10),
                       stored_to_float(pm.pm10),
                       "pm10", "ppm");
  #endif
}

void CayenneNotifier::notifyTempHumidity(TemperatureHumidityMeasurement tempHumidity) {
  #ifdef ARDUINO
  Cayenne.virtualWrite(static_cast<unsigned int>(CayenneChannel::temperature),
                       tempHumidity.temperature,
                       "temp", "c");
  Cayenne.virtualWrite(static_cast<unsigned int>(CayenneChannel::humidity),
                       tempHumidity.humidity,
                       "humidity", "p");
  #endif
}

void CayenneNotifier::notifyPressure(int32_t pressure) {
  #ifdef ARDUINO
  Cayenne.pascalWrite(static_cast<unsigned int>(CayenneChannel::pressure),
                      pressure);
  #endif
}
