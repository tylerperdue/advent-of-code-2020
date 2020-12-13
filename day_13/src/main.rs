fn main() {
    let mut input = include_str!("schedule.txt").lines();

    let time = input.next().unwrap().parse::<i64>().unwrap();

    let bus_ids: Vec<i64> = input.next().unwrap()
    	.split(",")
    	.map(|x| x.parse::<i64>().unwrap_or(-1))
    	.collect();

    println!("part 1: {}", part_1(time, &bus_ids));
    println!("part 2: {}", part_2(&bus_ids))
}

fn part_1(time: i64, buses: &Vec<i64>) -> i64 {
	let winner = buses.iter().filter(|&b| *b != -1)
		.map(|b| ((time as f64 / *b as f64).ceil() as i64 * b, b))
		.min().unwrap();

	(winner.0 - time) * winner.1
}

// had to ellicit some help for this one. my first attempt would have taken days... weeks???
fn part_2(buses: &Vec<i64>) -> i64 {
	let mut t = 0;
	let mut lcm = 1;

	for (i, bus) in buses.iter().enumerate().filter(|&(_, b)| *b != -1) {
		while (t + i as i64) % bus != 0 {
			t += lcm
		}

		lcm *= *bus as i64
	}

	return t
}
	