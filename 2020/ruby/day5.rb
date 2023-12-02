codes = [] 
 
 $<.map(&:strip).each{|x|
    lower = 0
    upper = 127
    x[..6].chars.each{|c|
        if c == "F"
            upper = lower + (upper - lower) / 2
        else
            lower = lower + (upper - lower) / 2
        end
    }
    row = upper
    lower = 0
    upper = 8
    x[7..].chars.each{|c|
        if c == "L"
            upper = lower + (upper - lower) / 2
        else
            lower = lower + (upper - lower) / 2
        end
    }
    column = lower
    codes << row*8+column
}
puts "silver: #{codes.max}"

min = codes.min
max = codes.max
puts "gold: #{(min..max).to_a - codes}"