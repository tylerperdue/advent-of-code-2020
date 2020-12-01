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
	}

	Ok(entries)
}

fn find_pair(r: &HashMap<i32, bool>) -> Option<(i32, i32)> {
	for (k, _v) in r {
		let p = 2020 - k;

		if r.contains_key(&p) {
			return Some((*k, p));
		}
	}

	None
}

fn find_triple(r: &HashMap<i32, bool>) -> Option<(i32, i32, i32)> {
	for (ki, _vi) in r {
		let p = 2020 - ki;

		for (kj, _vj) in r {
			let m = p - kj;

			if r.contains_key(&m) {
				return Some((*kj, *ki, m))
			}
		}
	}

	None
}


fn main() {
	let report = get_expense_report_from_archives().and_then(|s: String| read_report(s)).unwrap();

	let pair = find_pair(&report);
	let triple = find_triple(&report);

	if pair.is_none() || triple.is_none()  {
		panic!("no matches? that can't be right...")
	}

	let part_1 = pair.unwrap();
	let part_2 = triple.unwrap();

	println!("part 1: {} part 2: {}", (part_1.0 * part_1.1), (part_2.0 * part_2.1 *part_2.2))
}
