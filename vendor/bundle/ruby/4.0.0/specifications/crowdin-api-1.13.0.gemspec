# -*- encoding: utf-8 -*-
# stub: crowdin-api 1.13.0 ruby lib

Gem::Specification.new do |s|
  s.name = "crowdin-api".freeze
  s.version = "1.13.0".freeze

  s.required_rubygems_version = Gem::Requirement.new(">= 0".freeze) if s.respond_to? :required_rubygems_version=
  s.require_paths = ["lib".freeze]
  s.authors = ["Crowdin".freeze]
  s.date = "1980-01-02"
  s.description = "The Crowdin Ruby Client is used to interact with the Crowdin API from Ruby".freeze
  s.email = ["support@crowdin.com".freeze]
  s.executables = ["crowdin-console".freeze]
  s.files = ["bin/crowdin-console".freeze]
  s.homepage = "https://github.com/crowdin/crowdin-api-client-ruby".freeze
  s.licenses = ["MIT".freeze]
  s.required_ruby_version = Gem::Requirement.new(">= 2.4".freeze)
  s.rubygems_version = "3.6.9".freeze
  s.summary = "Ruby Client for the Crowdin API".freeze

  s.installed_by_version = "4.0.6".freeze

  s.specification_version = 4

  s.add_runtime_dependency(%q<open-uri>.freeze, [">= 0.1.0".freeze, "< 0.2.0".freeze])
  s.add_runtime_dependency(%q<rest-client>.freeze, [">= 2.0.0".freeze, "< 2.2.0".freeze])
  s.add_development_dependency(%q<bundler>.freeze, ["~> 2.2".freeze, ">= 2.2.32".freeze])
  s.add_development_dependency(%q<pry>.freeze, ["~> 0.14.1".freeze])
  s.add_development_dependency(%q<rake>.freeze, ["~> 13.0".freeze, ">= 13.0.6".freeze])
  s.add_development_dependency(%q<rspec>.freeze, ["~> 3.10".freeze])
  s.add_development_dependency(%q<rubocop>.freeze, ["~> 1.23".freeze])
  s.add_development_dependency(%q<simplecov>.freeze, ["~> 0.22".freeze])
  s.add_development_dependency(%q<simplecov-cobertura>.freeze, ["~> 2.1".freeze])
  s.add_development_dependency(%q<webmock>.freeze, ["~> 3.14".freeze])
end
