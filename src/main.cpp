#ifndef UNIT_TEST

#include <Arduino.h>
#include <Wire.h>
#include <DHT.h>
#include <TimeLib.h>

#include "pm_sensor/data_store.h"
#include "pm_sensor/display.h"
#include "pm_sensor/server.h"
#include "pm_sensor/sensor_pm_sds011.h"
#include "pm_sensor/sensor_pm.h"

int sdaPin = D1;
int sclPin = D2;
int dhtPin = D3;
int rxPin = D5;
int txPin = D6;

DHT dht(dhtPin, DHT22);

pm_sensor::DataStore data;
pm_sensor::Display display(data);
pm_sensor::Server server(data);
pm_sensor::SensorPMDeviceSDS011 sensor_pm_device(rxPin, txPin);

void pm_measurement_callback(pm_sensor::PMMeasurement measurement) {
  Serial.print("PM2.5 = ");
  Serial.print(measurement.pm2_5);
  Serial.print(", PM10 = ");
  Serial.println(measurement.pm10);
  data.current_pm = measurement;
}

pm_sensor::SensorPM sensor_pm(pm_measurement_callback, sensor_pm_device);


void setup() {
  Serial.begin(9600);
  dht.begin();
  sensor_pm.start();

  Wire.begin(sdaPin, sclPin);

  display.start();
  server.start();
}

int sent = 0;

// void sendData(float pm2_5, float pm10, float temperature, float humidity) {
//   if (sent) return;
//   sent = 1;
//   if (client.connect("api.thingspeak.com", 80)) {
//     // Construct API request body
//     String body = "field1=";
//     body += String(pm2_5);
//     body += "&field2=";
//     body += String(pm10);
//     body += "&field3=";
//     body += String(temperature);
//     body += "&field4=";
//     body += String(humidity);

//     client.println("POST /update HTTP/1.1");
//     client.println("Host: api.thingspeak.com");
//     client.println("User-Agent: ESP8266 (nothans)/1.0");
//     client.println("Connection: close");
//     client.println("X-THINGSPEAKAPIKEY: " THINGSPEAK_API_KEY);
//     client.println("Content-Type: application/x-www-form-urlencoded");
//     client.println("Content-Length: " + String(body.length()));
//     client.println("");
//     client.print(body);
//   }
//   client.stop();
// }

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

  data.current_temperature_humidity = pm_sensor::TemperatureHumidityMeasurement(temperature, humidity);

  sensor_pm.tick(0); // FIXME: pass time in seconds

  display.update();
  server.tick();
  delay(3000);
}

#endif // UNIT_TEST
