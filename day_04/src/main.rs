use std::collections::HashMap;

fn main() {
    let input = include_str!("passports.txt")
    	.lines()
    	.fold("".to_string(), |state, l| {
    		if l.to_string() != "" {
    			if state == "" {
    				return state + l;
    			}

    			return state + " " + l;
    		}

    		state + ","
    	});

    let hashed: Vec<HashMap<String, String>> = input.split(", ").map(|p| hash(p)).collect();

    let part_1 = hashed.iter().filter(|h| validations(h, false)).count();
    let part_2 = hashed.iter().filter(|h| validations(h, true)).count();

    println!("part 1: {} part 2: {}", part_1, part_2)
}

fn hash(s: &str) -> HashMap<String, String> {
	let mut h: HashMap<String, String> = HashMap::new();

	for f in s.split(' ') {
		let kv: Vec<&str> = f.split(':').collect();

		h.insert(kv[0].to_string(), kv[1].to_string());
	}

	h
}

fn validations(h: &HashMap<String, String>, tsa_mode: bool) -> bool {
	for k in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
		if !h.contains_key(k) {
			return false
		}

		if tsa_mode {
			if !valid(k, h.get(k).unwrap()) {
				return false
			}
		}
	}

	true
}

fn valid(k: &str, v: &str) -> bool {
	match k {
		"byr" => valid_byr(v),
		"iyr" => valid_iyr(v),
		"eyr" => valid_eyr(v),
		"hgt" => valid_hgt(v),
		"hcl" => valid_hcl(v),
		"ecl" => valid_ecl(v),
		"pid" => valid_pid(v),
		_ => false
	}
}

fn valid_byr(s: &str) -> bool {
	let p = s.parse::<i32>();
	if p.is_err() {
		return false
	}

	let x = p.unwrap();

	x >= 1920 && x <= 2002 
}

fn valid_iyr(s: &str) -> bool {
	let p = s.parse::<i32>();
	if p.is_err() {
		return false
	}

	let x = p.unwrap();

	x >= 2010 && x <= 2020 
}

fn valid_eyr(s: &str) -> bool {
	let p = s.parse::<i32>();
	if p.is_err() {
		return false
	}

	let x = p.unwrap();

	x >= 2020 && x <= 2030
}

fn valid_hgt(s: &str) -> bool {
	if s.contains("cm") {
		let v: Vec<&str> = s.split("cm").collect();
		let o = v.get(0);

		if o.is_none() {
			return false
		}

		let p = o.unwrap().parse::<i32>();
		if p.is_err() {
			return false
		}

		let x = p.unwrap();

		return x >= 150 && x <= 193
	}

	if s.contains("in") {
		let v: Vec<&str> = s.split("in").collect();
		let o = v.get(0);

		if o.is_none() {
			return false
		}

		let p = o.unwrap().parse::<i32>();
		if p.is_err() {
			return false
		}

		let x = p.unwrap();

		return x >= 59 && x <= 76
	}

	false
}

fn valid_hcl(s: &str) -> bool {
	let v: Vec<char> = s.chars().collect();

	if v.len() != 7 {
		return false
	}

	for (i, c) in v.iter().enumerate() {
		if i == 0 {
			if *c != '#' {
				return false
			}

			continue
		}

		if !c.is_alphanumeric() {
			return false
		}
	}

	true
}

fn valid_ecl(s: &str) -> bool {
	match s {
		"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
		_ => false
	}
}

fn valid_pid(s: &str) -> bool {
	if s.chars().count() != 9 {
		return false
	}

	let p = s.parse::<i32>();
	if p.is_err() {
		return false
	}

	true
}

