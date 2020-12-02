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
	min: i32,
	max: i32,
	letter: String,
}

impl Policy {
	fn new(p0: String, p1: String) -> Result<Policy, Box<dyn Error>> {
		let mm = p0.split('-').collect::<Vec<&str>>();

		Ok(Policy{
			min: mm[0].parse::<i32>()?,
			max: mm[1].parse::<i32>()?,
			letter: p1.split(':').collect::<Vec<&str>>()[0].to_string(),
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

fn valid_passwords_count(v: Vec<Entry>) -> i32 {
	let mut a = 0;

	for e in v {
		let c = e.1.split(&e.0.letter).count() as i32 - 1;

		if c >= e.0.min && c <= e.0.max {
			a += 1
		}
	}

	a
}

fn main() {
    let entries = get_passwords().and_then(|s: String| parse_passwords(s)).unwrap();
    println!("part 1: {}", valid_passwords_count(entries))
}
