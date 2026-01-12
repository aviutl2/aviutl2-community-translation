task default: %i[clean prepare download convert_locale]

task :clean do
  rm_rf "tmp/aviutl2"
  rm_rf "tmp/aviutl2.zip"
  puts "Cleaned up temporary files."
end
task :prepare do
  mkdir_p "tmp"
  File.write("tmp/.gitignore", "*\n")
  puts "Prepared temporary directory."
end

task :download do
  require "open-uri"
  require "zip"

  html_url = "https://spring-fragrance.mints.ne.jp/aviutl/"
  html_content = URI.open(html_url).read
  zip_filename = html_content.match(/<A HREF="(aviutl2beta[0-9a-z]+\.zip)">/)[1]
  zip_url = html_url + zip_filename
  zip_path = "tmp/aviutl2.zip"
  puts "Downloading #{zip_url}..."
  URI.open(zip_url) do |remote_file|
    File.open(zip_path, "wb") { |file| file.write(remote_file.read) }
  end
  puts "Downloaded to #{zip_path}."

  Zip::File.open(zip_path) do |zip_file|
    zip_file.each do |entry|
      dest_path = File.join("tmp", entry.name)
      FileUtils.mkdir_p(File.dirname(dest_path))
      zip_file.extract(entry, dest_path) unless File.exist?(dest_path)
    end
  end
  puts "Extracted ZIP to tmp/aviutl2."
end

task :convert_locale do
  puts "TODO"
end
