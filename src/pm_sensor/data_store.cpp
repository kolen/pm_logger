#include "pm_sensor/data_store.h"
#include <cmath>

int16_t pm_sensor::float_to_stored(float value) {
  return (int16_t) round(value * 10.0);
}
