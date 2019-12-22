#!/usr/bin/env ruby
require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'sqlite3', '~> 1.4.1'
  gem 'sinatra', '~> 2.0.7'
  gem 'thin', '~> 1.7.2'
end

require 'sqlite3'
require 'sinatra'
require 'date'
require 'bigdecimal'
require 'pty'

db = SQLite3::Database.new "#{Dir.home}/.logging_node/logging_node.sqlite"

get '/' do
  js = <<~END
  END

  template = <<~END
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/xterm/3.14.5/xterm.min.css" integrity="sha256-uTIrmf95e6IHlacC0wpDaPS58eWF314UC7OgdrD6AdU=" crossorigin="anonymous" />
    <section>
      <div id="chart-pressure"/>
    </section>
    <section>
      <div id="terminal" style="width: 0; padding: 2px;"></div>
      <button id="refreshButton">Request fresh data from device</button>
    </section>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/plotly.js/1.49.5/plotly.min.js" integrity="sha256-xHSdfdbRiDxTEfXllbJsH/p3znMFWgSVajVxZaSn958=" crossorigin="anonymous"></script>
    <!-- This is ancient version, as new versions does not work with script tag without npm -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/xterm/3.14.5/xterm.min.js" integrity="sha256-tDeULIXIGkXbz7dkZ0qcQajBIS22qS8jQ6URaeMoVJs=" crossorigin="anonymous"></script>
    <script src="app.js"></script>
  END
end

get '/app.js' do
  headers "Cache-Control" => "must-revalidate"
  send_file 'chart_server.js'
end

def format_time(timestamp)
  Time.at(timestamp).strftime('%Y-%m-%d %H:%M')
end

def format_pressure(pressure_pa)
  return nil if pressure_pa.nil?
  BigDecimal(pressure_pa) / BigDecimal("100")
end

get '/data/pressure.json' do
  headers "Content-Type" => "application/json"
  db.execute("select * from measurements_pressure;")
    .map { |ts, pressure| [format_time(ts), format_pressure(pressure)] }
    .transpose
    .to_json
end

post '/refresh' do
  command = "DATABASE_URL=#{Dir.home}/.logging_node/logging_node.sqlite logging_node"

  PTY.spawn(command) do |reader, _writer, pid|
    stream do |out|
      out << "Started logging_node (pid #{pid})\r\n"
      begin
        while true
          fragment = reader.readpartial(0x1000)
          out << fragment
        end
      rescue EOFError
        out << "\r\nProcess finished\r\n"
      end
    end
  end
end
