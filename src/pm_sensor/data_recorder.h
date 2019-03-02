#pragma once

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <stdexcept>
#ifndef ARDUINO
 #include <iostream>
 #include <time.h>
 #include <typeinfo>
#endif

namespace pm_sensor {
  template<typename T>
    class DataRecorder {
  public:
  DataRecorder(T *data_buffer, size_t capacity, int32_t sampling_period) :
    num_samples_capacity(capacity),
      num_samples_filled(0),
      sampling_period(sampling_period),
      last_sample_time(0),
      data_buffer(data_buffer) {
	memset(data_buffer, 0, capacity * sizeof(T));
      }

    void add_sample(T& sample, int32_t sample_time);
    T get_sample(int32_t sample_time) const;

    const size_t num_samples_capacity;
    size_t num_samples_filled;
    const int32_t sampling_period;
    int32_t last_sample_time;
    T *data_buffer; // TODO: make api to access it
  private:
    void shift_data(int num_samples);
  };

  template<typename T>
  void DataRecorder<T>::shift_data(int num_samples) {
    if (!num_samples) return;
    // Not efficient, but less error-prone than multiple pointers or
    // something like that
    int num_samples_moved = num_samples_capacity - num_samples;

    memcpy(data_buffer + (num_samples * sizeof(T)),
	   data_buffer,
	   num_samples_moved * sizeof(T));
    memset(data_buffer, 0, num_samples * sizeof(T));
  }

  template<typename T>
  void DataRecorder<T>::add_sample(T& sample, int32_t sample_time) {
    if (last_sample_time) {
      int32_t time_from_last = sample_time - last_sample_time;

      if (time_from_last % sampling_period != 0) {
	throw std::invalid_argument("Invalid sample time");
      }

      int32_t distance = time_from_last / sampling_period;
      shift_data(distance);
    }

    std::memcpy(data_buffer, &sample, sizeof(T));
    last_sample_time = sample_time;
    if (num_samples_filled < num_samples_capacity) {
      num_samples_filled++;
    }
  }

  template<typename T>
  T DataRecorder<T>::get_sample(int32_t sample_time) const {
    if (!last_sample_time) {
      return T();
    }
    auto time_from_last = sample_time - last_sample_time;

    if (time_from_last % sampling_period != 0) {
      throw std::invalid_argument("Invalid sample time");
    }

    size_t cell_index = time_from_last / sampling_period;

    if (cell_index >= num_samples_capacity) {
      return T(); // TODO: mark as "not found"
    } else {
      return data_buffer[cell_index];
    }
  }

#ifndef ARDUINO
  template<typename T>
  std::ostream &operator<<(std::ostream &os, DataRecorder<T> const &m) {
    os << "DataRecorder " << typeid(T).name() << " " << m.last_sample_time << std::endl;
    size_t i;
    auto time = m.last_sample_time;
    for(i = 0; i < m.num_samples_filled; i++) {
      time_t time1 = (time_t) time;
      struct tm *time_parts = gmtime(&time1);
      char date_s[32];
      strftime(date_s, 32, "%F %H:%M:%S", time_parts);
      os << "  " << date_s << ": " << m.data_buffer[i] << std::endl;
      time -= m.sampling_period;
    }
    return os;
  }
#endif
}
