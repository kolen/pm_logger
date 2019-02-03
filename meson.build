project('pm_sensor', 'cpp', license: 'MIT')
add_project_arguments('-std=c++11', language: 'cpp')
gtest = subproject('gtest')
gtest_dep = dependency('gtest', main: true, fallback: ['gtest', 'gtest_dep'])

include_dirs = include_directories('src')

lib = shared_library('pm_sensor',
                     'src/pm_sensor/data_store.cpp',
                     include_directories: include_dirs)
lib_dep = declare_dependency(link_with: lib, include_directories: include_dirs)

test_data_store = executable('test_data_store', 'test/test_data_store.cpp', dependencies: [gtest_dep, lib_dep])
test('DataStore', test_data_store)