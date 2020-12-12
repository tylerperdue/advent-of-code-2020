fn main() {
    let seats: Vec<Vec<char>> = include_str!("seats.txt")
    	.lines()
    	.map(|l| l.chars().collect())
    	.collect();

    println!("part 1: {} part 2: {}", round(&seats, &adjacent_count, 4), round(&seats, &seeable_count, 5))
}

fn round(seats: &Vec<Vec<char>>, count: &dyn Fn(&Vec<Vec<char>>, usize, usize, &char) -> i32, occ_threshold: i32) -> i32 {
	let mut new = seats.clone();

	for (i, row) in seats.iter().enumerate() {
		for (j, seat) in row.iter().enumerate() {
			if *seat == 'L' && count(&seats, i, j, &'#') == 0 {
				new[i][j] = '#'
			}

			if *seat == '#' && count(&seats, i, j, &'#') >= occ_threshold {
				new[i][j] = 'L'
			}
		}
	}

	if !equals(&seats, &new) {
		return round(&new, count, occ_threshold)
	}

	occupied(&new)
}

fn adjacent_count(seats: &Vec<Vec<char>>, row: usize, col: usize, ch: &char) -> i32 {
	deltas(seats, row, col).iter()
		.map(|x| seats[(row as i32+x.0) as usize][(col as i32+x.1) as usize])
		.filter(|seat| *seat == *ch).count() as i32
}

fn seeable_count(seats: &Vec<Vec<char>>, row: usize, col: usize, ch: &char) -> i32 {
	deltas(seats, row, col).iter().filter(|&x| seeable(seats, row, col, ch, *x)).count() as i32
}

fn seeable(seats: &Vec<Vec<char>>, row: usize, col: usize, ch: &char, d: (i32, i32)) -> bool {
	let mut depth = 2;
	let mut next = seats[(row as i32+d.0) as usize][(col as i32+d.1) as usize];

	while next == '.' && valid_depth(seats, row, col, d, depth) {
		next = seats[(row as i32+d.0*depth) as usize][(col as i32+d.1*depth) as usize];
		depth += 1
	}

	if next == *ch {
		return true
	}

	return false
}

fn valid_depth(seats: &Vec<Vec<char>>, row: usize, col: usize, d: (i32, i32), depth: i32) -> bool {
	let r = row as i32 + d.0 * depth;
	let c = col as i32 + d.1 * depth;

	r >= 0 && r != seats.len() as i32 && c >= 0 && c != seats[0].len() as i32
}

fn equals(one: &Vec<Vec<char>>, two: &Vec<Vec<char>>) -> bool {
	for i in 0..one.len() {
		for j in 0..one[i].len() {
			if one[i][j] != two[i][j] {
				return false
			}
		}
	}

	true
}

fn occupied(seats: &Vec<Vec<char>>) -> i32 {
	seats.iter().map(|row| row.iter().filter(|seat| **seat == '#').count() as i32)
		.fold(0, |acc, x| acc + x)
}

fn deltas(seats: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(i32, i32)> {
	if row == 0 && col == 0 {
		vec![(0, 1), (1, 1), (1, 0)]
	} else if row == seats.len()-1 && col == seats[0].len()-1 {
		vec![(0, -1), (-1, -1), (-1, 0)]
	} else if row == 0 && col != seats[0].len()-1 {
		vec![(0, 1), (1, 1), (1, 0), (1, -1), (0, -1)]
	} else if row == 0 && col == seats[0].len()-1 {
		vec![(0, -1), (1, -1), (1, 0)]
	} else if col == 0 && row != seats.len()-1 {
		vec![(0, 1), (1, 1), (1, 0), (-1, 0), (-1, 1)]
	} else if col == 0 && row == seats.len()-1 {
		vec![(0, 1), (-1, 1), (-1, 0)]
	} else if row == seats.len()-1 {
		vec![(-1, 0), (-1, 1), (0, 1), (0, -1), (-1, -1)]
	} else if col == seats[0].len()-1 {
		vec![(-1, 0), (1, 0), (1, -1), (0, -1), (-1, -1)]
	} else {
		vec![(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)]
	}
}
