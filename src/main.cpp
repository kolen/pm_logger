#ifndef UNIT_TEST

#include <Arduino.h>
#include <Wire.h>
#include <SdsDustSensor.h>
#include <DHT.h>
#include <ESP8266WiFi.h>
#include <TimeLib.h>

#include "credentials.h"
#include "data_store.h"

int sdaPin = D1;
int sclPin = D2;
int dhtPin = D3;
int rxPin = D5;
int txPin = D6;

SdsDustSensor sds(rxPin, txPin);
DHT dht(dhtPin, DHT22);

WiFiClient client;

DataStore data;

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

  if (sds.queryReportingMode().isActive()) {
    Serial.println("SDS011 in active reporting mode, setting query reporting mode");
    sds.setQueryReportingMode();
  }

  Serial.println("SDS011 fimrware version:");
  Serial.println(sds.queryFirmwareVersion().toString()); // prints firmware version
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

  Serial.print("Humidity: ");
  Serial.print(humidity);
  Serial.print(" %\t");
  Serial.print("Temperature: ");
  Serial.print(temperature);
  Serial.print(" *C ");
  Serial.println("");


  PmResult pm = sds.readPm();
  if (pm.isOk()) {
    Serial.print("PM2.5 = ");
    Serial.print(pm.pm25);
    Serial.print(", PM10 = ");
    Serial.println(pm.pm10);


    sendData(pm.pm25, pm.pm10, temperature, humidity);
  } else {
    Serial.print("Could not read values from sensor, reason: ");
    Serial.println(pm.statusToString());
  }

  delay(1000);
}

#endif // UNIT_TEST
