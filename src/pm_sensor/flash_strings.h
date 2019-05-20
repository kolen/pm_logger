#pragma once
// 'F' macro from arduino-esp8266 conflicts with gmock which have 'F'
// type (gmock-generated-function-mockers.h), so using 'FLS' macro
// here
#ifdef ARDUINO
#define FLS(x) F(x)
#else
#define FLS(x) x
#endif
