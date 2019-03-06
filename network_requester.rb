#!/usr/bin/env ruby
require 'socket'
require 'pp'

GET_CURRENT = 1
GET_RECORDED = 2
GET_RECORDED_BOUNDARIES = 3

RESPONSE_CURRENT = 1
RESPONSE_RECORDED = 2
RESPONSE_RECORDED_BOUNDARIES = 3

DATA_TYPE_PM = 1
DATA_TYPE_TEMPERATURE = 2

udp = UDPSocket.new
udp.connect("localhost", 12000)

def get_current(udp)
  udp.send([GET_CURRENT].pack("c"))

  data = udp.recvfrom(9)[0].unpack("c s>s> s>s>")
  return if data[0] != RESPONSE_CURRENT
  {
    type: :current,
    temperature: data[1],
    humidity: data[1],
    pm2_5: data[2],
    pm10: data[3]
  }
end

def get_recorded(udp, time, type)
  udp.send([GET_RECORDED, time, type].pack("c l> c"))

  data = udp.recvfrom(6)[0].unpack("c l> c s>s>")
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
  end
end

def get_boundaries(udp, type)
  udp.send [GET_RECORDED_BOUNDARIES, type].pack("c c"), 0

  data = udp.recvfrom(6)[0].unpack("cc l>s>")
  return if data[0] != RESPONSE_RECORDED_BOUNDARIES
  return if data[1] != type

  {
    type: :boundaries,
    last_sample: Time.at(data[2]),
    last_sample_ts: data[2],
    num_samples: data[3]
  }
end

pp get_boundaries(udp, DATA_TYPE_PM)
pp get_boundaries(udp, DATA_TYPE_TEMPERATURE)
