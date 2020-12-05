fn main() {
	let mut seats: Vec<i32> = include_str!("boarding-passes.txt")
		.lines()
		.map(|l| seat_id(l))
		.collect();

	println!("part 1: {}", seats.iter().max().unwrap());

	seats.sort();

	println!("part 2: {:#?}", my_seat(seats))
}

// TODO: use closures???
fn my_seat(v: Vec<i32>) -> i32 {
	let mut prev = 0;
	for s in v.iter() {
		if prev == 0 {
			prev = *s;
			continue
		}

		if s - prev != 1 {
			return s - 1
		}

		prev = *s
	}

	return 0  
}

fn seat_id(s: &str) -> i32 {
	row(s) * 8  + column(s) 
}

fn row(s: &str) -> i32 {
	s.chars().fold((0, 127), |acc, c| {
			match c {
				'F' => lower(acc),
				'B' => upper(acc),
				_ => acc
			}
		}).0
}

fn column(s: &str) -> i32 {
	s.chars().fold((0, 7), |acc, c| {
		match c {
			'L' => lower(acc),
			'R' => upper(acc),
			_ => acc
		}
	}).0
}

fn lower(t: (i32, i32)) -> (i32, i32) {
	(t.0, t.1 - d(t).floor() as i32)
}

fn upper(t: (i32, i32)) -> (i32, i32) {
	(t.0 + d(t).ceil() as i32, t.1)
}

fn d(t: (i32, i32)) -> f32 {
	(t.1 - t.0) as f32 / 2.0
}
