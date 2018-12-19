#include <Arduino.h>
#include <Wire.h>
#include <SdsDustSensor.h>
#include <DHT.h>
#include <LiquidCrystal_PCF8574.h>

int sdaPin = D1;
int sclPin = D2;
int dhtPin = D3;
int rxPin = D5;
int txPin = D6;

int lcdI2CAddress = 0x3f;

SdsDustSensor sds(rxPin, txPin);
LiquidCrystal_PCF8574 lcd(lcdI2CAddress);
DHT dht(dhtPin, DHT22);

void setup() {
  Serial.begin(9600);
  dht.begin();
  sds.begin();


  Wire.begin(sdaPin, sclPin);

  Wire.beginTransmission(lcdI2CAddress);
  auto lcd_result = Wire.endTransmission();
  Serial.print("Result of pinging LCD:");
  Serial.println(lcd_result);

  lcd.begin(16,2);
  lcd.setBacklight(0);
  delay(1000);
  lcd.setBacklight(255);
  delay(1000);
  lcd.setBacklight(50);

  lcd.clear();

  lcd.home();

  Serial.println(sds.queryFirmwareVersion().toString()); // prints firmware version
  Serial.println(sds.setActiveReportingMode().toString()); // ensures sensor is in 'active' reporting mode
  Serial.println(sds.setContinuousWorkingPeriod().toString()); // ensures sensor has continuous working period
  //- default but not recommended
}

void loop() {
  // Reading temperature or humidity takes about 250 milliseconds!
  // Sensor readings may also be up to 2 seconds 'old' (its a very slow sensor)
  float humidity = dht.readHumidity();
  // Read temperature as Celsius (the default)
  float temperature = dht.readTemperature();

  if (isnan(humidity) || isnan(temperature)) {
    Serial.println("Failed to read from DHT sensor!");
    return;
  }

  /*   0123456789ABCDEF

    0  *999.12 +40.1'C/
    1  *999.12  100.0%^
   */

  Serial.print("Humidity: ");
  Serial.print(humidity);
  Serial.print(" %\t");
  Serial.print("Temperature: ");
  Serial.print(temperature);
  Serial.print(" *C ");
  Serial.println("");

  lcd.setCursor(0x8, 0);
  lcd.printf("%+4.1f\xdf" "C", temperature);
  lcd.setCursor(0x9, 1);
  lcd.printf("%5.1f%%", humidity);

  PmResult pm = sds.readPm();
  if (pm.isOk()) {
    Serial.print("PM2.5 = ");
    Serial.print(pm.pm25);
    Serial.print(", PM10 = ");
    Serial.println(pm.pm10);

    lcd.setCursor(0, 0);
    lcd.printf("%6.2f", pm.pm25);
    lcd.setCursor(0, 1);
    lcd.printf("%6.2f", pm.pm10);
  } else {
    Serial.print("Could not read values from sensor, reason: ");
    Serial.println(pm.statusToString());
  }

  delay(1000);
}
