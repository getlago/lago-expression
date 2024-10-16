require 'spec_helper'


RSpec.describe Lago::ExpressionParser do
  describe '.parse' do
    it "returns an expression when it's valid" do
      expect(described_class.parse("1+2")).not_to be_nil
    end

    it "returns nil when it's not valid" do
      expect(described_class.parse("1+")).to be_nil
    end
  end
end
