require 'spec_helper'


RSpec.describe Lago::Expression do

  class Dummy < Struct.new(:v)
    def to_s
      v
    end
  end

  let(:event) { Lago::Event.new("code", 1234, {"property_1" => 1.23, "dummy" => Dummy.new(1), "decimal_property" => BigDecimal("2.3"), "property_2" => "test", "property_3" => "12.34"}) }

  describe '#evaluate' do
    context "with a simple math expression" do
      let(:expression) { Lago::ExpressionParser.parse("1 + 3") }

      it "returns a bigdecimal" do
        expect(expression.evaluate(event)).to eq(4.to_d)
        expect(expression.evaluate(event)).to be_a(BigDecimal)
      end
    end

    context "failing evaluation" do
      let(:expression) { Lago::ExpressionParser.parse('event.properties.does_not_exists') }

      it "raises an error" do
        expect {expression.evaluate(event)}.to raise_error(RuntimeError, /does_not_exists not found/)
      end
    end

    context "with a simple string expression" do
      let(:expression) { Lago::ExpressionParser.parse("'test'") }

      it "returns a string" do
        expect(expression.evaluate(event)).to eq('test')
        expect(expression.evaluate(event)).to be_a(String)
      end
    end

    context "with a math expression with a decimal value from the event" do
      let(:expression) { Lago::ExpressionParser.parse("(123 - event.properties.property_1) / 10") }

      it "returns the calculated value" do
        expect(expression.evaluate(event)).to eq(12.177)
      end
    end

    context "with a string expression with a decimal value from the event" do
      let(:expression) { Lago::ExpressionParser.parse("CONCAT(event.properties.property_1, 'test')") }

      it "returns the calculated value" do
        expect(expression.evaluate(event)).to eq("1.23test")
      end
    end

    context "with a decimal expression with a decimal value from the event" do
      let(:expression) { Lago::ExpressionParser.parse("event.properties.property_1 + event.properties.decimal_property") }

      it "returns the calculated value" do
        expect(expression.evaluate(event)).to eq(3.53)
      end
    end

    context "with a non-number / string value in the event payload" do
      let(:expression) { Lago::ExpressionParser.parse("event.properties.dummy") }

      it "returns the calculated value" do
        expect(expression.evaluate(event)).not_to be(nil)
      end
    end

    context "with a concat function" do
      let(:expression) { Lago::ExpressionParser.parse("concat(event.properties.property_2, '-', 'suffix')") }

      it "concats the string" do
        expect(expression.evaluate(event)).to eq('test-suffix')
      end
    end

    context "with rounding function" do
      let(:expression) { Lago::ExpressionParser.parse("round(event.properties.property_3, -1)") }

      it "rounds the property" do
        expect(expression.evaluate(event)).to eq(10)
      end
    end
  end
end
