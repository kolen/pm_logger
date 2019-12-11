[![CircleCI](https://circleci.com/gh/kolen/pm_logger.svg?style=svg)](https://circleci.com/gh/kolen/pm_logger)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=kolen/mm_map_tools)](https://dependabot.com)

# Firmware

## Building firmware

* Install [platformio](https://platformio.org)
* Run `pio run` in root dir

To flash firmware:

* Run `pio run -t upload`

## Running tests

* Install native C++ toolchain (for your system, not for cross-compilation), clang or gcc
* Install [meson](https://mesonbuild.com/) build system and [ninja](https://ninja-build.org/) low-level build system
* Run `meson builddir` in the root directory of project
* Change dir to `builddir` and run `ninja test`

## Using native simulator

Native simulator runs on unix-like systems. It runs the same main loop
as actual app and has networking, but instead of real sensor readings,
it reads the same value each time. Simulated time is 60x faster that
real time, i.e. each simulated hour is a real minute.

To build native simulator, use the same steps as for tests, but just
run `ninja`. It builds `pm_sensor_fake` executable in `builddir`.

# Logging node

## Building and installing

* Install Rust and Cargo
* Run `cargo build` in `logging_node` directory
* Install [Diesel](http://diesel.rs/) cli: `cargo install diesel_cli
  --no-default-features --features sqlite` (by default, all features
  are installed, which require client libs for postgresql and mysql,
  which are not needed for logging node)
* Run schema migration: `diesel migration run`
* Run built binary directly or with `cargo run`

# Homebridge plugin

## Installing

* Install node.js
* Install and configure [homebridge](https://github.com/nfarina/homebridge)
* Run `npm install -g` in `homebridge-pm-sensor` dir
* Add accessory to homebridge:

  ```json
      "accessories": [
          {
              "accessory": "PMSensor",
              "name": "pm_sensor"
          }
      ]
  ```
