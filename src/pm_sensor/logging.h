#pragma once

#ifdef ARDUINO
#include <Arduino.h>
#else
#include <iostream>
#endif

namespace pm_sensor {
  class Logging {
  public:
    template<typename S> static void print(S s) {
      #ifdef ARDUINO
      Serial.print(s);
      #else
      std::cout << s;
      #endif
    }

    template<typename S> static void println(S s) {
      #ifdef ARDUINO
      Serial.println(s);
      #else
      std::cout << s << std::endl;
      #endif
    }
  };
}
