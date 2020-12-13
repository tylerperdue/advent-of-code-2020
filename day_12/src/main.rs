fn main() {
    let input: Vec<&str> = include_str!("instructions.txt").lines().collect();
    let instructions = parsed(&input);

    let part_1 = navigate(&mut Normal::new(), &instructions);
    let part_2 = navigate(&mut ByWaypoint::new(), &instructions);
  
    println!("part 1: {} part 2: {}", part_1, part_2)
}

fn parsed(input: &Vec<&str>) -> Vec<(char, i128)> {
	let mut output: Vec<(char, i128)> = Vec::new();

	for line in input  {
		let chars: Vec<char> = line.chars().collect();

		output.push((chars[0], chars[1..].iter().collect::<String>().parse::<i128>().unwrap()))
	}

	output
}

trait GPS {
	fn teleport(&mut self, direction: char, spaces: i128);
	fn turn(&mut self, direction: char, spaces: i128);
	fn forward(&mut self, spaces: i128);
	fn location(&self, location_services: fn(i128, i128) -> i128) -> i128;
}

fn navigate(gps: &mut impl GPS, instructions: &Vec<(char, i128)>) -> i128 {
	for instruction in instructions {
		match instruction.0 {
			'N' | 'S' | 'E' | 'W' => gps.teleport(instruction.0, instruction.1),
			'L' | 'R' => gps.turn(instruction.0, instruction.1),
			'F' => gps.forward(instruction.1),
			_ => ()
		}
	}

	gps.location(manhattan_distance)
}

#[derive(Debug)]
struct Normal {
	x: i128,
	y: i128,

	facing: char,
}

impl Normal {
	fn new() -> Normal {
		Normal{x: 0, y: 0, facing: 'E'}
	}

	fn turn_90(&mut self, direction: char) {
		match direction {
			'L' => match self.facing {
				'N' => self.facing = 'W',
				'S' => self.facing = 'E',
				'E' => self.facing = 'N',
				'W' => self.facing = 'S',
				_ => ()
			},
			'R' => match self.facing {
				'N' => self.facing = 'E',
				'S' => self.facing = 'W',
				'E' => self.facing = 'S',
				'W' => self.facing = 'N',
				_ => ()
			}
			_ => ()
		}
	}
}

impl GPS for Normal {
	fn teleport(&mut self, direction: char, spaces: i128) {
		match direction {
			'N' => self.y += spaces,
			'S' => self.y -= spaces,
			'E' => self.x += spaces,
			'W' => self.x -= spaces,
			_ => ()
		}
	}

	fn turn(&mut self, direction: char, degrees: i128) {
		for _i in 0..degrees/90 {
			self.turn_90(direction)
		}
	}

	fn forward(&mut self, spaces: i128) {
		match self.facing {
			'N' => self.y += spaces,
			'S' => self.y -= spaces,
			'E' => self.x += spaces,
			'W' => self.x -= spaces,
			_ => ()
		}
	}

	fn location(&self, location_services: fn(i128, i128) -> i128) -> i128 {
		location_services(self.x, self.y)
	}
}

#[derive(Debug)]
struct ByWaypoint {
	x: i128,
	y: i128,

	way_point: (i128, i128)
}

impl ByWaypoint {
	fn new() -> ByWaypoint {
		ByWaypoint{x: 0, y: 0, way_point: (10, 1)}
	}
}

impl GPS for ByWaypoint {
	fn teleport(&mut self, direction: char, spaces: i128) {
		match direction {
			'N' => self.way_point.1 += spaces,
			'S' => self.way_point.1 -= spaces,
			'E' => self.way_point.0 += spaces,
			'W' => self.way_point.0 -= spaces,
			_ => ()
		}
	}

	fn turn(&mut self, direction: char, degrees: i128) {
		let conversion = match direction {
			'R' => match degrees {
				90 => 270,
				180 => 180,
				270 => 90,
				_ => 0
			}
			'L' => degrees,
			_ => 0
		};

		let x = self.way_point.0 - self.x;
		let y = self.way_point.1 - self.y;

		match conversion {
			90 => {
				self.way_point.0 = y * -1;
				self.way_point.1 = x
			},
			180 => {
				self.way_point.0 = x * -1;
				self.way_point.1 = y * -1
			}
			270 => {
				self.way_point.0 = y;
				self.way_point.1 = x * -1
			},
			_ => ()
		}

		self.way_point.0 += self.x;
		self.way_point.1 += self.y
	}

	fn forward(&mut self, spaces: i128) {
		let x = self.way_point.0 - self.x;
		let y = self.way_point.1 - self.y;

		for _i in 0..spaces {
			self.x = self.way_point.0;
			self.y = self.way_point.1;

			self.way_point.0 += x;
			self.way_point.1 += y;
		}
	}

	fn location(&self, location_services: fn(i128, i128) -> i128) -> i128 {
		location_services(self.x, self.y)
	}
}

fn manhattan_distance(x: i128, y: i128) -> i128 {
	(0 + x).abs() + (0 + y).abs()
}
