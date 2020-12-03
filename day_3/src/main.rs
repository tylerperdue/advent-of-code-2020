use std::fs;

use std::error::Error;

fn get_slopes() -> Result<String, Box<dyn Error>> {
	let s = fs::read_to_string("src/slopes.txt")?;
	Ok(s)
}

fn parse_slopes(s: String) -> Vec<Vec<char>> {
	let mut slopes: Vec<Vec<char>> = Vec::new();

	let lines: Vec<&str> = s.split('\n').collect();
	for l in lines {
		slopes.push(l.chars().collect::<Vec<char>>());
	}

	slopes
}

fn get_tree_count(s: Vec<Vec<char>>) -> i32 {
	let mut ans = 0;

	let mut r = 1;
	let mut c = 3;

	while r < s.len() {
		if c >= s[r].len() {
			c = c - s[r].len()
		}

		if s[r][c] == '#' {
			ans += 1
		}

		r += 1;
		c += 3
	}

	ans
}

fn main() {
	let slopes = get_slopes().and_then(|s: String| Ok(parse_slopes(s))).unwrap();
	println!("{}", get_tree_count(slopes));
}
