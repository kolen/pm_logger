#pragma once

#include <cstddef>
#include <cstdint>
#include <cstring>

struct data_recorder_t {
  char* data_buffer;
  size_t sample_size;
  size_t num_samples_capacity;
  size_t num_samples_filled;
  int32_t sampling_period;
  int32_t last_sample_time;
};

void data_recorder_init(struct data_recorder_t *self, char* data_buffer, size_t sample_size, size_t num_samples, int32_t sampling_period);
int data_recorder_add_sample(struct data_recorder_t *self, int32_t sample_time, const void* sample);
