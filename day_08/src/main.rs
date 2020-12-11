use std::collections::HashMap;

fn main() {
	let input = include_str!("boot-code.txt");
	println!("part 1: {} part 2: {}", part_1(input), part_2(input))
}

fn part_1(s: &str) -> i32 {
	let lines: Vec<&str> = s.lines().collect();

	let mut acc = 0;
	let mut index = 0;
	let mut map: HashMap<i32, bool> = HashMap::new();

	while !map.contains_key(&index) {
		let split: Vec<&str> = lines[index as usize].split(" ").collect();
		let op = split[0];
		let incr = split[1].parse::<i32>().unwrap();

		map.insert(index, true);

		if op == "jmp" {
			index += incr;
			continue
		}

		if op == "acc" {
			acc += incr
		}

		index += 1
	}

	acc
}

fn part_2(s: &str) -> i32 {
	let lines: Vec<String> = s.lines().map(|s| s.to_string()).collect();

	let mut perms: HashMap<i32, String> = HashMap::new();
	for index in 0..lines.len() {
		let split: Vec<&str> = lines[index as usize].split(" ").collect();
		let op = split[0];

		let new = match op {
			"jmp" => "nop".to_owned() + " " + split[1],
			"nop" => "jmp".to_owned() + " " + split[1],
			_ => op.to_owned() + " " + split[1],
		};
		
		if op != "acc" {
			perms.insert(index as i32, new);
		}
	}

	for (perm, swap) in perms {
		let mut map: HashMap<i32, bool> = HashMap::new();

		let mut acc = 0;
		let mut index = 0;

		while !map.contains_key(&index) {
			if (index as usize) == lines.len() {
				return acc
			}

			let instr = if index == perm {
				&swap
			} else {
				&lines[index as usize]
			};

			let split: Vec<&str> = instr.split(" ").collect();
			let op = split[0];
			let incr = split[1].parse::<i32>().unwrap();

			map.insert(index, true);

			if op == "jmp" {
				index += incr;
				continue
			}

			if op == "acc" {
				acc += incr
			}

			index += 1;
		}
	}

	0
}
