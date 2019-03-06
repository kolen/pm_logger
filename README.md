[![CircleCI](https://circleci.com/gh/kolen/pm_logger.svg?style=svg)](https://circleci.com/gh/kolen/pm_logger)

# Building firmware

* Install [platformio](https://platformio.org)
* Run `pio run` in root dir

To flash firmware:

* Run `pio run -t upload`

# Running tests

* Install native C++ toolchain (for your system, not for cross-compilation), clang or gcc
* Install [meson](https://mesonbuild.com/) build system and [ninja](https://ninja-build.org/) low-level build system
* Run `meson builddir` in the root directory of project
* Change dir to `builddir` and run `ninja build`

# Using native simulator

Native simulator runs on unix-like systems. It runs the same main loop
as actual app and has networking, but instead of real sensor readings,
it reads the same value each time. Simulated time is 60x faster that
real time, i.e. each simulated hour is a real minute.

To build native simulator, use the same steps as for tests, but just
run `ninja`. It builds `pm_sensor_fake` executable in `builddir`.
