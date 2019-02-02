#ifndef UNIT_TEST

#include <Arduino.h>
#include <Wire.h>
#include <SdsDustSensor.h>
#include <DHT.h>
#include <LiquidCrystal_PCF8574.h>
#include <ESP8266WiFi.h>
#include "credentials.h"

int sdaPin = D1;
int sclPin = D2;
int dhtPin = D3;
int rxPin = D5;
int txPin = D6;

int lcdI2CAddress = 0x3f;

SdsDustSensor sds(rxPin, txPin);
LiquidCrystal_PCF8574 lcd(lcdI2CAddress);
DHT dht(dhtPin, DHT22);

byte customChar2_5[] = {
  0x00,
  0x1B,
  0x0A,
  0x1B,
  0x11,
  0x1B,
  0x00,
  0x04
};

byte customChar10[] = {
  0x00,
  0x17,
  0x15,
  0x15,
  0x15,
  0x17,
  0x00,
  0x00
};

byte customCharConnection[] = {
  0x00,
  0x0E,
  0x11,
  0x04,
  0x0A,
  0x00,
  0x04,
  0x00
};

WiFiClient client;

void setup() {
  Serial.begin(9600);
  dht.begin();
  sds.begin();

  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  // TODO: don't wait for it here
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
  }

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

  lcd.createChar(0, customChar2_5);
  lcd.createChar(1, customChar10);
  lcd.createChar(2, customCharConnection);

  Serial.println(sds.queryFirmwareVersion().toString()); // prints firmware version
  Serial.println(sds.setActiveReportingMode().toString()); // ensures sensor is in 'active' reporting mode
  Serial.println(sds.setContinuousWorkingPeriod().toString()); // ensures sensor has continuous working period
  //- default but not recommended
}

int sent = 0;

void sendData(float pm2_5, float pm10, float temperature, float humidity) {
  if (sent) return;
  sent = 1;
  if (client.connect("api.thingspeak.com", 80)) {
    // Construct API request body
    String body = "field1=";
    body += String(pm2_5);
    body += "&field2=";
    body += String(pm10);
    body += "&field3=";
    body += String(temperature);
    body += "&field4=";
    body += String(humidity);

    client.println("POST /update HTTP/1.1");
    client.println("Host: api.thingspeak.com");
    client.println("User-Agent: ESP8266 (nothans)/1.0");
    client.println("Connection: close");
    client.println("X-THINGSPEAKAPIKEY: " THINGSPEAK_API_KEY);
    client.println("Content-Type: application/x-www-form-urlencoded");
    client.println("Content-Length: " + String(body.length()));
    client.println("");
    client.print(body);
  }
  client.stop();
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
    lcd.write("\0", 1);
    lcd.printf("%6.2f", pm.pm25);
    lcd.setCursor(0, 1);
    lcd.write("\1", 1);
    lcd.printf("%6.2f", pm.pm10);

    sendData(pm.pm25, pm.pm10, temperature, humidity);
  } else {
    Serial.print("Could not read values from sensor, reason: ");
    Serial.println(pm.statusToString());
  }

  delay(1000);
}

#endif // UNIT_TEST
