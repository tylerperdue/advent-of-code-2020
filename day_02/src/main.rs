use std::fs;

use std::error::Error;

#[derive(Debug)]
struct Entry(Policy, String);

impl Entry {
	fn new(s: String) -> Result<Entry, Box<dyn Error>> {
		let p = s.split(' ').collect::<Vec<&str>>();

		Ok(Entry(Policy::new(p[0].to_string(), p[1].to_string())?, p[2].to_string()))
	}
}

#[derive(Debug)]
struct Policy {
	a: usize,
	b: usize,
	l: String,
}

impl Policy {
	fn new(p0: String, p1: String) -> Result<Policy, Box<dyn Error>> {
		let ab = p0.split('-').collect::<Vec<&str>>();

		Ok(Policy{
			a: ab[0].parse::<usize>()?,
			b: ab[1].parse::<usize>()?,
			l: p1.split(':').collect::<Vec<&str>>()[0].to_string(),
		})
	}
}

fn get_passwords() -> Result<String, Box<dyn Error>> {
	let s = fs::read_to_string("src/passwords.txt")?;
	Ok(s)
}

fn parse_passwords(s: String) -> Result<Vec<Entry>, Box<dyn Error>> {
	let mut v: Vec<Entry> = Vec::new();

	for l in s.split('\n') {
		let e = Entry::new(l.to_string())?;
		v.push(e)
	}

	Ok(v)
}

fn valid_count_part_1(v: Vec<Entry>) -> i32 {
	let mut ans = 0;

	for e in v {
		let c = e.1.split(&e.0.l).count() - 1;

		if c >= e.0.a && c <= e.0.b {
			ans += 1
		}
	}

	ans
}

fn valid_count_part_2(v: Vec<Entry>) -> Result<i32, Box<dyn Error>> {
	let mut ans = 0;

	for e in v {
		let l = e.1.chars().nth(e.0.a - 1).unwrap().to_string();
		let r = e.1.chars().nth(e.0.b - 1).unwrap().to_string();

		if l != r && (l == e.0.l || r == e.0.l) {
			ans += 1
		}
	}

	Ok(ans)
}

fn main() {
	println!("part 1: {}", valid_count_part_1(get_passwords().and_then(|s: String| parse_passwords(s)).unwrap()));
	println!("part 2: {}", get_passwords().and_then(|s: String| parse_passwords(s)).and_then(|v: Vec<Entry>| valid_count_part_2(v)).unwrap());
}
