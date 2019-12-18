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

db = SQLite3::Database.new "#{Dir.home}/.logging_node/logging_node.sqlite"

get '/' do
  js = <<~END
  END

  template = <<~END
    <section>
      <div id="chart-pressure"/>
    </section>
    <section>
      <pre id="refreshProgress" style="background: #000; color: #b9b9b9; overflow: scroll;"></pre>
      <button id="refreshButton">Request fresh data from device</button>
    </section>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/plotly.js/1.49.5/plotly.min.js" integrity="sha256-xHSdfdbRiDxTEfXllbJsH/p3znMFWgSVajVxZaSn958=" crossorigin="anonymous"></script>
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
  stream do |out|
    IO.popen("DATABASE_URL=#{Dir.home}/.logging_node/logging_node.sqlite logging_node", err: [:child, :out]) do |io|
      io.each do |line|
        out << line
      end
    end
    out << "\nProcess finished with return value #{$?.exitstatus}"
  end
end
