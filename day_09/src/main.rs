use std::collections::HashMap;

static PREAMBLE_SIZE: usize = 25;

fn main() {
    let nums: Vec<i64> = include_str!("xmas-cipher.txt")
    	.lines()
    	.map(|l| l.parse::<i64>().unwrap())
    	.map(|n| n)
    	.collect();

    let hashed_preambles: Vec<HashMap<i64, bool>> = nums.iter()
    	.enumerate()
    	.filter(|&(i, _)| i >= PREAMBLE_SIZE)
    	.map(|(i, _)| (get_preamble(&nums, i)))
    	.map(|p| (hash(&p)))
    	.collect();

    let cache = part_1(&nums, &hashed_preambles);

	println!("part 1: {} part 2: {}", cache, part_2(&cache, &nums))
}

fn get_preamble(v: &Vec<i64>, i: usize) -> Vec<i64> {
	v[i-PREAMBLE_SIZE..i].to_vec()
}

fn hash(v: &Vec<i64>) -> HashMap<i64, bool> {
	let mut m: HashMap<i64, bool> = HashMap::new();

	for i in v {
		m.insert(*i, true);
	}

	m
}

fn part_1(nums: &Vec<i64>, v: &Vec<HashMap<i64, bool>>) -> i64 {
	for i in PREAMBLE_SIZE..nums.len() {
		if !contains_a_sum(&v[i-PREAMBLE_SIZE], &nums[i]) {
			return nums[i]
		}
	}

	0
} 

fn contains_a_sum(m: &HashMap<i64, bool>, n: &i64) -> bool {
	for k in m.keys() {
		let d = n - k;

		if m.contains_key(&d) {
			return true
		}
	}

	false
}

fn part_2(c: &i64, nums: &Vec<i64>) -> i64 {
	for i in 0..nums.len() {
		let mut v: Vec<i64> = Vec::new();
		let mut s = 0;

		for j in i+1..nums.len() {
			if s == *c {
				v.sort();
				return v[0] + v[v.len() - 1]
			}

			s += nums[j];
			v.push(nums[j])
		}
	}

	0
}
