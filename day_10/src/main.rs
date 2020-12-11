use std::collections::HashMap;

fn main() {
    let mut jolts: Vec<i64> = include_str!("jolts.txt")
    	.lines()
    	.map(|l| l.parse::<i64>().unwrap())
    	.collect();

    jolts.sort();

    jolts.insert(0, 0);

    let pairs: Vec<(i64, i64)> = jolts.iter().enumerate()
    	.map(|(i, n)| if i != jolts.len()-1 {(*n, jolts[i+1])} else {(*n, *n + 3)})
    	.collect();

    let ones = pairs.iter()
    	.filter(|t| t.1 - t.0 == 1)
    	.count();

    let threes = pairs.iter()
    	.filter(|t| t.1 - t.0 == 3)
    	.count();

    let mut cache: HashMap<i64, i64> = HashMap::new();

    println!("part 1: {} part 2: {}", ones * threes, arrangements(&mut cache, &jolts));
}

fn arrangements(c: &mut HashMap<i64, i64>, v: &Vec<i64>) -> i64 {
	if v.len() == 1 {
		return 1
	}

	if c.contains_key(&v[0]) {
		return *c.get(&v[0]).unwrap()
	}

	let a = v.iter().enumerate()
		.filter(|&(i, n)| i != 0 as usize && can_connect((v[0], *n)))
		.map(|(i, _n)| arrangements(c, &v[i..].to_vec()))
		.fold(0, |acc, x| acc + x);

	c.insert(v[0], a);

	a
}

fn can_connect(t: (i64, i64)) -> bool {
	match t.1 - t.0 {
		1 | 2 | 3 => true,
		_ => false
	}
}
