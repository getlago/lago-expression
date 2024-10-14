require_relative 'lib/lago-expression'
require 'benchmark/ips'

parsed = ExpressionParser.parse("concat(event.properties.foo, '-', event.code)")
event = Event.new("code", 1234, {"foo" => "bar"})

evaluated = parsed.evaluate(event)
puts "#{evaluated} is a #{evaluated.class}"

hash = {"foo" => "bar"}
code = "code"

Benchmark.ips do |x|
  x.report("regular") { [hash['foo'], '-', code].join('') }

  x.report("rust") { parsed.evaluate(event) }
  # Compare the iterations per second of the various reports!
  x.compare!
end
