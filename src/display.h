#pragma once
#include "data_store.h"
#include <LiquidCrystal_PCF8574.h>

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
