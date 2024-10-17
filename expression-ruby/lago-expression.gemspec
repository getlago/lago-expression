Gem::Specification.new do |spec|
  spec.name = 'lago-expression'
  spec.version = '0.0.1'
  spec.summary = '0.0.1'
  spec.authors = ['Lago']
  spec.required_ruby_version = '~> 3.3'
  spec.required_rubygems_version = ">= 3.3.11"

  spec.require_paths = ["lib"]
  spec.extensions = ['ext/lago_expression/Cargo.toml']

  spec.add_dependency 'bigdecimal'
  spec.add_dependency 'libclang', '~> 14'
  spec.add_dependency 'rake', '~> 13'
  spec.add_dependency 'rake-compiler', '~> 1.2'
  spec.add_dependency 'rb_sys', '~> 0.9.63'
  spec.add_development_dependency 'rspec', '~> 3'
end
