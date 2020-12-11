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

fn get_tree_count(s: &Vec<Vec<char>>, xy: (usize, usize)) -> i64 {
	let mut ans = 0;

	let mut r = xy.1;
	let mut c = xy.0;

	while r < s.len() {
		if c >= s[r].len() {
			c = c - s[r].len()
		}

		if s[r][c] == '#' {
			ans += 1
		}

		r += xy.1;
		c += xy.0
	}

	ans
}

fn main() {
	let slopes = get_slopes().and_then(|s: String| Ok(parse_slopes(s))).unwrap();

	let part_1 = get_tree_count(&slopes, (3, 1));
	let part_2 = vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)).into_iter().map(|xy| get_tree_count(&slopes, xy)).fold(1, |total, count| total * count);

	println!("part 1: {} part 2: {}", part_1, part_2)
}
