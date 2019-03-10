#!/usr/bin/env ruby

# based on https://github.com/me-no-dev/EspExceptionDecoder/blob/master/src/EspExceptionDecoder.java

require 'pp'
require 'optparse'

GDB = File.join Dir.home, ".platformio", "packages", "toolchain-xtensa", "bin", "xtensa-lx106-elf-gdb"

elf_file = ARGV.shift
raise "Specify elf file" unless elf_file

addresses = ARGF.readlines.flat_map do |s|
  s.scan(/(?<=\s)40[0-2][0-9a-f]{5}(?=\s|$)/)
end

address_exs = addresses.flat_map do |addr|
  ["-ex", "l *0x#{addr}"]
end

command = [GDB] + ["--batch", elf_file, "-ex", "set listsize 1"] + address_exs + ["-ex", "q"]
system *command
