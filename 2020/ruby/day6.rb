groups = [[]]
$<.map(&:strip).each{|line|
    if line == ""
        groups << []
    else
        groups[-1] << line.chars
    end
}

# puts groups.map(&:uniq).map(&:size).sum

puts groups.map{|group| group.reduce{|a,c|a&c}.size}.sum