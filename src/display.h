#include "data_store.h"

class Display {
 public:
 Display(DataStore& data): data(data) {};
  void start();
  void update();

 private:
  DataStore& data;
};
