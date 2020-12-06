use std::collections::HashMap;

fn main() {
    let input = flatten(include_str!("answers.txt"));
    let hashed: Vec<HashMap<String, (i32, i32)>> = input.split(", ").map(|s| hash(s)).collect();
    let part_1 = hashed.iter().fold(0, |acc, m| acc + m.keys().len());
    let part_2 = hashed.iter().fold(0, |acc, m| acc + m.iter().fold(0, |acc, kv| { if kv.1.0 == kv.1.1 { acc + 1} else {acc} }));
    println!("part 1: {} part 2: {}", part_1, part_2)
}

fn flatten(s: &str) -> String {
	s.lines().fold("".to_string(), |d, l| {
		if l.to_string() != "" {
			if d == "" {
				return d + l;
			}

			return d + " " + l
		}

		d + ","
	})
}

fn hash(s: &str) -> HashMap<String, (i32, i32)> {
	let mut m: HashMap<String, (i32, i32)> = HashMap::new();
	let sz = s.split(" ").count() as i32;

	s.chars().filter(|c| !c.is_whitespace()).for_each(|ch| {
		let c = m.entry(ch.to_string()).or_insert((0, sz));
		*c = (c.0 + 1, sz)
	});

	m
}
