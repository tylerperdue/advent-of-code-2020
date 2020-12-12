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
	count(&adjacent_seats(seats, row , col), ch)
}

fn count(seats: &Vec<char>, ch: &char) -> i32 {
	seats.iter().filter(|seat| *seat == ch).count() as i32
}

fn adjacent_seats(seats: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<char> {
	if row == 0 && col == 0 {
		vec![seats[row][col+1], seats[row+1][col+1], seats[row+1][col]]
	} else if row == seats.len()-1 && col == seats[0].len()-1 {
		vec![seats[row][col-1], seats[row-1][col-1], seats[row-1][col]]
	} else if row == 0 && col != seats[0].len()-1 {
		vec![seats[row][col+1], seats[row+1][col+1], seats[row+1][col], seats[row+1][col-1], seats[row][col-1]]
	} else if row == 0 && col == seats[0].len()-1 {
		vec![seats[row][col-1], seats[row+1][col-1], seats[row+1][col]]
	} else if col == 0 && row != seats.len()-1 {
		vec![seats[row][col+1], seats[row+1][col+1], seats[row+1][col], seats[row-1][col], seats[row-1][col+1]]
	} else if col == 0 && row == seats.len()-1 {
		vec![seats[row][col+1], seats[row-1][col+1], seats[row-1][col]]
	} else if row == seats.len()-1 {
		vec![seats[row-1][col], seats[row-1][col+1], seats[row][col+1], seats[row][col-1], seats[row-1][col-1]]
	} else if col == seats[0].len()-1 {
		vec![seats[row-1][col], seats[row+1][col], seats[row+1][col-1], seats[row][col-1], seats[row-1][col-1]]
	} else {
		vec![seats[row][col+1], seats[row+1][col+1],seats[row+1][col], seats[row+1][col-1], seats[row][col-1], seats[row-1][col-1], seats[row-1][col], seats[row-1][col+1]]
	}
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

fn seeable_seats_count(seats: &Vec<Vec<char>>, row: usize, col: usize, ch: &char) -> i32 {
	if row == 0 && col == 0 {
		vec!
		[step(seats, row, col, 0, 1), step(seats, row, col, 1, 1), step(seats, row, col, )]
	// } else if row == seats.len()-1 && col == seats[0].len()-1 {
		
	// } else if row == 0 && col != seats[0].len()-1 {
		
	// } else if row == 0 && col == seats[0].len()-1 {
		
	// } else if col == 0 && row != seats.len()-1 {
		
	// } else if col == 0 && row == seats.len()-1 {
	// 	vec![seats[row][col+1], seats[row-1][col+1], seats[row-1][col]]
	// } else if row == seats.len()-1 {
	// 	vec![seats[row-1][col], seats[row-1][col+1], seats[row][col+1], seats[row][col-1], seats[row-1][col-1]]
	// } else if col == seats[0].len()-1 {
	// 	vec![seats[row-1][col], seats[row+1][col], seats[row+1][col-1], seats[row][col-1], seats[row-1][col-1]]
	// } else {
	} else {
		vec![seats[row][col+1], seats[row+1][col+1],seats[row+1][col], seats[row+1][col-1], seats[row][col-1], seats[row-1][col-1], seats[row-1][col], seats[row-1][col+1]]
	}
}

fn deltas(seats: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(usize, usize) {
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
		vec![(-1, 0), (-1. 1), (0, 1), (0, -1), (-1, -1)]
	} else if col == seats[0].len()-1 {
		vec![(-1, 0), (1, 0), (1, -1), (0, -1), (-1, -1)]
	} else {
		vec![seats[row][col+1], seats[row+1][col+1],seats[row+1][col], seats[row+1][col-1], seats[row][col-1], seats[row-1][col-1], seats[row-1][col], seats[row-1][col+1]]
	}
}

fn step(seats: &Vec<char>, row: usize, col: usize, x: usize, y: usize, ch: &char) -> bool {
	match seats.iter().filter(|seat| *seat == ch).count() {
		0 => false,
		_ => true
	}
}
