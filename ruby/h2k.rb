require 'yaml'

file = File.open(ARGV[0])

content = file.read

content_arr = content.split(/^\-\-\-$/)

content_arr.each do |content|
    if content == ""
        next
    end
    content_yaml = YAML.load(content)
    puts(content)
    out_file_name = "./" + content_yaml["kind"].downcase + "-" + content_yaml["metadata"]["name"].downcase + ".yaml"
    File.write(out_file_name, content)
end