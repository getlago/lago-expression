Gem::Specification.new do |spec|
  spec.name = 'lago-expression'
  spec.version = '0.0.1'
  spec.summary = '0.0.1'
  spec.authors = ['Lago']
  spec.extensions = ['ext/lago-expression/extconf.rb']
  spec.required_ruby_version = '3.2'

  spec.add_development_dependency 'libclang', '~> 14'
  spec.add_development_dependency 'rake-compiler', '~> 1.2'
  spec.add_development_dependency 'rb_sys', '~> 0.9'
end
