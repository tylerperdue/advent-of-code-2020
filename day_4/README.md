## Day 4

### Part 1

Validate each passport in `src/passports.txt` and return the number of valid passports. 

Definition of valid passport: contains: byr, iyr, eyr, hgt, hcl, ecl, pid.

### Part 2

Validate each passport in `src/passports.txt` and return the number of valid passports. 

Definition of valid passport: contains: byr, iyr, eyr, hgt, hcl, ecl, pid. 

byr: four digits; at least 1920 and at most 2002.

iyr: four digits; at least 2010 and at most 2020.

eyr: four digits; at least 2020 and at most 2030.

hgt: a number followed by either cm or in:
- if cm, the number must be at least 150 and at most 193.
- if in, the number must be at least 59 and at most 76.

hcl: a # followed by exactly six characters 0-9 or a-f.

ecl: exactly one of: amb blu brn gry grn hzl oth.

pid: a nine-digit number, including leading zeroes.

cid: ignored, missing or not.

***

### Reflection

More string parsing!!! Ugh

Not super happy with the code but used structs and struct methods.

Was hoping to learn more closures but just didn't have the time today. 

Lots of error scenarios here... which I think could be avoided using more iterators.
