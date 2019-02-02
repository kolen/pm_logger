#include "data_store.h"

namespace pm_sensor {
  class Display {
  public:
  Display(DataStore& data): data(data) {};
    void start();
    void update();

  private:
    DataStore& data;
  };
}
