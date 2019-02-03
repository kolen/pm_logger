#include "pm_sensor/data_store.h"

int16_t pm_sensor::float_to_stored(float value) {
  return (int16_t)(value * 10.0);
}
