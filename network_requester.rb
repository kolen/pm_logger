#!/usr/bin/env ruby
require 'socket'
require 'pp'
require 'csv'

GET_CURRENT = 1
GET_RECORDED = 2
GET_RECORDED_BOUNDARIES = 3

RESPONSE_CURRENT = 1
RESPONSE_RECORDED = 2
RESPONSE_RECORDED_BOUNDARIES = 3

DATA_TYPE_PM = 1
DATA_TYPE_TEMPERATURE = 2
DATA_TYPE_PRESSURE = 3

udp = UDPSocket.new
udp.connect("pm_sensor.local", 12000)

def get_current(udp)
  udp.send([GET_CURRENT].pack("c"))

  data = udp.recvfrom(9)[0].unpack("c s>s> s>s> l>")
  return if data[0] != RESPONSE_CURRENT
  {
    type: :current,
    temperature: data[1],
    humidity: data[1],
    pm2_5: data[2],
    pm10: data[3],
    pressure: data[4],
  }
end

def get_recorded(udp, time, type)
  udp.send [GET_RECORDED, time, type].pack("c l> c"), 0

  bin = udp.recvfrom(10)[0]
  data = bin.unpack("c l> c s>s>")
  data2 = bin.unpack("c l> c l>")
  return if data[0] != RESPONSE_RECORDED
  return if data[1] != time
  return if data[2] != type

  case data[2]
  when DATA_TYPE_PM
    {
      type: :recorded,
      data_type: :pm,
      pm2_5: data[3],
      pm10: data[4]
    }
  when DATA_TYPE_TEMPERATURE
    {
      type: :recorded,
      data_type: :temperature,
      temperature: data[3],
      humidity: data[4]
    }
  when DATA_TYPE_PRESSURE
    {
      type: :recorded,
      data_type: :pressure,
      pressure: data2[3]
    }
  end
end

def get_boundaries(udp, type)
  udp.send [GET_RECORDED_BOUNDARIES, type].pack("c c"), 0

  data = udp.recvfrom(8)[0].unpack("cc l>s>")
  return if data[0] != RESPONSE_RECORDED_BOUNDARIES
  return if data[1] != type

  {
    type: :boundaries,
    last_sample: Time.at(data[2]),
    last_sample_ts: data[2],
    num_samples: data[3]
  }
end


pm_boundaries = get_boundaries(udp, DATA_TYPE_PM)
temp_boundaries = get_boundaries(udp, DATA_TYPE_TEMPERATURE)
pressure_boundaries = get_boundaries(udp, DATA_TYPE_PRESSURE)
pp pm_boundaries
pp temp_boundaries
pp pressure_boundaries

def each_measurement(boundary, period)
  return if boundary[:last_sample_ts].zero?
  num_samples = boundary[:num_samples]
  time = boundary[:last_sample_ts]
  while num_samples > 0
    sleep 0.1
    yield time
    num_samples -= 1
    time -= period
  end
end

puts "PM measurements"

each_measurement(pm_boundaries, 60*60) do |time|
  data = get_recorded(udp, time, DATA_TYPE_PM)
  CSV do |csv|
    csv << [Time.at(time), data[:pm2_5], data[:pm10]]
  end
end

puts "Temperature/humidity measurements"

each_measurement(temp_boundaries, 60*10) do |time|
  data = get_recorded(udp, time, DATA_TYPE_TEMPERATURE)
  CSV do |csv|
    csv << [Time.at(time), data[:temperature], data[:humidity]]
  end
end

puts "Pressure measurements"

each_measurement(pressure_boundaries, 60*10) do |time|
  data = get_recorded(udp, time, DATA_TYPE_PRESSURE)
  CSV do |csv|
    csv << [Time.at(time), data[:pressure]]
  end
end
