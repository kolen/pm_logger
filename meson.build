project('pm_sensor', 'cpp', license: 'MIT')
add_project_arguments('-std=c++11', language: 'cpp')
#add_project_arguments(['-fsanitize=address', '-fno-omit-frame-pointer'], language: 'cpp')
gtest = subproject('gtest')
gtest_dep = dependency('gtest', main: true, fallback: ['gtest', 'gtest_dep'])

include_dirs = include_directories('src')

lib = shared_library('pm_sensor',
                     'src/pm_sensor/scheduler.cpp',
                     'src/pm_sensor/data_store.cpp',
                     'src/pm_sensor/sensor_pm.cpp',
                     include_directories: include_dirs)
lib_dep = declare_dependency(link_with: lib, include_directories: include_dirs)

test_data_store = executable('test_data_store', 'test/test_data_store.cpp', dependencies: [gtest_dep, lib_dep])
test('DataStore', test_data_store)
test_data_recorder = executable('test_data_recorder', 'test/test_data_recorder.cpp', dependencies: [gtest_dep, lib_dep])
test('DataRecorder', test_data_recorder)
test_scheduler = executable('test_scheduler', 'test/test_scheduler.cpp', dependencies: [gtest_dep, lib_dep])
test('Scheduler', test_scheduler)
test_sensor_pm = executable('test_sensor_pm', 'test/test_sensor_pm.cpp', dependencies: [gtest_dep, lib_dep])
test('Scheduler', test_sensor_pm)
