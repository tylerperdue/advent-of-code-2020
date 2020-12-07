use std::collections::HashMap;

fn main() {
	let input = include_str!("rules.txt");
	let hashed = hash(input);
	let part_1 = hashed.keys().fold(0, |acc, k| if can_hold_a_shiny_gold(&hashed, k) {acc + 1} else {acc});
	let part_2 = count_bags(&hashed, "shiny gold");
	println!("part 1: {} part 2: {}", part_1, part_2)
}

fn hash(s: &str) -> HashMap<String, Vec<(String, i32)>> {
	let mut m: HashMap<String, Vec<(String, i32)>> = HashMap::new();

	for l in s.lines() {
		let x: Vec<&str> = l.split(" bags contain ").collect();

		if x[1] != "no other bags." {
			let mut v: Vec<(String, i32)> = Vec::new();
			for r in x[1].split(", ") {
				let y: Vec<&str> = r.split(" ").collect();
				v.push((y[1].to_owned() + " " + y[2], y[0].parse::<i32>().unwrap()));
			}

			m.insert(x[0].to_string(), v);
		}
	}

	m
}

fn can_hold_a_shiny_gold(m: &HashMap<String, Vec<(String, i32)>>, k: &str) -> bool {
	if m.get(k).is_none() {
		return false
	}

	if m.get(k).unwrap().iter().filter(|c| c.0.to_string() == "shiny gold").count() > 0 {
		return true
	}

	for c in m.get(k).unwrap() {
		if can_hold_a_shiny_gold(m, &c.0) {
			return true
		}
	}

	return false
}

fn count_bags(m: &HashMap<String, Vec<(String, i32)>>, k: &str) -> i32 {
	if m.get(k).is_none() {
		return 0
	}

	m.get(k).unwrap().iter().fold(0, |c, t| c + t.1 + t.1 * count_bags(m, &t.0))
}
