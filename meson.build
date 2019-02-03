project('pm_sensor', 'cpp', license: 'MIT')
gtest = subproject('gtest')
gtest_dep = dependency('gtest', main: true, fallback: ['gtest', 'gtest_dep'])

e = executable('test_foo', 'test/test_foo.cpp', dependencies: gtest_dep)

test('foo test', e)
