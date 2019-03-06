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
