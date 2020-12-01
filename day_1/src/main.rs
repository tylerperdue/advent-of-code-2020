use std::fs;

use std::error::Error;
use std::collections::HashMap;

fn get_expense_report_from_archives() -> Result<String, Box<dyn Error>> {
	let s = fs::read_to_string("src/expense_report.txt")?;
	Ok(s)
}

fn read_report(s: String) -> Result<HashMap<i32, bool>, Box<dyn Error>> {
	let mut entries = std::collections::HashMap::new();

	for line in s.split('\n') {
		let entry = line.parse::<i32>()?;
		entries.insert(entry, true);
	};

	Ok(entries)
}

fn find_mistake(r: HashMap<i32, bool>) -> Option<(i32, i32)> {
	for (k, _v) in &r {
		let p = 2020 - k;

		if r.contains_key(&p) {
			return Some((*k, p));
		}
	};

	None
}


fn main() {
	let report = get_expense_report_from_archives().and_then(|s: String| read_report(s)).unwrap();
	let mistake = find_mistake(report);

	if mistake.is_none() {
		panic!("no mistakes? that can't be right...")
	}

	let answer = mistake.unwrap().0 * mistake.unwrap().1;
	println!("answer: {}", answer)
}
