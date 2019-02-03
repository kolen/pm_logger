#include <cstdint>
#include <Arduino.h>
#include <Wire.h>
#include "pm_sensor/display.h"

#define ONE_DIGIT_AFTER_POINT(x) (x)/10, (x)%10

static uint8_t customChar2_5[] = {
  0x00,
  0x1B,
  0x0A,
  0x1B,
  0x11,
  0x1B,
  0x00,
  0x04
};

static uint8_t customChar10[] = {
  0x00,
  0x17,
  0x15,
  0x15,
  0x15,
  0x17,
  0x00,
  0x00
};

static uint8_t customCharConnection[] = {
  0x00,
  0x0E,
  0x11,
  0x04,
  0x0A,
  0x00,
  0x04,
  0x00
};

void pm_sensor::Display::start() {
  Wire.beginTransmission(i2c_address);
  auto lcd_result = Wire.endTransmission();
  Serial.print("Initializing LCD:");
  Serial.println(lcd_result);

  lcd.begin(16,2);
  lcd.setBacklight(0);
  lcd.clear();
  lcd.home();

  lcd.createChar(0, customChar2_5);
  lcd.createChar(1, customChar10);
  lcd.createChar(2, customCharConnection);

  this->data = data;
}

void pm_sensor::Display::update() {
  /*   0123456789ABCDEF

    0  *999.1  +40.1'C/
    1  *999.1   100.0%^
   */

  lcd.setCursor(0x8, 0);
  lcd.printf("%+4d.%1d\xdf" "C", ONE_DIGIT_AFTER_POINT(data.current_temperature));
  lcd.setCursor(0x9, 1);
  lcd.printf("%4d.%1d%%", ONE_DIGIT_AFTER_POINT(data.current_humidity));

  lcd.setCursor(0, 0);
  lcd.write("\0", 1);
  lcd.printf("%5d.%1d", ONE_DIGIT_AFTER_POINT(data.current_pm2_5));
  lcd.setCursor(0, 1);
  lcd.write("\1", 1);
  lcd.printf("%5d.%1d", ONE_DIGIT_AFTER_POINT(data.current_pm10));
}
