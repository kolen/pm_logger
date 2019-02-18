#pragma once
#include "pm_sensor/data_store.h"
#ifdef ARDUINO
 #include <LiquidCrystal_PCF8574.h>
#endif

namespace pm_sensor {
  class Display {
  public:
  Display(DataStore& data): data(data), lcd(i2c_address) {};
    void start();
    void update();

  private:
    DataStore& data;
    int i2c_address = 0x3f;
    LiquidCrystal_PCF8574 lcd;
  };
}
