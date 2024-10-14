require_relative 'lib/lago-expression'

parsed = ExpressionParser.parse("1 + 2")
event = Event.new("code", 1234, {"foo" => "bar"})

evaluated = parsed.evaluate(event)
puts "#{evaluated} is a #{evaluated.class}"


parsed = ExpressionParser.parse("concat(event.properties.foo, '-', event.code)")
event = Event.new("code", 1234, {"foo" => "bar"})

evaluated = parsed.evaluate(event)
puts "#{evaluated} is a #{evaluated.class}"
