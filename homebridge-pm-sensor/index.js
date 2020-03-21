var Accessory, Service, Characteristic, UUIDGen;
var dgram = require('dgram');

module.exports = function(homebridge) {
  Accessory = homebridge.platformAccessory;

  Service = homebridge.hap.Service;
  Characteristic = homebridge.hap.Characteristic;
  UUIDGen = homebridge.hap.uuid;

  homebridge.registerAccessory("homebridge-pm-sensor", "PMSensor", PMSensor, true);
}

function PMSensor(log, config) {
  // this.category = Accessory.Categories.SENSOR;
  this.log = log;

  this.displayName = config["name"];

  this.temperatureService = new Service.TemperatureSensor(`Temperature ${this.displayName}`);
  this.humidityService = new Service.HumiditySensor(`Humidity ${this.displayName}`);

  this.socket = dgram.createSocket('udp4');
  this.interval = config.interval || 60 * 5;

  this.temperatureService = new Service.TemperatureSensor("Temperature");
  this.humidityService = new Service.HumiditySensor("Humidity");
  this.airQualityService = new Service.AirQualitySensor("Air quality");
  this.airQualityService.addOptionalCharacteristic(Characteristic.PM2_5Density);
  this.airQualityService.addOptionalCharacteristic(Characteristic.PM10Density);

  this.informationService = new Service.AccessoryInformation();
  this.informationService
    .setCharacteristic(Characteristic.Manufacturer, "Internet der Scheiße GmbH")
    .setCharacteristic(Characteristic.Model, "G-1")
    .setCharacteristic(Characteristic.SerialNumber, "123-456-789")
    .setCharacteristic(Characteristic.FirmwareRevision, 3);

  zis = this;
  this.socket.on("message", function(message, rinfo) {
    // FIXME: this is bug in firmware, length should be 9
    if (message.length != 13 || message[0] != 1) {
      return;
    }

    zis.temperature = (message[1] << 8 | message[2]) * 0.1;
    zis.humidity =    (message[3] << 8 | message[4]) * 0.1;
    zis.pm2_5 =       (message[5] << 8 | message[6]) * 0.1;
    zis.pm10 =        (message[7] << 8 | message[8]) * 0.1;
    zis.pressure =    (message[9] << 24 | message[10] << 16 | message[11] << 8 | message[12]);

    zis.log("temp: ", zis.temperature, " hum: ", zis.humidity, " pm: ", zis.pm2_5, zis.pm10, "pressure: ", zis.pressure);

    zis.temperatureService.setCharacteristic(Characteristic.CurrentTemperature, zis.temperature);
    zis.humidityService.setCharacteristic(Characteristic.CurrentRelativeHumidity, zis.humidity);
    zis.airQualityService.setCharacteristic(Characteristic.PM2_5Density, zis.pm2_5);
    zis.airQualityService.setCharacteristic(Characteristic.PM10Density, zis.pm10);
    zis.airQualityService.setCharacteristic(Characteristic.AirQuality, zis.airQuality(zis.pm2_5, zis.pm10));
    // There's no characteristic for pressure in HomeKit
  });

  setInterval(this.poll.bind(this), this.interval * 1000);
  this.poll();
}

PMSensor.prototype = {
  getServices: function() {
    return [this.informationService, this.temperatureService, this.humidityService, this.airQualityService];
  },

  poll: function() {
    var message = Buffer.from([1]);
    this.socket.send(message, 0, 1, 12000, "pm_sensor.local", function(err, bytes) {
      console.log("send", err, bytes);
    });
  },

  airQuality: function(pm2_5, pm10) {
    // https://en.wikipedia.org/wiki/Air_quality_index#Europe

    var airQuality25 = (
      function() {
        if (pm2_5 <= 15) {
          return Characteristic.AirQuality.EXCELLENT;
        } else if (pm2_5 <= 30) {
          return Characteristic.AirQuality.GOOD;
        } else if (pm2_5 <= 55) {
          return Characteristic.AirQuality.FAIR;
        } else if (pm2_5 <= 110) {
          return Characteristic.AirQuality.INFERIOR;
        } else {
          return Characteristic.AirQuality.POOR;
        }
      })();

    var airQuality10 = (
      function() {
        if (pm10 <= 25) {
          return Characteristic.AirQuality.EXCELLENT;
        } else if (pm10 <= 50) {
          return Characteristic.AirQuality.GOOD;
        } else if (pm10 <= 90) {
          return Characteristic.AirQuality.FAIR;
        } else if (pm10 <= 180) {
          return Characteristic.AirQuality.INFERIOR;
        } else {
          return Characteristic.AirQuality.POOR;
        }
      })();

    return Math.max(airQuality25, airQuality10);
  }
}
