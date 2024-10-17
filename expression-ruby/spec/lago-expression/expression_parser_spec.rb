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

  describe '.validate' do
    it "returns nil when it's valid" do
      expect(described_class.validate("1+2")).to be_nil
    end

    it "returns an error when it's not valid" do
      error = described_class.validate("1+")
      expect(error).not_to be_nil
      expect(error).to include('1+')
      expect(error).to include('expected')
    end
  end
end
