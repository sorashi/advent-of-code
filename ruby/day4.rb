passports = []
current_passport = {}
for line in $<
    if line.strip == ""
        passports << current_passport.clone
        current_passport = {}
    else
        current_passport.merge!(line.split.map{|x|x.split':'}.to_h)
    end
end
passports << current_passport.clone
valid = []

@validations = {
    "byr" => -> (v) { /^\d{4}$/ =~ v and v.to_i >= 1920 and v.to_i <= 2002 },
    "iyr" => -> (v) { /^\d{4}$/ =~ v and v.to_i >= 2010 and v.to_i <= 2020 },
    "eyr" => -> (v) { /^\d{4}$/ =~ v and v.to_i >= 2020 and v.to_i <= 2030 },
    "hgt" => -> (v) { 
        if match = v.match(/^(\d+)(in|cm)$/)
            number, unit = match.captures
            number = number.to_i
            if unit == "cm" and number >= 150 and number <= 193
                next true
            elsif unit == "in" and number >= 59 and number <= 76
                next true
            else
                next false
            end
        end
        next false
    },
    "hcl" => -> (v) { /^#[0-9a-f]{6}$/ =~ v },
    "ecl" => -> (v) { "amb blu brn gry grn hzl oth".split.include? v },
    "pid" => -> (v) { /^\d{9}$/ =~ v }
}

def validate_passport(passport)
    raise StandardError if passport.nil? or not passport.instance_of? Hash
    for k, v in passport
        if @validations.key?(k) and not @validations[k].call(v)
            return false
        end 
    end
    return true
end

for passport in passports
    if @validations.all?{|k,_| passport.key?(k)} and validate_passport(passport)
        valid << passport
    end
end
p valid.size