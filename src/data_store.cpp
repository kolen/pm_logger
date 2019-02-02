#include "data_store.h"

int32_t DataStore::float_to_stored(float value) {
  return (int32_t)(value * 10.0);
}
