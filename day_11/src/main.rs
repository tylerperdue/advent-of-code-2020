fn main() {
    let seats: Vec<Vec<char>> = include_str!("seats.txt")
    	.lines()
    	.map(|l| l.chars().collect())
    	.collect();

    println!("part 1: {} part 2: {}", part_1(&seats), part_2(&seats))
}

fn part_1(seats: &Vec<Vec<char>>) -> i32 {
	let mut new = seats.clone();

	for (i, row) in seats.iter().enumerate() {
		for (j, seat) in row.iter().enumerate() {
			if *seat == 'L' && adjacent_count(&seats, i, j, &'#') == 0 {
				new[i][j] = '#'
			}

			if *seat == '#' && adjacent_count(&seats, i, j, &'#') >= 4 {
				new[i][j] = 'L'
			}
		}
	}

	if !equals(&seats, &new) {
		return part_1(&new)
	}

	occupied(&new)
}

fn adjacent_count(seats: &Vec<Vec<char>>, row: usize, col: usize, ch: &char) -> i32 {
	deltas(seats, row, col).iter()
		.map(|x| seats[(row as i32+x.0) as usize][(col as i32+x.1) as usize])
		.filter(|seat| *seat == *ch).count() as i32
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

fn part_2(seats: &Vec<Vec<char>>) -> i32 {
	occupied(seats)
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
