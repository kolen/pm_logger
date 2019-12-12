#pragma once

#include <cstdint>
#include "pm_sensor/data_store.h"

namespace pm_sensor {
  class CayenneNotifier {
  public:
    CayenneNotifier(char* username, char* password, char* clientID):
      username(username), password(password), clientID(clientID) { }

    void begin();
    void tick(int32_t time);
    void notifyPM(PMMeasurement pm);
    void notifyTempHumidity(TemperatureHumidityMeasurement tempHumidity);
    void notifyPressure(int32_t pressure);
  private:
    char* username;
    char* password;
    char* clientID;
  };
}
